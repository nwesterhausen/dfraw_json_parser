use crate::parser::raws::dimensions::Dimensions;

use super::{Color, Condition, SpriteGraphic};

impl SpriteGraphic {
    pub fn parse(token: &str) -> Option<Self> {
        let split = token.split(':').collect::<Vec<&str>>();
        match *split.first().unwrap_or(&"") {
            "CREATURE_GRAPHICS" | "CREATURE_CASTE_GRAPHICS" => Self::parse_creature(token),
            "TILE_GRAPHICS" => Self::parse_tile(token),
            "SHRUB" | "PICKED" | "SEED" | "CROP" => Self::parse_plant(token),
            _ => {
                log::debug!("Unable to parse graphic from {}", token);
                None
            }
        }
    }
    pub fn parse_tile(token: &str) -> Option<Self> {
        // [TILE_GRAPHICS:ITEMS4:0:1:ITEM_BRANCH]
        //idx 0           1      2 3 4
        // [TILE_GRAPHICS:ITEM_COFFIN:1:0:ITEM_COFFIN_WOOD_UNUSED]
        //idx 0           1           2 3 4
        let split = token.split(':').collect::<Vec<&str>>();

        let object_type = match split.first() {
            Some(v) => *v,
            _ => {
                return None;
            }
        };
        let tile_sheet = match split.get(1) {
            Some(v) => *v,
            _ => {
                return None;
            }
        };
        let tile_offset_x = match split.get(2) {
            Some(v) => *v,
            _ => {
                return None;
            }
        };
        let tile_offset_y = match split.get(3) {
            Some(v) => *v,
            _ => {
                return None;
            }
        };

        // let tile_target_identifier = match split.get(4) {
        //     Some(v) => *v,
        //     _ => {
        //         return None;
        //     }
        // };

        if !object_type.eq("TILE_GRAPHICS") {
            return None;
        }

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
            tile_page_id: String::from(tile_sheet),
            offset: Dimensions::from_xy(offset_x, offset_y),
            color: Color::AsIs,
            large_image: false,
            offset2: Dimensions::zero(),
            secondary_condition: Condition::None,
        })
    }
    #[allow(clippy::too_many_lines)]
    pub fn parse_creature(token: &str) -> Option<Self> {
        // [<condition>:<tile page identifier>:<x position>:<y position>:<color type>:<secondary condition>]
        //   0           1                      2            3             4            5
        // [<condition>:<tile page identifier>:LARGE_IMAGE:<x1>:<y1>:<x2>:<y2>:<color type>:<secondary condition>]
        //   0           1                      2          3    4     5    6    7            8
        let split = token.split(':').collect::<Vec<&str>>();

        let first_condition = match split.first() {
            Some(v) => *v,
            _ => {
                return None;
            }
        };
        let tile_sheet = match split.get(1) {
            Some(v) => *v,
            _ => {
                return None;
            }
        };
        let token3 = match split.get(2) {
            Some(v) => *v,
            _ => {
                return None;
            }
        };

        match first_condition {
            "LAYER_SET"
            | "LAYER"
            | "CONDITION_CHILD"
            | "CONDITION_NOT_CHILD"
            | "CONDITION_HAUL_COUNT"
            | "CONDITION_HAUL_COUNT_MIN"
            | "CONDITION_HAUL_COUNT_MAX"
            | "CONDITION_PROFESSION_CATEGORY"
            | "CONDITION_MATERIAL_FLAG"
            | "CONDITION_DYE"
            | "CONDITION_NOT_DYED"
            | "LAYER_GROUP"
            | "BODY_UPPER"
            | "CONDITION_SYN_CLASS"
            | "CONDITION_CASTE"
            | "TISSUE_MAY_HAVE_SHAPING"
            | "TISSUE_NOT_SHAPED"
            | "TISSUE_MIN_LENGTH"
            | "TISSUE_MAX_LENGTH"
            | "CONDITION_GHOST"
            | "END_LAYER_GROUP" => {
                return None;
            }
            _ => (),
        }

        let primary_condition = Condition::from_str(first_condition);

        // everything is different if large or not
        if let "LARGE_IMAGE" = token3 {
            // Large means we have 2 sets of offsets.
            let first_offset_x = match split.get(3) {
                Some(v) => *v,
                _ => {
                    return None;
                }
            };
            let first_offset_y = match split.get(4) {
                Some(v) => *v,
                _ => {
                    return None;
                }
            };
            let second_offset_x = match split.get(5) {
                Some(v) => *v,
                _ => {
                    return None;
                }
            };
            let second_offset_y = match split.get(6) {
                Some(v) => *v,
                _ => {
                    return None;
                }
            };

            let offset_x = match first_offset_x.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_x, {:?}", first_offset_x, e);
                    return None;
                }
            };
            let offset_y = match first_offset_y.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_y, {:?}", first_offset_y, e);
                    return None;
                }
            };
            let final_offset_x = match second_offset_x.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_x, {:?}", second_offset_x, e);
                    return None;
                }
            };
            let final_offset_y = match second_offset_y.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_y, {:?}", second_offset_y, e);
                    return None;
                }
            };

            // Color is optional
            let color_token = match split.get(7) {
                Some(v) => *v,
                _ => {
                    return Some(Self {
                        primary_condition,
                        tile_page_id: String::from(tile_sheet),
                        offset: Dimensions::from_xy(offset_x, offset_y),
                        color: Color::AsIs,
                        large_image: true,
                        offset2: Dimensions::from_xy(final_offset_x, final_offset_y),
                        secondary_condition: Condition::None,
                    });
                }
            };

            let color = Color::from_str(color_token);

            // Secondary Condition is Optional
            let second_condition = match split.get(8) {
                Some(v) => *v,
                _ => {
                    return Some(Self {
                        primary_condition,
                        tile_page_id: String::from(tile_sheet),
                        offset: Dimensions::from_xy(offset_x, offset_y),
                        color,
                        large_image: true,
                        offset2: Dimensions::from_xy(final_offset_x, final_offset_y),
                        secondary_condition: Condition::None,
                    });
                }
            };

            let secondary_condition = Condition::from_str(second_condition);

            Some(Self {
                primary_condition,
                tile_page_id: String::from(tile_sheet),
                offset: Dimensions::from_xy(offset_x, offset_y),
                color,
                large_image: true,
                offset2: Dimensions::from_xy(final_offset_x, final_offset_y),
                secondary_condition,
            })
        } else {
            let first_offset_y = match split.get(3) {
                Some(v) => *v,
                _ => {
                    return None;
                }
            };
            let offset_x = match token3.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_x, {:?}", token3, e);
                    return None;
                }
            };
            let offset_y = match first_offset_y.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_y, {:?}", first_offset_y, e);
                    return None;
                }
            };

            // Color is optional
            let color_token = match split.get(4) {
                Some(v) => *v,
                _ => {
                    return Some(Self {
                        primary_condition,
                        tile_page_id: String::from(tile_sheet),
                        offset: Dimensions::from_xy(offset_x, offset_y),
                        color: Color::AsIs,
                        large_image: false,
                        offset2: Dimensions::zero(),
                        secondary_condition: Condition::None,
                    });
                }
            };

            let color = Color::from_str(color_token);

            // Secondary Condition is Optional
            let second_condition = match split.get(5) {
                Some(v) => *v,
                _ => {
                    return Some(Self {
                        primary_condition,
                        tile_page_id: String::from(tile_sheet),
                        offset: Dimensions::from_xy(offset_x, offset_y),
                        color,
                        large_image: false,
                        offset2: Dimensions::zero(),
                        secondary_condition: Condition::None,
                    });
                }
            };

            let secondary_condition = Condition::from_str(second_condition);

            Some(Self {
                primary_condition,
                tile_page_id: String::from(tile_sheet),
                offset: Dimensions::from_xy(offset_x, offset_y),
                color,
                large_image: false,
                offset2: Dimensions::zero(),
                secondary_condition,
            })
        }
    }
    pub fn parse_plant(token: &str) -> Option<Self> {
        // 	[SHRUB:DARK_ELF_PLANT_STANDARD:0:0]
        let split = token.split(':').collect::<Vec<&str>>();
        log::warn!("Parsing plant from {}", token);
        let plant_type = match split.first() {
            Some(v) => *v,
            _ => {
                return None;
            }
        };

        let primary_condition = Condition::from_str(plant_type);

        let Some(tile_page_id) = split.get(1) else {
            log::warn!("Not enough pieces to tokenize in {}", token);
            return None;
        };

        let Some(partition2) = split.get(2) else {
            log::warn!("Not enough pieces to tokenize in {}", token);
            return None;
        };

        let Some(partition3) = split.get(3) else {
            log::warn!("Not enough pieces to tokenize in {}", token);
            return None;
        };

        let offset_x = match partition2.parse() {
            Ok(n) => n,
            Err(e) => {
                log::warn!("Failed to parse {} as offset_x, {:?}", partition2, e);
                return None;
            }
        };
        let offset_y = match partition3.parse() {
            Ok(n) => n,
            Err(e) => {
                log::warn!("Failed to parse {} as offset_y, {:?}", partition3, e);
                return None;
            }
        };
        Some(Self {
            primary_condition,
            tile_page_id: String::from(*tile_page_id),
            offset: Dimensions::from_xy(offset_x, offset_y),
            color: Color::AsIs,
            large_image: false,
            offset2: Dimensions::zero(),
            secondary_condition: Condition::None,
        })
    }
}
