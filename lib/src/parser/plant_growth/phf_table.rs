use phf::phf_map;

use super::tokens::GrowthTag;
use super::tokens::GrowthType;
use super::tokens::PlantPart;

/// Mapping of growth tokens to strings
pub static GROWTH_TOKENS: phf::Map<&'static str, GrowthTag> = phf_map! {
    "GROWTH" => GrowthTag::Growth,
    "GROWTH_NAME" => GrowthTag::GrowthName,
    "GROWTH_ITEM" => GrowthTag::GrowthItem,
    "GROWTH_HOST_TILE" => GrowthTag::GrowthHostTile,
    "GROWTH_TRUNK_HEIGHT_PERC" => GrowthTag::GrowthTrunkHeightPercent,
    "GROWTH_DENSITY" => GrowthTag::GrowthDensity,
    "GROWTH_TIMING" => GrowthTag::GrowthTiming,
    "GROWTH_PRINT" => GrowthTag::GrowthPrint,
    "GROWTH_HAS_SEED" => GrowthTag::GrowthHasSeed,
    "GROWTH_DROPS_OFF" => GrowthTag::GrowthDropsOff,
    "GROWTH_DROPS_OFF_NO_CLOUD" => GrowthTag::GrowthDropsOffNoCloud,
};

/// Mapping of growth type tokens to strings
pub static GROWTH_TYPE_TOKENS: phf::Map<&'static str, GrowthType> = phf_map! {
    "LEAVES" => GrowthType::Leaves,
    "SPATHES" => GrowthType::Spathes,
    "FRUIT" => GrowthType::Fruit,
    "FLOWERS" => GrowthType::Flowers,
    "NUT" => GrowthType::Nut,
    "SEED_CATKINS" => GrowthType::SeedCatkins,
    "POLLEN_CATKINS" => GrowthType::PollenCatkins,
    "CONE" => GrowthType::Cone,
    "SEED_CONE" => GrowthType::SeedCone,
    "POLLEN_CONE" => GrowthType::PollenCone,
    "FEATHERS" => GrowthType::Feathers,
    "EGGS" => GrowthType::Eggs,
    "POD" => GrowthType::Pod,
    "NONE" => GrowthType::None,
};

/// Mapping of plant part tokens to strings
pub static PLANT_PART_TOKENS: phf::Map<&'static str, PlantPart> = phf_map! {
    "TWIGS" => PlantPart::Twigs,
    "BRANCHES" => PlantPart::Branches,
    "BRANCHES_AND_TWIGS" => PlantPart::BranchesAndTwigs,
    "LIGHT_BRANCHES" => PlantPart::Branches,
    "LIGHT_BRANCHES_AND_TWIGS" => PlantPart::BranchesAndTwigs,
    "ALL_BRANCHES_AND_TWIGS" => PlantPart::AllBranchesAndTwigs,
    "HEAVY_BRANCHES" => PlantPart::HeavyBranches,
    "DIRECTED_BRANCHES" => PlantPart::HeavyBranches,
    "HEAVY_BRANCHES_AND_TRUNK" => PlantPart::HeavyBranchesAndTrunk,
    "DIRECTED_BRANCHES_AND_TRUNK" => PlantPart::HeavyBranchesAndTrunk,
    "TRUNK" => PlantPart::Trunk,
    "ROOTS" => PlantPart::Roots,
    "CAP" => PlantPart::Cap,
    "SAPLING" => PlantPart::Sapling,
};
