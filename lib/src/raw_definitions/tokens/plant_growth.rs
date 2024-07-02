use crate::tags::PlantGrowthTag;

/// Mapping of growth tokens to strings
pub static PLANT_GROWTH_TOKENS: phf::Map<&'static str, PlantGrowthTag> = phf::phf_map! {
  "GROWTH" => PlantGrowthTag::Growth,
  "GROWTH_NAME" => PlantGrowthTag::GrowthName,
  "GROWTH_ITEM" => PlantGrowthTag::GrowthItem,
  "GROWTH_HOST_TILE" => PlantGrowthTag::GrowthHostTile,
  "GROWTH_TRUNK_HEIGHT_PERC" => PlantGrowthTag::GrowthTrunkHeightPercent,
  "GROWTH_DENSITY" => PlantGrowthTag::GrowthDensity,
  "GROWTH_TIMING" => PlantGrowthTag::GrowthTiming,
  "GROWTH_PRINT" => PlantGrowthTag::GrowthPrint,
  "GROWTH_HAS_SEED" => PlantGrowthTag::GrowthHasSeed,
  "GROWTH_DROPS_OFF" => PlantGrowthTag::GrowthDropsOff,
  "GROWTH_DROPS_OFF_NO_CLOUD" => PlantGrowthTag::GrowthDropsOffNoCloud,
};
