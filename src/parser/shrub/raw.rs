use serde::{Deserialize, Serialize};

use crate::parser::color::Color;
use crate::parser::seed_material::raw::SeedMaterial;
use crate::parser::serializer_helper;

use super::phf_table::SHRUB_TOKENS;
use super::tokens::{SeasonToken, ShrubToken};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Shrub {
    /// Allows the plant to grow in farm plots during the given season.
    /// If the plant is a surface plant, allows it to grow in the wild during this season; wild surface plants without
    /// this token will disappear at the beginning of the season. Underground plants grow wild in all seasons, regardless
    /// of their season tokens.
    /// Default: empty (plant will not grow in farm plots)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    growing_season: Vec<SeasonToken>,
    /// How long the plant takes to grow to harvest in a farm plot. Unit hundreds of ticks.
    /// There are 1008 GROWDUR units in a season. Defaults to 300.
    #[serde(skip_serializing_if = "serializer_helper::is_default_grow_duration")]
    grow_duration: u32,
    /// Has no known effect. Previously set the value of the harvested plant.
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    value: u32,
    /// The tile used when the plant is harvested whole, or is ready to be picked from a farm plot. May either be a cp437
    /// tile number, or a character between single quotes. See character table. Defaults to 231 (τ).
    #[serde(skip_serializing_if = "serializer_helper::is_default_picked_tile")]
    picked_tile: u8,
    /// The tile used when a plant harvested whole has wilted. Defaults to 169 (⌐).
    #[serde(skip_serializing_if = "serializer_helper::is_default_dead_picked_tile")]
    dead_picked_tile: u8,
    /// The tile used to represent this plant when it is wild, alive, and has no growths. Defaults to 34 (").
    #[serde(skip_serializing_if = "serializer_helper::is_default_shrub_tile")]
    shrub_tile: u8,
    /// The tile used to represent this plant when it is dead in the wild. Defaults to 34 (").
    #[serde(skip_serializing_if = "serializer_helper::is_default_dead_shrub_tile")]
    dead_shrub_tile: u8,
    /// The maximum stack size collected when gathered via herbalism (possibly also from farm plots?). Defaults to 5.
    #[serde(skip_serializing_if = "serializer_helper::is_default_cluster_size")]
    cluster_size: u32,
    /// The color of the plant when it has been picked whole, or when it is ready for harvest in a farm plot. Defaults to 2:0:0 (dark green).
    #[serde(skip_serializing_if = "Color::is_default")]
    picked_color: Color,
    /// The color of the plant when it has been picked whole, but has wilted. Defaults to 0:0:1 (dark gray).
    #[serde(skip_serializing_if = "Color::is_default")]
    dead_picked_color: Color,
    /// The color of the plant when it is alive, wild, and has no growths. Defaults to 2:0:0 (dark green).
    #[serde(skip_serializing_if = "Color::is_default")]
    shrub_color: Color,
    /// The color of the plant when it is dead in the wild. Defaults to 6:0:0 (brown).
    #[serde(skip_serializing_if = "Color::is_default")]
    dead_shrub_color: Color,
    /// The shrub will drown once the water on its tile reaches this level. Defaults to 4.
    #[serde(skip_serializing_if = "serializer_helper::is_default_shrub_drown_level")]
    shrub_drown_level: u8,

    // Todo: fix these with actual values (materials and seed)
    /// Names a drink made from the plant, allowing it to be used in entity resources.
    /// Previously also permitted brewing the plant into alcohol made of this material.
    /// Now, a MATERIAL_REACTION_PRODUCT of type DRINK_MAT should be used on the proper plant material.
    #[serde(skip_serializing_if = "String::is_empty")]
    drink: String,
    /// Permits milling the plant at a quern or millstone into a powder made of this material and allows its use in entity resources.
    /// Said material should have \[POWDER_MISC_PLANT\] to permit proper stockpiling. This token makes the whole plant harvestable regardless
    /// of which material is designated for milling.
    /// For plants with millable growths, use only MATERIAL_REACTION_PRODUCT or ITEM_REACTION_PRODUCT tokens to define the milling products.
    #[serde(skip_serializing_if = "String::is_empty")]
    mill: String,
    /// Permits processing the plant at a farmer's workshop to yield threads made of this material and allows its use in entity resources.
    /// Said material should have \[THREAD_PLANT\] to permit proper stockpiling.
    #[serde(skip_serializing_if = "String::is_empty")]
    thread: String,
    /// Causes the plant to yield plantable seeds made of this material and having these properties.
    /// Said material should have \[SEED_MAT\] to permit proper stockpiling.
    #[serde(skip_serializing_if = "SeedMaterial::is_empty")]
    seed: SeedMaterial,
    /// Permits processing the plant into a vial at a still to yield extract made of this material.
    /// Said material should have \[EXTRACT_STORAGE:FLASK\].
    #[serde(skip_serializing_if = "String::is_empty")]
    extract_still_vial: String,
    /// Permits processing the plant into a vial at a farmer's workshop to yield extract made of this material.
    /// Said material should have \[EXTRACT_STORAGE:VIAL\].
    #[serde(skip_serializing_if = "String::is_empty")]
    extract_vial: String,
    /// Permits processing the plant into a barrel at a farmer's workshop to yield extract made of this material.
    /// Said material should have \[EXTRACT_STORAGE:BARREL\].
    #[serde(skip_serializing_if = "String::is_empty")]
    extract_barrel: String,
}

