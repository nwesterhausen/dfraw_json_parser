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
    Zombie,
    Necromancer,
    Male,
    Female,
    VampireCursed,
    Ghoul,
    DisturbedDead,
    NotArtifact,
    CraftedArtifact,
    Dye,
    NotDyed,
    Crop,
    Seed,
    Picked,
    Shrub,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Color {
    AsIs,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteGraphic {
    #[serde(rename = "primaryCondition")]
    primary_condition: Condition,
    #[serde(rename = "tilePageId")]
    tile_page_id: String,
    offset: Dimensions,
    color: Color,
    #[serde(rename = "largeImage")]
    large_image: bool,
    offset2: Dimensions,
    #[serde(rename = "secondaryCondition")]
    secondary_condition: Condition,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Creature,
    CreatureCaste,
    Tile,
    Empty,
    Plant,
}

#[derive(Debug, Clone)]
pub struct DFGraphic {
    // Common Raw file Things
    raw_header: DFRawCommon,

    caste_identifier: String,
    pub kind: Kind,
    graphics: Vec<SpriteGraphic>,
}
