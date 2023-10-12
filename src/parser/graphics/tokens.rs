use serde::{Deserialize, Serialize};

use super::phf_table::GRAPHIC_CONDITION_TAGS;

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Condition {
    None,
    #[default]
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
    // Additional Conditions
    Layer,
    LayerSet,
    LayerGroup,
    EndLayerGroup,
    BodyUpper,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum ColorModification {
    #[default]
    AsIs,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphicType {
    Creature,
    CreatureCaste,
    Tile,
    Empty,
    Plant,
    #[default]
    Unknown,
}

impl ColorModification {
    pub fn from_token(token: &str) -> Self {
        if let "AS_IS" = token {
            Self::AsIs
        } else {
            log::warn!("Failed to parse {} as ColorModification", token);
            Self::default()
        }
    }
}

impl Condition {
    pub fn from_token(token: &str) -> Self {
        if let Some(condition) = GRAPHIC_CONDITION_TAGS.get(token) {
            *condition
        } else {
            log::warn!("Failed to parse {} as Condition", token);
            Self::default()
        }
    }
}
