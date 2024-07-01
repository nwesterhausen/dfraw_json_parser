use crate::tags::PlantGrowthTypeTag;

/// Mapping of growth type tokens to strings
pub static PLANT_GROWTH_TYPE_TOKENS: phf::Map<&'static str, PlantGrowthTypeTag> = phf::phf_map! {
  "LEAVES" => PlantGrowthTypeTag::Leaves,
  "SPATHES" => PlantGrowthTypeTag::Spathes,
  "FRUIT" => PlantGrowthTypeTag::Fruit,
  "FLOWERS" => PlantGrowthTypeTag::Flowers,
  "NUT" => PlantGrowthTypeTag::Nut,
  "SEED_CATKINS" => PlantGrowthTypeTag::SeedCatkins,
  "POLLEN_CATKINS" => PlantGrowthTypeTag::PollenCatkins,
  "CONE" => PlantGrowthTypeTag::Cone,
  "SEED_CONE" => PlantGrowthTypeTag::SeedCone,
  "POLLEN_CONE" => PlantGrowthTypeTag::PollenCone,
  "FEATHERS" => PlantGrowthTypeTag::Feathers,
  "EGGS" => PlantGrowthTypeTag::Eggs,
  "POD" => PlantGrowthTypeTag::Pod,
  "NONE" => PlantGrowthTypeTag::None,
};
