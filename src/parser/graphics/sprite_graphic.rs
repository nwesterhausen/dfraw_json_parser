use serde::{Deserialize, Serialize};

use super::{
    dimensions::Dimensions,
    phf_table::GRAPHIC_TYPE_TAGS,
    tokens::{ColorModification, Condition, GraphicType},
};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpriteGraphic {
    primary_condition: Condition,
    tile_page_id: String,
    offset: Dimensions,
    color: ColorModification,
    large_image: bool,
    offset2: Dimensions,
    secondary_condition: Condition,
}

impl SpriteGraphic {
    pub fn from_token(key: &str, value: &str) -> Option<Self> {
        todo!("fix parsing---no obj type");
        // Recombine token for parsing
        let token = format!("{key}:{value}");
        // The key should match our GRAPHIC_TYPE_TOKENS
        if !GRAPHIC_TYPE_TAGS.contains_key(key) {
            log::warn!(
                "Failed to parse {} as SpriteGraphic, unknown key {}",
                value,
                key
            );
            return None;
        }

        match GRAPHIC_TYPE_TAGS.get(key).unwrap_or(&GraphicType::Unknown) {
            GraphicType::Creature | GraphicType::CreatureCaste => {
                // parse creature
                SpriteGraphic::parse_creature_from_token(&token)
            }
            GraphicType::Plant => {
                // parse plant
                None
            }
            GraphicType::Tile => SpriteGraphic::parse_tile_from_value(value),
            _ => {
                log::warn!(
                    "Failed to parse {} as SpriteGraphic, unknown key {}",
                    value,
                    key
                );
                None
            }
        }
    }
    fn parse_tile_from_value(value: &str) -> Option<Self> {
        // ...ITEMS4:0:1:ITEM_BRANCH]
        // ...ITEM_COFFIN:1:0:ITEM_COFFIN_WOOD_UNUSED]

        let mut split = value.split(':');
        let tile_sheet = match split.next() {
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

        // Target identifier is optional
        // let tile_target_identifier = match split.next() {
        //     Some(v) => String::from(v),
        //     _ => String::new(),
        // };

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
            primary_condition: Condition::None,
            tile_page_id: tile_sheet,
            offset: Dimensions::from_xy(offset_x, offset_y),
            color: ColorModification::AsIs,
            large_image: false,
            offset2: Dimensions::zero(),
            secondary_condition: Condition::None,
        })
    }
    fn parse_creature_from_token(token: &str) -> Option<Self> {
        // [<condition>:<tile page identifier>:<x position>:<y position>:<color type>:<secondary condition>]
        //   0           1                      2            3             4            5
        // [<condition>:<tile page identifier>:LARGE_IMAGE:<x1>:<y1>:<x2>:<y2>:<color type>:<secondary condition>]
        //   0           1                      2          3    4     5    6    7            8

        // Based on the length of the split, we can determine if this is a large image or not
        let mut split = token.split(':');

        let condition = match split.next() {
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

        let large_image = match split.next() {
            Some(v) => v == "LARGE_IMAGE",
            _ => false,
        };

        if large_image {
            return SpriteGraphic::parse_large_creature_with_split(
                condition.as_str(),
                tile_page_id.as_str(),
                split.collect::<Vec<&str>>().as_slice(),
            );
        }

        let x1: i32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as x1, {:?}", v, e);
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let y1: i32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as y1, {:?}", v, e);
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let color = match split.next() {
            Some(v) => ColorModification::from_token(v),
            _ => ColorModification::AsIs,
        };

        let secondary_condition = match split.next() {
            Some(v) => Condition::from_token(v),
            _ => Condition::None,
        };

        Some(Self {
            primary_condition: Condition::from_token(condition.as_str()),
            tile_page_id,
            offset: Dimensions::from_xy(x1, y1),
            color,
            large_image,
            offset2: Dimensions::zero(),
            secondary_condition,
        })
    }
    fn parse_large_creature_with_split(
        condition: &str,
        tile_page_id: &str,
        split: &[&str],
    ) -> Option<Self> {
        // [<condition>:<tile page identifier>:LARGE_IMAGE:<x1>:<y1>:<x2>:<y2>:<color type>:<secondary condition>]
        //   0           1                      2          3    4     5    6    7            8

        // We know it's a large image, and the split should contain 6 elements (x1, y1, x2, y2, color type, secondary condition)
        if split.len() != 6 {
            log::warn!(
                "Failed to parse {} as SpriteGraphic, invalid number of tokens",
                split.join(":")
            );
            return None;
        }

        let x1: i32 = match split.first() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_x1, {:?}", v, e);
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
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_y1, {:?}", v, e);
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
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_x2, {:?}", v, e);
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
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_y2, {:?}", v, e);
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let color = match split.get(4) {
            Some(v) => ColorModification::from_token(v),
            _ => ColorModification::AsIs,
        };

        let secondary_condition = match split.get(5) {
            Some(v) => Condition::from_token(v),
            _ => Condition::None,
        };

        Some(Self {
            primary_condition: Condition::from_token(condition),
            tile_page_id: String::from(tile_page_id),
            offset: Dimensions::from_xy(x1, y1),
            color,
            large_image: true,
            offset2: Dimensions::from_xy(x2, y2),
            secondary_condition,
        })
    }
}
