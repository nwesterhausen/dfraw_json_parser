use super::tokens::{Condition, GraphicType, GrowthTag, PlantGraphicTemplate, TilePageTag};

pub static CONDITION_TAGS: phf::Map<&'static str, Condition> = phf::phf_map! {
    "DEFAULT" => Condition::Default,
    "ANIMATED" => Condition::Animated,
    "CORPSE" => Condition::Corpse,
    "CHILD" => Condition::Child,
    "BABY" => Condition::Baby,
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
    "REMAINS" => Condition::Remains,
    "VERMIN" => Condition::Vermin,
    "LIGHT_VERMIN" => Condition::LightVermin,
    "HIVE" => Condition::Hive,
    "SWARM_SMALL" => Condition::SwarmSmall,
    "SWARM_MEDIUM" => Condition::SwarmMedium,
    "SWARM_LARGE" => Condition::SwarmLarge,

    "NOT_ARTIFACT" => Condition::NotArtifact,
    "IS_CRAFTED_ARTIFACT" => Condition::CraftedArtifact,

    "SHRUB" => Condition::Shrub,
    "PICKED" => Condition::Picked,
    "SEED" => Condition::Seed,
    "CROP" => Condition::Crop,
    "CROP_SPROUT" => Condition::CropSprout,
    "CROP_L" => Condition::CropL,
    "CROP_M" => Condition::CropM,
    "CROP_R" => Condition::CropR,
    "SHRUB_DEAD" => Condition::ShrubDead,
    "SAPLING" => Condition::Sapling,

    "CONDITION_NOT_CHILD" => Condition::NotChild,
    "CONDITION_CHILD" => Condition::Child,
    "CONDITION_BABY" => Condition::Baby,
    "CONDITION_HAUL_COUNT_MIN" => Condition::HaulCountMin,
    "CONDITION_HAUL_COUNT_MAX" => Condition::HaulCountMax,
    "CONDITION_ITEM_WORN" => Condition::ItemWorn,
    "CONDITION_PROFESSION_CATEGORY" => Condition::ProfessionCategory,
    "CONDITION_SYN_CLASS" => Condition::SyndromeClass,
    "CONDITION_CASTE" => Condition::Caste,
    "CONDITION_TISSUE_LAYER" => Condition::TissueLayer,
    "CONDITION_MATERIAL_FLAG" => Condition::MaterialFlag,
    "CONDITION_MATERIAL_TYPE" => Condition::MaterialType,
    "CONDITION_DYE" => Condition::Dye,
    "CONDITION_NOT_DYED" => Condition::NotDyed,
    "SHUT_OFF_IF_ITEM_PRESENT" => Condition::ShutOffIfItemPresent,
    "CONDITION_RANDOM_PART_INDEX" => Condition::RandomPartIndex,
    "CONDITION_GHOST" => Condition::Ghost,
    "TISSUE_MAY_HAVE_COLOR" => Condition::TissueMayHaveColor,
    "TISSUE_MIN_LENGTH" => Condition::TissueMinLength,
    "TISSUE_MAX_LENGTH" => Condition::TissueMaxLength,
    "TISSUE_MAY_HAVE_SHAPING" => Condition::TissueMayHaveShaping,
    "TISSUE_NOT_SHAPED" => Condition::TissueNotShaped,
    "TISSUE_SWAP" => Condition::TissueSwap,
};

pub static GRAPHIC_TYPE_TAGS: phf::Map<&'static str, GraphicType> = phf::phf_map! {
    "CREATURE_GRAPHICS" => GraphicType::Creature,
    "CREATURE_CASTE_GRAPHICS" => GraphicType::CreatureCaste,
    "TILE_GRAPHICS" => GraphicType::Tile,
    "PLANT_GRAPHICS" => GraphicType::Plant,
    "SHRUB" => GraphicType::Plant,
    "SHRUB_DEAD" => GraphicType::Plant,
    "PICKED" => GraphicType::Plant,
    "SEED" => GraphicType::Plant,
    "SAPLING" => GraphicType::Plant,
    "CROP" => GraphicType::Plant,
    "CROP_SPROUT" => GraphicType::Plant,
    "CROP_L" => GraphicType::Plant,
    "CROP_M" => GraphicType::Plant,
    "CROP_R" => GraphicType::Plant,
};

pub static GROWTH_TAGS: phf::Map<&'static str, GrowthTag> = phf::phf_map! {
    "GROWTH_FRUIT" => GrowthTag::Fruit,
};

pub static TILE_PAGE_TAGS: phf::Map<&'static str, TilePageTag> = phf::phf_map! {
    "TILE_DIM" => TilePageTag::TileDim,
    "PAGE_DIM_PIXELS" => TilePageTag::PageDim,
    "PAGE_DIM" => TilePageTag::PageDim,
    "FILE" => TilePageTag::File,
};

pub static PLANT_GRAPHIC_TEMPLATES: phf::Map<&'static str, PlantGraphicTemplate> = phf::phf_map! {
    "STANDARD_LEAVES" => PlantGraphicTemplate::StandardLeaves,
    "STANDARD_FLOWERS_1" => PlantGraphicTemplate::StandardFlowers1,
    "STANDARD_FRUIT_1" => PlantGraphicTemplate::StandardFruit1,
    "STANDARD_FLOWERS_2" => PlantGraphicTemplate::StandardFlowers2,
    "STANDARD_FRUIT_2" => PlantGraphicTemplate::StandardFruit2,
    "STANDARD_FLOWERS_3" => PlantGraphicTemplate::StandardFlowers3,
    "STANDARD_FRUIT_3" => PlantGraphicTemplate::StandardFruit3,
    "STANDARD_FLOWERS_4" => PlantGraphicTemplate::StandardFlowers4,
    "STANDARD_FRUIT_4" => PlantGraphicTemplate::StandardFruit4,
};
