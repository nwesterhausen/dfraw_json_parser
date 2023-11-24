use serde::{Deserialize, Serialize};
use tracing::warn;

use super::{dimensions::Dimensions, phf_table::CONDITION_TAGS, tokens::Condition};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpriteLayer {
    layer_name: String,
    tile_page_id: String,
    offset: Dimensions,
    offset_2: Dimensions,
    large_image: bool,
    conditions: Vec<(Condition, String)>,
}

impl SpriteLayer {
    pub fn get_tile_page_id(&self) -> &str {
        self.tile_page_id.as_str()
    }
    pub fn parse_condition_token(&mut self, key: &str, value: &str) {
        // Condition is the key, and it should match a value in LAYER_CONDITION_TAGS
        if let Some(condition) = CONDITION_TAGS.get(key) {
            // It's true that some conditions have a value, some have a tag, and some are standalone.
            // At the moment we only care about saving the tag, so we'll just save the value as a string.
            self.conditions.push((*condition, String::from(value)));
        } else {
            // Manually avoid ISSUE_MIN_LENGTH which is a typo in one of the mods.. This hack should be removed once the mod is fixed.
            if key == "ISSUE_MIN_LENGTH" {
                return;
            }
            warn!(
                "Failed to parse {} as LayerCondition, unknown key {}",
                value, key
            );
        }
    }
    pub fn parse_layer_from_value(value: &str) -> Option<Self> {
        // 		...BODY:CREATURES_DOMESTIC:0:21]
        let mut split = value.split(':');

        let layer_name = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };
        let tile_page_id = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let fourth_position_token = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let large_image = matches!(fourth_position_token.as_str(), "LARGE_IMAGE");

        if large_image {
            return Self::parse_large_layer_with_split(
                layer_name.as_str(),
                tile_page_id.as_str(),
                split.collect::<Vec<&str>>().as_slice(),
            );
        }

        let tile_offset_y = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let offset_x: i32 = match fourth_position_token.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_layer_from_value: Failed to parse {} as offset_x, {}",
                    fourth_position_token, value
                );
                return None;
            }
        };

        let offset_y: i32 = match tile_offset_y.parse() {
            Ok(n) => n,
            Err(_e) => {
                warn!(
                    "parse_layer_from_value: Failed to parse {} as offset_y, {}",
                    tile_offset_y, value
                );
                return None;
            }
        };

        Some(Self {
            layer_name,
            tile_page_id,
            offset: Dimensions::from_xy(offset_x, offset_y),
            ..Self::default()
        })
    }

    fn parse_large_layer_with_split(
        layer_name: &str,
        tile_page_id: &str,
        split: &[&str],
    ) -> Option<SpriteLayer> {
        let x1: i32 = match split.first() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_large_creature_with_split: Failed to parse {} as offset_x1 {:?}",
                        v, split
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let y1: i32 = match split.get(1) {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_large_creature_with_split: Failed to parse {} as offset_y1 {:?}",
                        v, split
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let x2: i32 = match split.get(2) {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_large_creature_with_split: Failed to parse {} as offset_x2 {:?}",
                        v, split
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let y2: i32 = match split.get(3) {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    warn!(
                        "parse_large_creature_with_split: Failed to parse {} as offset_y2 {:?}",
                        v, split
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        Some(Self {
            layer_name: String::from(layer_name),
            tile_page_id: String::from(tile_page_id),
            large_image: true,
            offset: Dimensions::from_xy(x1, y1),
            offset_2: Dimensions::from_xy(x2, y2),
            ..Self::default()
        })
    }
}
