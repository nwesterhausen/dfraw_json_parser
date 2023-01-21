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
        if !split[0].eq("TILE_GRAPHICS") {
            return None;
        }
        let tile_page_id = if let Some(s) = split.get(1) {
            String::from(*s)
        } else {
            log::error!(
                "Missing required number of tokens for TILE_GRAPHICS! {}",
                token
            );
            return None;
        };
        let offset_x: i32 = if let Some(s) = split.get(2) {
            match s.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_x, {:?}", split[2], e);
                    return None;
                }
            }
        } else {
            log::error!(
                "Missing required number of tokens for TILE_GRAPHICS! {}",
                token
            );
            return None;
        };
        let offset_y: i32 = if let Some(s) = split.get(3) {
            match s.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_x, {:?}", split[2], e);
                    return None;
                }
            }
        } else {
            log::error!(
                "Missing required number of tokens for TILE_GRAPHICS! {}",
                token
            );
            return None;
        };

        Some(Self {
            primary_condition: Condition::None,
            tile_page_id,
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
        // [<condition>:<tile page identifier>:LARGE_IMAGE:<x1>:<y1>:<x2>:<y2>:<color type>:<secondary condition>]
        let split = token.split(':').collect::<Vec<&str>>();

        let partition0 = split[0];

        match partition0 {
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

        let primary_condition = Condition::from_str(partition0);
        let Some(tile_page_id) = split.get(1) else {
            log::warn!("Not enough pieces to tokenize in {}", token);
            return None;
        };

        let Some(partition2) = split.get(2) else {
            log::warn!("Not enough pieces to tokenize in {}", token);
            return None;
        };

        // everything is different if large or not
        if let "LARGE_IMAGE" = *partition2 {
            let offset_x = match split[3].parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_x, {:?}", split[2], e);
                    return None;
                }
            };
            let offset_y = match split[4].parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_y, {:?}", split[3], e);
                    return None;
                }
            };
            let final_offset_x = match split[5].parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_x, {:?}", split[2], e);
                    return None;
                }
            };
            let final_offset_y = match split[6].parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_y, {:?}", split[3], e);
                    return None;
                }
            };

            // Color is optional
            if split.len() == 7 {
                return Some(Self {
                    primary_condition,
                    tile_page_id: String::from(*tile_page_id),
                    offset: Dimensions::from_xy(offset_x, offset_y),
                    color: Color::AsIs,
                    large_image: true,
                    offset2: Dimensions::from_xy(final_offset_x, final_offset_y),
                    secondary_condition: Condition::None,
                });
            }

            let color = Color::from_str(split[4]);

            // Secondary Condition is Optional
            if split.len() == 5 {
                return Some(Self {
                    primary_condition,
                    tile_page_id: String::from(*tile_page_id),
                    offset: Dimensions::from_xy(offset_x, offset_y),
                    color,
                    large_image: true,
                    offset2: Dimensions::from_xy(final_offset_x, final_offset_y),
                    secondary_condition: Condition::None,
                });
            }

            let secondary_condition = Condition::from_str(split[5]);

            Some(Self {
                primary_condition,
                tile_page_id: String::from(*tile_page_id),
                offset: Dimensions::from_xy(offset_x, offset_y),
                color,
                large_image: true,
                offset2: Dimensions::from_xy(final_offset_x, final_offset_y),
                secondary_condition,
            })
        } else {
            let offset_x = match split[2].parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_x, {:?}", split[2], e);
                    return None;
                }
            };
            let offset_y = match split[3].parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_y, {:?}", split[3], e);
                    return None;
                }
            };

            // Color is optional
            if split.len() == 4 {
                return Some(Self {
                    primary_condition,
                    tile_page_id: String::from(*tile_page_id),
                    offset: Dimensions::from_xy(offset_x, offset_y),
                    color: Color::AsIs,
                    large_image: false,
                    offset2: Dimensions::zero(),
                    secondary_condition: Condition::None,
                });
            }

            let color = Color::from_str(split[4]);

            // Secondary Condition is Optional
            if split.len() == 5 {
                return Some(Self {
                    primary_condition,
                    tile_page_id: String::from(*tile_page_id),
                    offset: Dimensions::from_xy(offset_x, offset_y),
                    color,
                    large_image: false,
                    offset2: Dimensions::zero(),
                    secondary_condition: Condition::None,
                });
            }

            let secondary_condition = Condition::from_str(split[5]);

            Some(Self {
                primary_condition,
                tile_page_id: String::from(*tile_page_id),
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
        let partition0 = split[0];

        let primary_condition = Condition::from_str(partition0);
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
