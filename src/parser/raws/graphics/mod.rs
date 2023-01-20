use serde::{Deserialize, Serialize};

use super::{dimensions::Dimensions, DFRawCommon};

mod impl_basic;
mod impl_enums;
mod parse;
mod sprite_graphic;
mod typed_json;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    None,
    Default,
    Animated,
    Corpse,
    Child,
    TrainedHunter,
    TrainedWar,
    ListIcon,
    Skeleton,
    SkeletonWithSkull,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Color {
    AsIs,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteGraphic {
    primary_condition: Condition,
    tile_page_id: String,
    offset: Dimensions,
    color: Color,
    large_image: bool,
    offset2: Dimensions,
    secondary_condition: Condition,
}

struct TileGraphicsRectangle {
    tile_page_id: String,
    offset: Dimensions,
    offset2: Dimensions,
    body_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Creature,
    CreatureCaste,
    Tile,
    Empty,
}

#[derive(Debug, Clone)]
pub struct DFGraphic {
    // Common Raw file Things
    raw_header: DFRawCommon,

    caste_identifier: String,
    pub kind: Kind,
    graphics: Vec<SpriteGraphic>,
}
