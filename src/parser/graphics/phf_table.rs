use super::tokens::{Condition, GraphicType, LayerCondition, TilePageTag};

pub static GRAPHIC_CONDITION_TAGS: phf::Map<&'static str, Condition> = phf::phf_map! {
    "DEFAULT" => Condition::Default,
    "ANIMATED" => Condition::Animated,
    "CORPSE" => Condition::Corpse,
    "CHILD" => Condition::Child,
    "TRAINED_WAR" => Condition::TrainedWar,
    "TRAINED_HUNTER" => Condition::TrainedHunter,
    "LIST_ICON" => Condition::ListIcon,
    "SKELETON" => Condition::Skeleton,
    "SKELETON_WITH_SKULL" => Condition::SkeletonWithSkull,
    "ZOMBIE" => Condition::Zombie,
    "NECROMANCER" => Condition::Necromancer,
    "MALE" => Condition::Male,
    "FEMALE" => Condition::Female,
    "VAMPCURSE" => Condition::VampireCursed,
    "GHOUL" => Condition::Ghoul,
    "DISTURBED_DEAD" => Condition::DisturbedDead,
    "NOT_ARTIFACT" => Condition::NotArtifact,
    "IS_CRAFTED_ARTIFACT" => Condition::CraftedArtifact,
    "CONDITION_DYE" => Condition::Dye,
    "CONDITION_NOT_DYED" => Condition::NotDyed,
    "SHRUB" => Condition::Shrub,
    "PICKED" => Condition::Picked,
    "SEED" => Condition::Seed,
    "CROP" => Condition::Crop,
};

pub static GRAPHIC_TYPE_TAGS: phf::Map<&'static str, GraphicType> = phf::phf_map! {
    "CREATURE_GRAPHICS" => GraphicType::Creature,
    "CREATURE_CASTE_GRAPHICS" => GraphicType::CreatureCaste,
    "TILE_GRAPHICS" => GraphicType::Tile,
    "PLANT_GRAPHICS" => GraphicType::Plant,
    "SHRUB" => GraphicType::Plant,
    "PICKED" => GraphicType::Plant,
    "SEED" => GraphicType::Plant,
    "CROP" => GraphicType::Plant,
    "SAPLING" => GraphicType::Plant,
};

pub static LAYER_CONDITION_TAGS: phf::Map<&'static str, LayerCondition> = phf::phf_map! {
    "CONDITION_NOT_CHILD" => LayerCondition::ConditionNotChild,
    "CONDITION_CHILD" => LayerCondition::ConditionChild,
    "CONDITION_HAUL_COUNT_MIN" => LayerCondition::ConditionHaulCountMin,
    "CONDITION_HAUL_COUNT_MAX" => LayerCondition::ConditionHaulCountMax,
};

pub static TILE_PAGE_TAGS: phf::Map<&'static str, TilePageTag> = phf::phf_map! {
    "TILE_DIM" => TilePageTag::TileDim,
    "PAGE_DIM" => TilePageTag::PageDim,
    "FILE" => TilePageTag::File,
};
