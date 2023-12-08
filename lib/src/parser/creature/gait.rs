use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
/// Gaits are a way to describe how a creature moves. Defined in the raws with:
///       GAIT:type:name:full speed:build up time:turning max:start speed:energy use
///                              ||
///                              vv
///          use `NO_BUILD_UP` if you jump immediately to full speed
///
///          these optional flags go at the end:
///                `LAYERS_SLOW` - fat/muscle layers slow the movement (muscle-slowing counter-acted by strength bonus)
///                `STRENGTH` - strength attribute can speed/slow movement
///                `AGILITY` - agility attribute can speed/slow movement
///                `STEALTH_SLOWS:<n>` - n is percentage slowed
///                it would be interesting to allow quirky attributes (like mental stats), but they aren't supported yet
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
    /// The type of gait, e.g. WALK, CLIMB, SWIM, CRAWL, FLY
    gait_type: GaitType,
    /// The name of the gait, e.g. Walk, Run, Jog, Stroll, Creep
    name: String,
    /// The full speed of the gait
    full_speed: u32,
    /// The energy use of the gait
    energy_use: u32,
    /// The gait modifiers
    ///
    /// These are optional, and may be empty.
    modifiers: Vec<Modifier>,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub enum GaitType {
    /// Travel on foot/the ground
    Walk,
    /// Climbing on walls, etc.
    Climb,
    /// Swimming in water
    Swim,
    /// Crawling on the ground
    Crawl,
    /// Flying through the air
    Fly,
    /// Other gait type which is unexpected, but we can still handle it
    Other(String),
    #[default]
    /// Unknown gait type (unset)
    Unknown,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Modifier {
    /// Fat/muscle layers slow the movement (muscle-slowing counter-acted by strength bonus)
    LayersSlow,
    /// Strength attribute can speed/slow movement
    Strength,
    /// Agility attribute can speed/slow movement
    Agility,
    /// Stealth slows the movement
    StealthSlows(u32),
    /// No build up time
    NoBuildUp,
    /// Build up time. Only used if the gait has a build up time.
    BuildUp {
        /// The build up time
        time: u32,
        /// The turning max
        turning_max: u32,
        /// The start speed
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
        gait.full_speed = parts.next().unwrap_or("0").parse().unwrap_or(0);

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
                        gait.modifiers.push(Modifier::StealthSlows(value));
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
