use serde::{Deserialize, Serialize};

use crate::parser::names::SingPlurName;
use crate::parser::ranges::Ranges;

use super::{
    phf_table::{GROWTH_TOKENS, PLANT_PART_TOKENS},
    tokens::{GrowthTag, GrowthType, PlantPart},
};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlantGrowth {
    /// Plant growths are not given an identifier, since they are just supporting
    /// data for the plant definition. They are defined instead by the type of growth.
    growth_type: GrowthType,
    /// The name of the growth. This is actually defined with "GROWTH_NAME" key in the raws.
    name: SingPlurName,
    /// The item grown by this growth. This is actually defined with "GROWTH_ITEM" key in the raws.
    /// This is a string until we make a proper item structure. Technically there are 2 arguments:
    /// 1. item token, 2: material token. Generally the item type should be PLANT_GROWTH:NONE.
    item: String,
    /// Specifies on which part of the plant this growth grows. This is defined with "GROWTH_HOST_TILE" key.
    /// This can be unused, like in the case of crops where the plant is the growth (I think?).    
    #[serde(skip_serializing_if = "Vec::is_empty")]
    host_tiles: Vec<PlantPart>,
    /// Controls the height on the trunk above which the growth begins to appear.
    /// The first value is the percent of the trunk height where the growth begins appearing:
    /// 0 will cause it along the entire trunk (above the first tile), 100 will cause it to appear
    /// at the topmost trunk tile. Can be larger than 100 to cause it to appear above the trunk.
    /// The second value must be -1, but might be intended to control whether it starts height counting
    /// from the bottom or top.
    #[serde(skip_serializing_if = "Ranges::is_default_trunk_height_percentage")]
    trunk_height_percentage: [i32; 2],
    /// Currently has no effect.
    #[serde(skip_serializing_if = "Ranges::is_default_growth_density")]
    density: u32,
    /// Specifies the appearance of the growth. This is defined with "GROWTH_PRINT" key.
    /// This is a string until we make a proper print structure.
    #[serde(skip_serializing_if = "String::is_empty")]
    print: String,
    /// Specifies at which part of the year the growth appears. Default is all year round.
    /// Minimum: 0, Maximum: 402_200. This is defined with "GROWTH_TIMING" key.
    #[serde(skip_serializing_if = "Ranges::is_default_growth_timing")]
    timing: [u32; 2],
    /// Where we gather some of the growth's tags.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<GrowthTag>,
}

impl PlantGrowth {
    pub fn new(growth_type: GrowthType) -> PlantGrowth {
        PlantGrowth {
            growth_type,
            ..PlantGrowth::default()
        }
    }
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(tag) = GROWTH_TOKENS.get(key) else {
            log::warn!(
                "PlantGrowthParsing: called `Option::unwrap()` on a `None` value for presumed caste tag: {}",
                key
            );
            return;
        };

        if value.is_empty() {
            // If there is no value, we just add the tag to the list.
            self.tags.push(tag.clone());
            return;
        }

        match tag {
            GrowthTag::GrowthName => {
                self.name = SingPlurName::from_value(value);
            }
            GrowthTag::GrowthItem => {
                self.item = value.to_string();
            }
            GrowthTag::GrowthHostTile => {
                let Some(part) = PLANT_PART_TOKENS.get(value) else {
                    log::warn!(
                        "PlantGrowthParsing: called `Option::unwrap()` on a `None` value for presumed plant part: {}",
                        value
                    );
                    return;
                };
                self.host_tiles.push(part.clone());
            }
            GrowthTag::GrowthTrunkHeightPercent => {
                let split: Vec<&str> = value.split(':').collect::<Vec<&str>>();
                if split.len() != 2 {
                    log::warn!(
                        "PlantGrowthParsing: called `Option::unwrap()` on a `None` value for presumed plant part: {}",
                        value
                    );
                    return;
                }
                let percentage: i32 = match split.first().unwrap_or(&"").parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("min_value parsing error\n{:?}", e);
                        return;
                    }
                };
                let dir: i32 = match split.get(1).unwrap_or(&"").parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("max_value parsing error\n{:?}", e);
                        return;
                    }
                };
                self.trunk_height_percentage = [percentage, dir];
            }
            GrowthTag::GrowthDensity => {
                self.density = value.parse().unwrap_or_default();
            }
            GrowthTag::GrowthTiming => {
                let split: Vec<&str> = value.split(':').collect::<Vec<&str>>();
                if split.len() != 2 {
                    log::warn!(
                        "PlantGrowthParsing: called `Option::unwrap()` on a `None` value for presumed plant part: {}",
                        value
                    );
                    return;
                }
                let start: u32 = match split.first().unwrap_or(&"").parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("min_value parsing error\n{:?}", e);
                        return;
                    }
                };
                let end: u32 = match split.get(1).unwrap_or(&"").parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("max_value parsing error\n{:?}", e);
                        return;
                    }
                };
                self.timing = [start, end];
            }
            GrowthTag::GrowthPrint => {
                self.print = value.to_string();
            }
            _ => {
                // If we don't recognize the tag, we just add it to the list.
                self.tags.push(tag.clone());
            }
        }
    }
}
