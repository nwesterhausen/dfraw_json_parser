use serde::{Deserialize, Serialize};
use tracing::{error, warn};

use crate::parser::{clean_search_vec, serializer_helper, Searchable, SingPlurName};

use super::{
    phf_table::{GROWTH_TOKENS, PLANT_PART_TOKENS},
    tokens::{GrowthTag, GrowthType, PlantPart},
};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default, specta::Type)]
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
    host_tiles: Option<Vec<PlantPart>>,
    /// Controls the height on the trunk above which the growth begins to appear.
    /// The first value is the percent of the trunk height where the growth begins appearing:
    /// 0 will cause it along the entire trunk (above the first tile), 100 will cause it to appear
    /// at the topmost trunk tile. Can be larger than 100 to cause it to appear above the trunk.
    /// The second value must be -1, but might be intended to control whether it starts height counting
    /// from the bottom or top.
    trunk_height_percentage: Option<[i32; 2]>,
    /// Currently has no effect.
    density: Option<u32>,
    /// Specifies the appearance of the growth. This is defined with "GROWTH_PRINT" key.
    /// This is a string until we make a proper print structure.
    print: Option<String>,
    /// Specifies at which part of the year the growth appears. Default is all year round.
    /// Minimum: 0, Maximum: 402_200. This is defined with "GROWTH_TIMING" key.
    timing: Option<[u32; 2]>,
    /// Where we gather some of the growth's tags.
    tags: Option<Vec<GrowthTag>>,
}

impl PlantGrowth {
    pub fn new(growth_type: GrowthType) -> PlantGrowth {
        PlantGrowth {
            growth_type,
            ..PlantGrowth::default()
        }
    }
    #[allow(clippy::too_many_lines)]
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(tag) = GROWTH_TOKENS.get(key) else {
            warn!(
                "PlantGrowthParsing: called `Option::unwrap()` on a `None` value for presumed caste tag: {}",
                key
            );
            return;
        };

        if value.is_empty() {
            if self.tags.is_none() {
                self.tags = Some(Vec::new());
            }
            // If there is no value, we just add the tag to the list.
            if let Some(tags) = &mut self.tags {
                tags.push(tag.clone());
            }
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
                if self.host_tiles.is_none() {
                    self.host_tiles = Some(Vec::new());
                }
                let Some(part) = PLANT_PART_TOKENS.get(value) else {
                    warn!(
                        "PlantGrowthParsing: called `Option::unwrap()` on a `None` value for presumed plant part: {}",
                        value
                    );
                    return;
                };
                if let Some(host_tiles) = &mut self.host_tiles {
                    host_tiles.push(part.clone());
                }
            }
            GrowthTag::GrowthTrunkHeightPercent => {
                let split: Vec<&str> = value.split(':').collect::<Vec<&str>>();
                if split.len() != 2 {
                    warn!(
                        "PlantGrowthParsing: called `Option::unwrap()` on a `None` value for presumed plant part: {}",
                        value
                    );
                    return;
                }
                let percentage: i32 = match split.first().unwrap_or(&"").parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("min_value parsing error\n{:?}", e);
                        return;
                    }
                };
                let dir: i32 = match split.get(1).unwrap_or(&"").parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("max_value parsing error\n{:?}", e);
                        return;
                    }
                };
                self.trunk_height_percentage = Some([percentage, dir]);
            }
            GrowthTag::GrowthDensity => {
                self.density = Some(value.parse().unwrap_or_default());
            }
            GrowthTag::GrowthTiming => {
                let split: Vec<&str> = value.split(':').collect::<Vec<&str>>();
                if split.len() != 2 {
                    warn!(
                        "PlantGrowthParsing: called `Option::unwrap()` on a `None` value for presumed plant part: {}",
                        value
                    );
                    return;
                }
                let start: u32 = match split.first().unwrap_or(&"").parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("min_value parsing error\n{:?}", e);
                        return;
                    }
                };
                let end: u32 = match split.get(1).unwrap_or(&"").parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("max_value parsing error\n{:?}", e);
                        return;
                    }
                };
                self.timing = Some([start, end]);
            }
            GrowthTag::GrowthPrint => {
                self.print = Some(value.to_string());
            }
            _ => {
                // If we don't recognize the tag, we just add it to the list.
                if self.tags.is_none() {
                    self.tags = Some(Vec::new());
                }
                if let Some(tags) = &mut self.tags {
                    tags.push(tag.clone());
                }
            }
        }
    }

    /// Function to "clean" the raw. This is used to remove any empty list or strings,
    /// and to remove any default values. By "removing" it means setting the value to None.
    ///
    /// This also will remove the metadata if `is_metadata_hidden` is true.
    ///
    /// Steps for all "Option" fields:
    /// - Set any metadata to None if `is_metadata_hidden` is true.
    /// - Set any empty string to None.
    /// - Set any empty list to None.
    /// - Set any default values to None.
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(host_tiles) = &cleaned.host_tiles {
            if host_tiles.is_empty() {
                cleaned.host_tiles = None;
            }
        }

        if let Some(tags) = &cleaned.tags {
            if tags.is_empty() {
                cleaned.tags = None;
            }
        }

        if serializer_helper::is_default_trunk_height_percentage(&cleaned.trunk_height_percentage) {
            cleaned.trunk_height_percentage = None;
        }
        if serializer_helper::is_default_growth_density(cleaned.density) {
            cleaned.density = None;
        }
        if serializer_helper::is_default_growth_timing(&cleaned.timing) {
            cleaned.timing = None;
        }

        if let Some(print) = &cleaned.print {
            if print.is_empty() {
                cleaned.print = None;
            }
        }

        cleaned
    }
}

impl Searchable for PlantGrowth {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.extend(self.name.as_vec());
        vec.push(format!("{:?}", self.growth_type));
        vec.push(self.item.clone());
        if let Some(tags) = &self.tags {
            vec.extend(tags.iter().map(|tag| format!("{tag:?}")));
        }

        clean_search_vec(vec.as_slice())
    }
}
