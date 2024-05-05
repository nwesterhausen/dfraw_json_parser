use serde::{Deserialize, Serialize};
use tracing::warn;



#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, specta::Type)]
#[serde(rename_all = "camelCase")]
/// Gaits are a way to describe how a creature moves. Defined in the raws with:
///
/// "GAIT:type:name:full speed:build up time:turning max:start speed:energy use"
///
/// * use `NO_BUILD_UP` if you jump immediately to full speed
///
/// these optional flags go at the end:
///
/// * `LAYERS_SLOW` - fat/muscle layers slow the movement (muscle-slowing counter-acted by strength bonus)
/// * `STRENGTH` - strength attribute can speed/slow movement
/// * `AGILITY` - agility attribute can speed/slow movement
/// * `STEALTH_SLOWS:<n>` - n is percentage slowed
/// * it would be interesting to allow quirky attributes (like mental stats), but they aren't supported yet
///
/// Examples:
///
///    `[CV_NEW_TAG:GAIT:WALK:Sprint:!ARG4:10:3:!ARG2:50:LAYERS_SLOW:STRENGTH:AGILITY:STEALTH_SLOWS:50]`
///    `[CV_NEW_TAG:GAIT:WALK:Run:!ARG3:5:3:!ARG2:10:LAYERS_SLOW:STRENGTH:AGILITY:STEALTH_SLOWS:20]`
///    `[CV_NEW_TAG:GAIT:WALK:Jog:!ARG2:NO_BUILD_UP:5:LAYERS_SLOW:STRENGTH:AGILITY:STEALTH_SLOWS:10]`
///    `[CV_NEW_TAG:GAIT:WALK:Walk:!ARG1:NO_BUILD_UP:0]`
///    `[CV_NEW_TAG:GAIT:WALK:Stroll:!ARG5:NO_BUILD_UP:0]`
///    `[CV_NEW_TAG:GAIT:WALK:Creep:!ARG6:NO_BUILD_UP:0]`
pub struct Gait {
    /// The type of gait
    gait_type: GaitType,
    /// The name of the gait
    name: String,
    /// The maximum speed achievable by a creature using this gait.
    max_speed: u32,
    /// The energy use of the gait
    energy_use: u32,
    /// The gait modifiers
    ///
    /// These are optional, and may be empty.
    modifiers: Vec<Modifier>,
}



#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, specta::Type)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub enum GaitType {
    /// Travel on foot/the ground
    /// Used for moving normally over ground tiles.
    Walk,
    /// Travel on foot/the ground
    /// Used for moving over ground tiles whilst prone.
    Crawl,
    /// Climbing on walls, etc.
    /// Used for moving whilst climbing.
    Climb,
    /// Swimming in water/liquid
    /// Used for moving through tiles containing water or magma at a depth of at least 4/7.
    Swim,
    /// Flying through the air
    /// Used for moving through open space.
    Fly,
    /// Other gait type which is unexpected, but we can still handle it
    Other(String),
    #[default]
    /// Unknown gait type (unset)
    Unknown,
}



#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum Modifier {
    /// Fat/muscle layers slow the movement (muscle-slowing counter-acted by strength bonus)
    /// Makes `THICKENS_ON_ENERGY_STORAGE` and `THICKENS_ON_STRENGTH` tissue layers slow movement depending on how thick they are.
    /// Adding the `STRENGTH` gait flag counteracts the impact of the latter layer.
    LayersSlow,
    /// Speeds/slows movement depending on the creature's Strength stat.
    Strength,
    /// Speeds/slows movement depending on the creature's Agility stat.
    Agility,
    /// Stealth slows movement by the specified percentage when the creature is sneaking.
    StealthSlows {
        /// The percentage slowed
        percentage: u32,
    },
    /// No build up time
    NoBuildUp,
    /// Build up time. Only used if the gait has a build up time.
    BuildUp {
        /// The build up time indicates how long it will take for a creature using this gait to go from `<start speed>` to `<max speed>`.
        /// For example, a value of 10 means that it should be able to reach the maximum speed by moving 10 tiles in a straight line over even terrain.
        time: u32,
        /// The turning max indicates the maximum speed permissible when the creature suddenly changes its direction of motion.
        /// The creature's speed will be reduced to `<max turning speed>` if traveling at a higher speed than this before turning.
        turning_max: u32,
        /// The creature's speed when it starts moving using this gait
        start_speed: u32,
    },
}

