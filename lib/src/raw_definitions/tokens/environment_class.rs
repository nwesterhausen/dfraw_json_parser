//! String token to parsed tag map for environment class tokens.

use crate::tags::EnvironmentClassTag;

/// Map of environment classes to their string representation.
pub static ENVIRONMENT_CLASS_TOKENS: phf::Map<&'static str, EnvironmentClassTag> = phf::phf_map! {
  "ALL_STONE" => EnvironmentClassTag::AllStone,
  "IGNEOUS_ALL" => EnvironmentClassTag::IgneousAll,
  "IGNEOUS_EXTRUSIVE" => EnvironmentClassTag::IgneousExtrusive,
  "IGNEOUS_INTRUSIVE" => EnvironmentClassTag::IgneousIntrusive,
  "SOIL" => EnvironmentClassTag::Soil,
  "SOIL_SAND" => EnvironmentClassTag::SoilSand,
  "SOIL_OCEAN" => EnvironmentClassTag::SoilOcean,
  "SEDIMENTARY" => EnvironmentClassTag::Sedimentary,
  "METAMORPHIC" => EnvironmentClassTag::Metamorphic,
  "ALLUVIAL" => EnvironmentClassTag::Alluvial,
};
