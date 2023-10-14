use serde::{Deserialize, Serialize};

use super::phf_table::CONDITION_TAGS;

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
    Baby,
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
    Remains,
    Vermin,
    LightVermin,
    Hive,
    SwarmSmall,
    SwarmMedium,
    SwarmLarge,

    NotArtifact,
    CraftedArtifact,
    Dye,
    NotDyed,

    Crop,
    Seed,
    Picked,
    Shrub,
    Sapling,
    CropSprout,
    CropL,
    CropM,
    CropR,
    ShrubDead,

    NotChild,
    HaulCountMin,
    HaulCountMax,
    ItemWorn,
    ProfessionCategory,
    SyndromeClass,
    Caste,
    TissueLayer,
    MaterialFlag,
    MaterialType,
    ShutOffIfItemPresent,
    RandomPartIndex,
    Ghost,
    TissueMayHaveColor,
    TissueMinLength,
    TissueMaxLength,
    TissueMayHaveShaping,
    TissueNotShaped,
    TissueSwap,
    // Additional Conditions
    Layer,
    LayerSet,
    LayerGroup,
    EndLayerGroup,
    BodyUpper,
    CopyOfTemplate,
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PlantGraphicTemplate {
    StandardLeaves,
    StandardFruit1,
    StandardFruit2,
    StandardFruit3,
    StandardFruit4,
    StandardFlowers1,
    StandardFlowers2,
    StandardFlowers3,
    StandardFlowers4,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum GrowthTag {
    Fruit,
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
    Template,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum TilePageTag {
    TileDim,
    PageDim,
    File,
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
        if let Some(condition) = CONDITION_TAGS.get(token) {
            *condition
        } else {
            log::warn!("Failed to parse {} as Condition", token);
            Self::default()
        }
    }
}