impl Shrub {
    pub fn new() -> Shrub {
        Shrub {
            grow_duration: 300,
            picked_tile: 231,
            dead_picked_tile: 169,
            shrub_tile: 34,
            dead_shrub_tile: 34,
            cluster_size: 5,
            shrub_drown_level: 4,
            ..Shrub::default()
        }
    }
    #[allow(clippy::too_many_lines)]
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(tag) = SHRUB_TOKENS.get(key) else {
            log::warn!("Unknown shrub token: {}", key);
            return;
        };

        match tag {
            ShrubToken::Spring => {
                self.growing_season.push(SeasonToken::Spring);
            }
            ShrubToken::Summer => {
                self.growing_season.push(SeasonToken::Summer);
            }
            ShrubToken::Autumn => {
                self.growing_season.push(SeasonToken::Autumn);
            }
            ShrubToken::Winter => {
                self.growing_season.push(SeasonToken::Winter);
            }
            ShrubToken::GrowDuration => {
                self.grow_duration = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("grow_duration parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            ShrubToken::Value => {
                self.value = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("value parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            ShrubToken::PickedTile => {
                self.picked_tile = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("picked_tile parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            ShrubToken::DeadPickedTile => {
                self.dead_picked_tile = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("dead_picked_tile parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            ShrubToken::ShrubTile => {
                self.shrub_tile = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("shrub_tile parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            ShrubToken::DeadShrubTile => {
                self.dead_shrub_tile = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("dead_shrub_tile parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            ShrubToken::ClusterSize => {
                self.cluster_size = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("cluster_size parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            ShrubToken::PickedColor => {
                self.picked_color = Color::from_value(value);
            }
            ShrubToken::DeadPickedColor => {
                self.dead_picked_color = Color::from_value(value);
            }
            ShrubToken::ShrubColor => {
                self.shrub_color = Color::from_value(value);
            }
            ShrubToken::DeadShrubColor => {
                self.dead_shrub_color = Color::from_value(value);
            }
            ShrubToken::ShrubDrownLevel => {
                self.shrub_drown_level = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("shrub_drown_level parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            ShrubToken::Drink => {
                self.drink = String::from(value);
            }
            ShrubToken::Mill => {
                self.mill = String::from(value);
            }
            ShrubToken::Thread => {
                self.thread = String::from(value);
            }
            ShrubToken::Seed => {
                self.seed = SeedMaterial::from_value(value);
            }
            ShrubToken::ExtractStillVial => {
                self.extract_still_vial = String::from(value);
            }
            ShrubToken::ExtractVial => {
                self.extract_vial = String::from(value);
            }
            ShrubToken::ExtractBarrel => {
                self.extract_barrel = String::from(value);
            }
            ShrubToken::Unknown => {
                log::warn!("Unknown shrub token: {}", key);
            }
        }
    }
}
