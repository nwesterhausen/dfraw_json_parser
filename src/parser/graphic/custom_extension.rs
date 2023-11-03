use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::parser::helpers::serializer_helper;

use crate::parser::graphic::GraphicType;

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomGraphicExtension {
    extension_type: GraphicType,
    #[serde(skip_serializing_if = "String::is_empty")]
    tile_page_id: String,
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    value_1: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    value_2: u32,
}

impl CustomGraphicExtension {
    pub fn from_value(extension_type: GraphicType, value: &str) -> Option<Self> {
        // 2 Options:
        // [CUSTOM_EDGING:          4]
        // [CUSTOM_RAMP:            6]
        // ( key )              value_1
        // [CUSTOM_EDGE_W:      GRASS_OTHER:    4:        2]
        // ( key )              tile_page_id  value_1  value_2
        let mut split = value.split(':');
        let possible_value_1 = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        // If we can parse u32 from the first value, we have the first option
        if let Ok(value_1) = possible_value_1.parse::<u32>() {
            return Some(Self {
                extension_type,
                value_1,
                ..Self::default()
            });
        }

        // Otherwise, we have the second option
        let tile_page_id = possible_value_1;
        let possible_value_1 = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };
        let possible_value_2 = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        if let (Ok(value_1), Ok(value_2)) = (
            possible_value_1.parse::<u32>(),
            possible_value_2.parse::<u32>(),
        ) {
            Some(Self {
                extension_type,
                tile_page_id,
                value_1,
                value_2,
            })
        } else {
            warn!(
                "CustomGraphicExtension::from_value: Failed to parse {} OR {} as u32",
                possible_value_1, possible_value_2
            );
            None
        }
    }
}
