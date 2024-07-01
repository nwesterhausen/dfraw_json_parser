use crate::tags::GrowthTag;

/// Map of growth tag tags to their string representation.
pub static GROWTH_TOKENS: phf::Map<&'static str, GrowthTag> = phf::phf_map! {
  "GROWTH_FRUIT" => GrowthTag::Fruit,
  // [GROWTH_1:GRASS_FLOWERS:0:1]
  "GROWTH_1" => GrowthTag::Growth1,
  // [GROWTH_2:GRASS_FLOWERS:1:1]
  "GROWTH_2" => GrowthTag::Growth2,
  // [GROWTH_3:GRASS_FLOWERS:2:1]
  "GROWTH_3" => GrowthTag::Growth3,
  // [GROWTH_4:GRASS_FLOWERS:3:1]
  "GROWTH_4" => GrowthTag::Growth4,
};
