use serde::{Deserialize, Serialize};

use super::{dimensions::Dimensions, phf_table::LAYER_CONDITION_TAGS, tokens::LayerCondition};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpriteLayer {
    layer_name: String,
    tile_page_id: String,
    offset: Dimensions,
    conditions: Vec<(LayerCondition, i32)>,
}

impl SpriteLayer {
    pub fn from_token(key: &str, value: &str) -> Option<Self> {
        // First check if key === "LAYER" which means a new layer is starting..
        if let "LAYER" = key {
            // Parse the value into a SpriteLayer
            Self::parse_layer_from_value(value)
        } else {
            // Not a layer, return None
            None
        }
    }
    pub fn parse_condition_token(&mut self, key: &str, value: &str) {
        // Condition is the key, and it should match a value in LAYER_CONDITION_TAGS
        if let Some(condition) = LAYER_CONDITION_TAGS.get(key) {
            // Some conditions do not have an i32 value, so we need to check if the condition
            // is one of those. (e.g. CONDITION_NOT_CHILD, CONDITION_CHILD)
            if let LayerCondition::ConditionNotChild | LayerCondition::ConditionChild = condition {
                // These conditions do not have an i32 value, so we can just add them to the list
                self.conditions.push((condition.clone(), 0));
                return;
            }
            // Parse the value into an i32
            let value: i32 = match value.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as i32, {:?}", value, e);
                    return;
                }
            };

            // Add the condition to the list of conditions
            self.conditions.push((condition.clone(), value));
        } else {
            log::warn!(
                "Failed to parse {} as LayerCondition, unknown key {}",
                value,
                key
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
        let tile_offset_x = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };
        let tile_offset_y = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let offset_x: i32 = match tile_offset_x.parse() {
            Ok(n) => n,
            Err(e) => {
                log::warn!("Failed to parse {} as offset_x, {:?}", tile_offset_x, e);
                return None;
            }
        };

        let offset_y: i32 = match tile_offset_y.parse() {
            Ok(n) => n,
            Err(e) => {
                log::warn!("Failed to parse {} as offset_y, {:?}", tile_offset_y, e);
                return None;
            }
        };

        Some(Self {
            layer_name,
            tile_page_id,
            offset: Dimensions::from_xy(offset_x, offset_y),
            conditions: Vec::new(),
        })
    }
}