impl Gait {
    /// Parse a gait given the raw string (i.e. the string after the `GAIT:` tag)
    ///
    /// ## Parameters
    ///
    /// * `raw_gait` - The raw string to parse
    ///
    /// ## Returns
    ///
    /// The parsed gait
    pub fn from_value(value: &str) -> Self {
        let mut gait = Gait::default();
        let mut parts = value.split(':');
        let mut has_build_up = false;

        // First part is the gait type
        gait.gait_type = match parts.next() {
            Some("WALK") => GaitType::Walk,
            Some("CLIMB") => GaitType::Climb,
            Some("SWIM") => GaitType::Swim,
            Some("CRAWL") => GaitType::Crawl,
            Some("FLY") => GaitType::Fly,
            Some(other) => GaitType::Other(other.to_string()),
            None => GaitType::Unknown,
        };

        // Next will be gait name
        gait.name = parts.next().unwrap_or("").to_string();

        // Next will be full speed
        gait.max_speed = parts.next().unwrap_or("0").parse().unwrap_or(0);

        // Next is build up time. Now if this is `NO_BUILD_UP`, then we don't have a build up time, and we also
        // don't have a turning max or start speed. Otherwise, we have a build up time, and we *should* have a
        // turning max and start speed.
        if let Some(raw_value) = parts.next() {
            if raw_value == "NO_BUILD_UP" {
                gait.modifiers.push(Modifier::NoBuildUp);
            } else if let Ok(value) = raw_value.parse() {
                gait.modifiers.push(Modifier::BuildUp {
                    time: value,
                    turning_max: 0,
                    start_speed: 0,
                });
                has_build_up = true;
            }
        }

        if has_build_up {
            // Next is turning max
            if let Some(raw_value) = parts.next() {
                if let Ok(value) = raw_value.parse::<u32>() {
                    // Modify the build up modifier to include the turning max
                    if let Some(Modifier::BuildUp {
                        time,
                        turning_max: _,
                        start_speed,
                    }) = gait.modifiers.pop()
                    {
                        gait.modifiers.push(Modifier::BuildUp {
                            time,
                            turning_max: value,
                            start_speed,
                        });
                    }
                }
            }

            // Next is start speed
            if let Some(raw_value) = parts.next() {
                if let Ok(value) = raw_value.parse::<u32>() {
                    // Modify the build up modifier to include the start speed
                    if let Some(Modifier::BuildUp {
                        time,
                        turning_max,
                        start_speed: _,
                    }) = gait.modifiers.pop()
                    {
                        gait.modifiers.push(Modifier::BuildUp {
                            time,
                            turning_max,
                            start_speed: value,
                        });
                    }
                }
            }
        }

        // Next is energy use. This might be the final part, or there might be modifiers after this.
        gait.energy_use = parts.next().unwrap_or("0").parse().unwrap_or(0);

        // Now we have modifiers. These are optional, so we'll just loop until we run out of parts.
        parts.clone().enumerate().for_each(|(idx, s)| match s {
            "LAYERS_SLOW" => gait.modifiers.push(Modifier::LayersSlow),
            "STRENGTH" => gait.modifiers.push(Modifier::Strength),
            "AGILITY" => gait.modifiers.push(Modifier::Agility),
            "STEALTH_SLOWS" => {
                if let Some(raw_value) = parts.nth(idx + 1) {
                    if let Ok(value) = raw_value.parse() {
                        gait.modifiers
                            .push(Modifier::StealthSlows { percentage: value });
                    }
                } else {
                    warn!("STEALTH_SLOWS modifier is missing a value in {value}");
                }
            }
            _ => {}
        });

        gait
    }

    /// Returns true if the gait is empty (i.e. unset/default)
    ///
    /// ## Returns
    ///
    /// True if the gait is empty, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.gait_type == GaitType::Unknown
    }
}
