use serde::{Deserialize, Serialize};

mod impl_basic;
mod impl_enums;
mod parse;
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
struct Graphic {
    primary_condition: Condition,
    tile_page_id: String,
    offset_x: i32,
    offset_y: i32,
    color: Color,
    large_image: bool,
    offset_x_2: i32,
    offset_y_2: i32,
    secondary_condition: Condition,
}

struct TileGraphicsRectangle {
    tile_page_id: String,
    offset_x: i32,
    offset_y: i32,
    offset_x_2: i32,
    offset_y_2: i32,
    body_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Creature,
    CreatureCaste,
    Tile,
    Empty,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteGraphic {
    identifier: String,
    caste_identifier: String,
    pub kind: Kind,
    graphics: Vec<Graphic>,
}
