use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::parser::serializer_helper;

use super::{
    dimensions::Dimensions,
    phf_table::{CONDITION_TAGS, GRAPHIC_TYPE_TAGS},
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
    #[serde(skip_serializing_if = "ColorModification::is_default")]
    color: ColorModification,
    #[serde(skip_serializing_if = "serializer_helper::is_false")]
    large_image: bool,
    #[serde(skip_serializing_if = "Dimensions::is_empty")]
    offset2: Dimensions,
    #[serde(skip_serializing_if = "Condition::is_none")]
    secondary_condition: Condition,
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    color_pallet_swap: u32,
    #[serde(skip_serializing_if = "String::is_empty")]
    target_identifier: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    extra_descriptor: String,
}

impl SpriteGraphic {
    pub fn get_tile_page_id(&self) -> &str {
        self.tile_page_id.as_str()
    }
    pub fn from_token(key: &str, value: &str, graphic_type: GraphicType) -> Option<Self> {
        // Recombine token for parsing
        let token = format!("{key}:{value}");
        let specific_graphic_type = GRAPHIC_TYPE_TAGS.get(key).copied().unwrap_or(graphic_type);

        match specific_graphic_type {
            GraphicType::Creature | GraphicType::CreatureCaste => {
                // parse creature
                SpriteGraphic::parse_creature_from_token(&token)
            }
            GraphicType::Plant => {
                // parse plant
                SpriteGraphic::parse_plant_from_token(&token)
            }
            GraphicType::ToolWood
            | GraphicType::ToolGlass
            | GraphicType::ToolMetal
            | GraphicType::ToolStone
            | GraphicType::ToolWoodVariant
            | GraphicType::ToolGlassVariant
            | GraphicType::ToolMetalVariant
            | GraphicType::ToolStoneVariant
            | GraphicType::ToolDamage => {
                SpriteGraphic::parse_tile_with_color_pallet_from_value(value)
            }
            GraphicType::ToolShape | GraphicType::ShapeLargeGem | GraphicType::ShapeSmallGem => {
                SpriteGraphic::parse_tile_with_extra_descriptor_from_value(value)
            }
            GraphicType::StatueCreature
            | GraphicType::StatueCreatureCaste
            | GraphicType::StatuesSurfaceGiant => {
                SpriteGraphic::parse_creature_statue_from_token(&token)
            }
            GraphicType::Template
            | GraphicType::CustomWorkshop
            | GraphicType::AddTool
            | GraphicType::Ammo
            | GraphicType::SiegeAmmo
            | GraphicType::Weapon => {
                // parse template ""
                Some(Self {
                    primary_condition: Condition::CopyOfTemplate,
                    tile_page_id: format!("{key}:{value}"),
                    ..Self::default()
                })
            }
            _ => {
                // Assume most are tiles
                if let Some(v) = SpriteGraphic::parse_tile_from_value(value) {
                    return Some(v);
                }
                log::warn!(
                    "Failed to parse {} as SpriteGraphic, unknown key {}",
                    value,
                    key
                );
                None
            }
        }
    }
    fn parse_plant_from_token(token: &str) -> Option<Self> {
        // [SHRUB:PLANT_STANDARD:0:0]
        // [PICKED:PLANT_STANDARD:1:0]
        // [Condition, TilePageId, OffsetX, OffsetY]
        let mut split = token.split(':');

        let sprite_condition = match split.next() {
            Some(v) => *CONDITION_TAGS.get(v).unwrap_or(&Condition::None),
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
            Err(_e) => {
                log::warn!(
                    "parse_plant_from_token: Failed to parse {} as offset_x, {}",
                    tile_offset_x,
                    token
                );
                return None;
            }
        };

        let offset_y: i32 = match tile_offset_y.parse() {
            Ok(n) => n,
            Err(_e) => {
                log::warn!(
                    "parse_plant_from_token: Failed to parse {} as offset_y, {}",
                    tile_offset_y,
                    token
                );
                return None;
            }
        };

        Some(Self {
            primary_condition: sprite_condition,
            tile_page_id,
            offset: Dimensions::from_xy(offset_x, offset_y),
            ..Self::default()
        })
    }
    fn parse_tile_with_color_pallet_from_value(value: &str) -> Option<Self> {
        // .[TOOL_GRAPHICS_WOOD:        1:      ITEM_BOOKCASE:      0:      0]
        // (     key                color_id    tile_page_id    offset_x   offset_y)
        let mut split = value.split(':');

        let color_id: u32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    log::warn!(
                        "parse_tile_with_color_pallet_from_value: Failed to parse {} as color_id {}",
                        v,
                        value
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

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

        let offset_x: i32 = match tile_offset_x.parse() {
            Ok(n) => n,
            Err(_e) => {
                log::warn!(
                    "parse_tile_with_color_pallet_from_value: Failed to parse {} as offset_x {}",
                    tile_offset_x,
                    value
                );
                return None;
            }
        };

        let offset_y: i32 = match tile_offset_y.parse() {
            Ok(n) => n,
            Err(_e) => {
                log::warn!(
                    "parse_tile_with_color_pallet_from_value: Failed to parse {} as offset_y {}",
                    tile_offset_y,
                    value
                );
                return None;
            }
        };

        Some(Self {
            tile_page_id: tile_sheet,
            offset: Dimensions::from_xy(offset_x, offset_y),
            color_pallet_swap: color_id,
            ..Self::default()
        })
    }
    fn parse_tile_from_value(value: &str) -> Option<Self> {
        // .[TOY_GRAPHICS:              ITEM_TOY:           1:     4:          ITEM_TOY_MINIFORGE:GLASS]
        // .[ARMOR_GRAPHICS:            ITEMS4:             1:     4:          ITEM_ARMOR_CAPE]
        // .[TOOL_GRAPHICS:             TOOLS:              0:     14:         ITEM_TOOL_HONEYCOMB]
        // .[WEAPON_GRAPHICS_DEFAULT:   WEAPONS:            2:     20]               (none)
        // .[WEAPON_GRAPHICS_UPRIGHT_1T:UPRIGHT_WEAPONS:    0:     5]                (none)
        // .[ITEMS2:                    1:          20:             ITEM_SLAB_ENGRAVED]
        // (     key                    tile_page_id    offset_x   offset_y    Option<tile_target_identifier>)
        let mut split = value.split(':');

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

        // Target identifier is optional
        let target_identifier = split.join(":");

        let offset_x: i32 = match tile_offset_x.parse() {
            Ok(n) => n,
            Err(_e) => {
                log::warn!(
                    "parse_tile_from_value: Failed to parse {} as offset_x {}",
                    tile_offset_x,
                    value
                );
                return None;
            }
        };

        let offset_y: i32 = match tile_offset_y.parse() {
            Ok(n) => n,
            Err(_e) => {
                log::warn!(
                    "parse_tile_from_value: Failed to parse {} as offset_y {}",
                    tile_offset_y,
                    value
                );
                return None;
            }
        };

        Some(Self {
            tile_page_id,
            offset: Dimensions::from_xy(offset_x, offset_y),
            target_identifier,
            ..Self::default()
        })
    }
    fn parse_tile_with_extra_descriptor_from_value(value: &str) -> Option<Self> {
        // .[TOOL_GRAPHICS_SHAPE:       LONG_DIE_8:             ITEMS4:         2:          0]
        // .[SHAPE_GRAPHICS_LARGE_GEM:  BAGUETTE_CUT_GEM:       GEMS:           1:          0]
        // .[SHAPE_GRAPHICS_SMALL_GEM:  BAGUETTE_CUT_GEM:       SMALLGEMS:      0:          0]
        // (     key                    extra_descriptor      tile_page_id    offset_x   offset_y)
        let mut split = value.split(':');

        let extra_descriptor = match split.next() {
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

        // Target identifier is optional
        let target_identifier = split.join(":");

        let offset_x: i32 = match tile_offset_x.parse() {
            Ok(n) => n,
            Err(_e) => {
                log::warn!(
                    "parse_tile_with_extra_descriptor_from_value: Failed to parse {} as offset_x {}",
                    tile_offset_x,
                    value
                );
                return None;
            }
        };

        let offset_y: i32 = match tile_offset_y.parse() {
            Ok(n) => n,
            Err(_e) => {
                log::warn!(
                    "parse_tile_with_extra_descriptor_from_value: Failed to parse {} as offset_y {}",
                    tile_offset_y,
                    value
                );
                return None;
            }
        };

        Some(Self {
            tile_page_id,
            offset: Dimensions::from_xy(offset_x, offset_y),
            target_identifier,
            extra_descriptor,
            ..Self::default()
        })
    }
    fn parse_creature_statue_from_token(token: &str) -> Option<Self> {
        // [DEFAULT:    STATUES_LAYERED:        0:  0:  0:  1]
        // [DEFAULT:    STATUES_SURFACE_LARGE:  1:  0:  1:  1]
        //  condition   tile_page_id            x1  y1  x2  y2
        let mut split = token.split(':');

        let condition = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let x1: i32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    log::warn!(
                        "parse_creature_statue_from_token: Failed to parse {} as x1 {}",
                        v,
                        token
                    );
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
                Err(_e) => {
                    log::warn!(
                        "parse_creature_statue_from_token: Failed to parse {} as y1 {}",
                        v,
                        token
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };
        let x2: i32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    log::warn!(
                        "parse_creature_statue_from_token: Failed to parse {} as x2 {}",
                        v,
                        token
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };
        let y2: i32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    log::warn!(
                        "parse_creature_statue_from_token: Failed to parse {} as y2 {}",
                        v,
                        token
                    );
                    return None;
                }
            },
            _ => {
                return None;
            }
        };

        let primary_condition =
            if let Some(parsed_condition) = Condition::from_token(condition.as_str()) {
                parsed_condition
            } else {
                log::warn!(
                "parse_creature_statue_from_token: Failed to parse {} as primary_condition in {}",
                condition,
                token
            );
                Condition::None
            };

        Some(Self {
            primary_condition,
            tile_page_id: String::from("STATUES"),
            offset: Dimensions::from_xy(x1, y1),
            offset2: Dimensions::from_xy(x2, y2),
            ..Self::default()
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

        let fourth_position_token = match split.next() {
            Some(v) => String::from(v),
            _ => {
                return None;
            }
        };

        let large_image = matches!(fourth_position_token.as_str(), "LARGE_IMAGE");

        if large_image {
            return SpriteGraphic::parse_large_creature_with_split(
                condition.as_str(),
                tile_page_id.as_str(),
                split.collect::<Vec<&str>>().as_slice(),
            );
        }

        // x1 actually is parsed from fourth_position_token
        let x1: i32 = match fourth_position_token.parse() {
            Ok(n) => n,
            Err(_e) => {
                log::warn!(
                    "parse_creature_from_token: Failed to parse {} as x1 {}",
                    fourth_position_token,
                    token
                );
                return None;
            }
        };

        let y1: i32 = match split.next() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    log::warn!(
                        "parse_creature_from_token: Failed to parse {} as y1 {}",
                        v,
                        token
                    );
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

        let primary_condition =
            if let Some(parsed_condition) = Condition::from_token(condition.as_str()) {
                parsed_condition
            } else {
                log::warn!(
                    "Failed to parse {} as primary_condition in {}",
                    condition,
                    token
                );
                Condition::None
            };

        let secondary_condition = match split.next() {
            Some(v) => {
                if let Some(condition) = Condition::from_token(v) {
                    condition
                } else {
                    log::warn!("Failed to parse {} as secondary_condition in {}", v, token);
                    Condition::None
                }
            }
            _ => Condition::None,
        };

        if primary_condition == Condition::None {
            log::warn!(
                "Failed to parse {} as primary_condition large_animal_sprite {}",
                condition,
                tile_page_id
            );
            return None;
        }

        Some(Self {
            primary_condition,
            tile_page_id,
            offset: Dimensions::from_xy(x1, y1),
            color,
            secondary_condition,
            ..Self::default()
        })
    }
    #[allow(clippy::too_many_lines)]
    fn parse_large_creature_with_split(
        condition: &str,
        tile_page_id: &str,
        split: &[&str],
    ) -> Option<Self> {
        // [<condition>:<tile page identifier>:LARGE_IMAGE:<x1>:<y1>:<x2>:<y2>:<color type>:<secondary condition>]
        //   0           1                      2          3    4     5    6    7            8
        let x1: i32 = match split.first() {
            Some(v) => match v.parse() {
                Ok(n) => n,
                Err(_e) => {
                    log::warn!(
                        "parse_large_creature_with_split: Failed to parse {} as offset_x1 {:?}",
                        v,
                        split
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
                    log::warn!(
                        "parse_large_creature_with_split: Failed to parse {} as offset_y1 {:?}",
                        v,
                        split
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
                    log::warn!(
                        "parse_large_creature_with_split: Failed to parse {} as offset_x2 {:?}",
                        v,
                        split
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
                    log::warn!(
                        "parse_large_creature_with_split: Failed to parse {} as offset_y2 {:?}",
                        v,
                        split
                    );
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

        let primary_condition = if let Some(parsed_condition) = Condition::from_token(condition) {
            parsed_condition
        } else {
            log::warn!(
                "Failed to parse {} as primary_condition in {}",
                condition,
                split.join(":")
            );
            Condition::None
        };

        let secondary_condition = match split.get(5) {
            Some(v) => {
                if let Some(condition) = Condition::from_token(v) {
                    condition
                } else {
                    log::warn!(
                        "Failed to parse {} as secondary_condition in {}",
                        v,
                        split.join(":")
                    );
                    Condition::None
                }
            }
            _ => Condition::None,
        };

        if primary_condition == Condition::None {
            log::warn!(
                "Failed to parse {} as primary_condition large_animal_sprite {}",
                condition,
                tile_page_id
            );
            return None;
        }

        Some(Self {
            primary_condition,
            tile_page_id: String::from(tile_page_id),
            offset: Dimensions::from_xy(x1, y1),
            color,
            large_image: true,
            offset2: Dimensions::from_xy(x2, y2),
            secondary_condition,
            ..Self::default()
        })
    }
}
