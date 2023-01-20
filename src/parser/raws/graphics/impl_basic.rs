use crate::parser::raws::dimensions::Dimensions;

use super::{Color, Condition, DFGraphic, Kind, SpriteGraphic};

impl SpriteGraphic {
    pub fn from_token(token: String) -> Option<SpriteGraphic> {
        // [CREATURE_CASTE_GRAPHICS:<creature id>:<caste id>]
        // [CREATURE_GRAPHICS:<creature id>]
        let split = token.split(':').collect::<Vec<&str>>();
        match split[0] {
            "CREATURE_GRAPHICS" => Some(SpriteGraphic {
                caste_identifier: String::new(),
                graphics: Vec::new(),
                identifier: String::from(split[1]),
                kind: Kind::Creature,
            }),
            "CREATURE_CASTE_GRAPHICS" => Some(SpriteGraphic {
                caste_identifier: String::from(split[2]),
                identifier: String::from(split[1]),
                kind: Kind::CreatureCaste,
                graphics: Vec::new(),
            }),
            "TILE_GRAPHICS" => {
                let Some(identifier) = split.get(4) else {
                    return None;
                };
                let graphic = match DFGraphic::parse(token.clone()) {
                    Some(g) => g,
                    None => {
                        return Some(SpriteGraphic {
                            caste_identifier: String::new(),
                            graphics: Vec::new(),
                            identifier: String::from(*identifier),
                            kind: Kind::Tile,
                        });
                    }
                };

                Some(SpriteGraphic {
                    caste_identifier: String::new(),
                    graphics: vec![graphic],
                    identifier: String::from(*identifier),
                    kind: Kind::Tile,
                })
            }
            _ => None,
        }
    }
    pub fn add_tile_from_token(&mut self, token: String) {
        match self.kind {
            Kind::Creature | Kind::CreatureCaste => {
                let graphic = match DFGraphic::parse_creature(token) {
                    Some(g) => g,
                    None => {
                        return;
                    }
                };
                self.graphics.push(graphic);
            }
            Kind::Tile => {
                let graphic = match DFGraphic::parse(token) {
                    Some(g) => g,
                    None => {
                        return;
                    }
                };
                self.graphics.push(graphic);
            }
            Kind::Empty => {
                return;
            }
        }
    }
    pub fn empty() -> Self {
        Self {
            identifier: String::from("dfraw_json_parser empty"),
            caste_identifier: String::new(),
            kind: Kind::Empty,
            graphics: Vec::new(),
        }
    }
}

impl DFGraphic {
    pub fn parse(token: String) -> Option<Self> {
        let split = token.split(':').collect::<Vec<&str>>();
        match split[0] {
            "CREATURE_GRAPHICS" | "CREATURE_CASTE_GRAPHICS" => Self::parse_creature(token),
            "TILE_GRAPHICS" => Self::parse_tile(token),
            _ => {
                log::debug!("Unable to parse graphic from {}", token);
                return None;
            }
        }
    }
    pub fn parse_tile(token: String) -> Option<Self> {
        // [TILE_GRAPHICS:ITEMS4:0:1:ITEM_BRANCH]
        // [TILE_GRAPHICS:ITEM_COFFIN:1:0:ITEM_COFFIN_WOOD_UNUSED]
        let split = token.split(':').collect::<Vec<&str>>();
        if !split[0].eq("TILE_GRAPHICS") {
            return None;
        }
        let tile_page_id = match split.get(1) {
            Some(s) => String::from(*s),
            None => {
                log::error!(
                    "Missing required number of tokens for TILE_GRAPHICS! {}",
                    token
                );
                return None;
            }
        };
        let offset_x: i32 = match split.get(2) {
            Some(s) => match s.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_x, {:?}", split[2], e);
                    return None;
                }
            },
            None => {
                log::error!(
                    "Missing required number of tokens for TILE_GRAPHICS! {}",
                    token
                );
                return None;
            }
        };
        let offset_y: i32 = match split.get(3) {
            Some(s) => match s.parse() {
                Ok(n) => n,
                Err(e) => {
                    log::warn!("Failed to parse {} as offset_x, {:?}", split[2], e);
                    return None;
                }
            },
            None => {
                log::error!(
                    "Missing required number of tokens for TILE_GRAPHICS! {}",
                    token
                );
                return None;
            }
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
    pub fn parse_creature(token: String) -> Option<Self> {
        // [<condition>:<tile page identifier>:<x position>:<y position>:<color type>:<secondary condition>]
        // [<condition>:<tile page identifier>:LARGE_IMAGE:<x1>:<y1>:<x2>:<y2>:<color type>:<secondary condition>]
        let split = token.split(':').collect::<Vec<&str>>();

        let partition0 = split[0];

        match partition0 {
            "LAYER_SET" | "LAYER" => {
                return None;
            }
            "CONDITION_CHILD" | "CONDITION_NOT_CHILD" | "CONDITION_HAUL_COUNT" => {
                return None;
            }
            "CONDITION_HAUL_COUNT_MIN" | "CONDITION_HAUL_COUNT_MAX" => {
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
        match *partition2 {
            "LARGE_IMAGE" => {
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
                let offset_x_2 = match split[5].parse() {
                    Ok(n) => n,
                    Err(e) => {
                        log::warn!("Failed to parse {} as offset_x, {:?}", split[2], e);
                        return None;
                    }
                };
                let offset_y_2 = match split[6].parse() {
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
                        offset2: Dimensions::from_xy(offset_x_2, offset_y_2),
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
                        offset2: Dimensions::from_xy(offset_x_2, offset_y_2),
                        secondary_condition: Condition::None,
                    });
                }

                let secondary_condition = Condition::from_str(split[5]);

                return Some(Self {
                    primary_condition,
                    tile_page_id: String::from(*tile_page_id),
                    offset: Dimensions::from_xy(offset_x, offset_y),
                    color,
                    large_image: true,
                    offset2: Dimensions::from_xy(offset_x_2, offset_y_2),
                    secondary_condition,
                });
            }
            _ => {
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

                return Some(Self {
                    primary_condition,
                    tile_page_id: String::from(*tile_page_id),
                    offset: Dimensions::from_xy(offset_x, offset_y),
                    color,
                    large_image: false,
                    offset2: Dimensions::zero(),
                    secondary_condition,
                });
            }
        };
    }
}
