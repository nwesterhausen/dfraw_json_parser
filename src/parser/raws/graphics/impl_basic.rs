use crate::parser::raws::{info_txt::DFInfoFile, DFRawCommon, RawObjectKind};

use super::{DFGraphic, Kind, SpriteGraphic};

impl DFGraphic {
    pub fn new(raw: &str, token: &str, info_text: &DFInfoFile) -> Self {
        // [CREATURE_CASTE_GRAPHICS:<creature id>:<caste id>]
        // [CREATURE_GRAPHICS:<creature id>]
        // [TILE_GRAPHICS:ITEMS4:0:1:ITEM_BRANCH]
        //idx 0           1      2 3 4
        // [TILE_GRAPHICS:ITEM_COFFIN:1:0:ITEM_COFFIN_WOOD_UNUSED]
        //idx 0           1           2 3 4
        let split = token.split(':').collect::<Vec<&str>>();

        match *split.first().unwrap_or(&"") {
            "TILE_GRAPHICS" => {
                let Some(identifier) = split.get(4) else {
                    return Self {
                        raw_header: DFRawCommon::from(
                            split[1],
                            raw,
                            info_text,
                            RawObjectKind::Graphics,
                        ),
                        caste_identifier: String::new(),
                        graphics: Vec::new(),
                        kind: Kind::Empty,
                    }
                };
                let graphic = match SpriteGraphic::parse(token) {
                    Some(g) => g,
                    None => {
                        return Self {
                            raw_header: DFRawCommon::from(
                                split[1],
                                raw,
                                info_text,
                                RawObjectKind::Graphics,
                            ),
                            caste_identifier: String::new(),
                            graphics: Vec::new(),
                            kind: Kind::Empty,
                        }
                    }
                };

                Self {
                    raw_header: DFRawCommon::from(
                        identifier,
                        raw,
                        info_text,
                        RawObjectKind::Graphics,
                    ),
                    caste_identifier: String::new(),
                    graphics: vec![graphic],
                    kind: Kind::Tile,
                }
            }
            "CREATURE_CASTE_GRAPHICS" => Self {
                raw_header: DFRawCommon::from(split[1], raw, info_text, RawObjectKind::Graphics),
                caste_identifier: String::from(split[2]),
                graphics: Vec::new(),
                kind: Kind::Empty,
            },
            "CREATURE_GRAPHICS" => Self {
                raw_header: DFRawCommon::from(split[1], raw, info_text, RawObjectKind::Graphics),
                caste_identifier: String::new(),
                graphics: Vec::new(),
                kind: Kind::Creature,
            },
            "PLANT_GRAPHICS" => Self {
                raw_header: DFRawCommon::from(split[1], raw, info_text, RawObjectKind::Graphics),
                caste_identifier: String::new(),
                graphics: Vec::new(),
                kind: Kind::Plant,
            },
            _ => Self {
                raw_header: DFRawCommon::from(token, raw, info_text, RawObjectKind::Graphics),
                caste_identifier: String::new(),
                graphics: Vec::new(),
                kind: Kind::Empty,
            },
        }
    }
    pub fn add_tile_from_token(&mut self, token: &str) {
        match self.kind {
            Kind::Creature | Kind::CreatureCaste => {
                let graphic = match SpriteGraphic::parse_creature(token) {
                    Some(g) => g,
                    None => {
                        return;
                    }
                };
                self.graphics.push(graphic);
            }
            Kind::Tile => {
                let graphic = match SpriteGraphic::parse(token) {
                    Some(g) => g,
                    None => {
                        return;
                    }
                };
                self.graphics.push(graphic);
            }
            Kind::Plant => {
                let graphic = match SpriteGraphic::parse_plant(token) {
                    Some(g) => g,
                    None => {
                        return;
                    }
                };
                self.graphics.push(graphic);
            }
            Kind::Empty => {}
        }
    }
    pub fn get_raw_header(&self) -> &DFRawCommon {
        &self.raw_header
    }
    pub fn get_caste_identifier(&self) -> String {
        self.caste_identifier.to_string()
    }
    pub fn get_kind(&self) -> Kind {
        self.kind
    }
    pub fn get_graphics(&self) -> Vec<SpriteGraphic> {
        // cloning vec
        self.graphics.clone()
    }
}
