use crate::tags::InorganicTag;

/// Map of inorganic tokens to their string representation.
pub static INORGANIC_TOKENS: phf::Map<&'static str, InorganicTag> = phf::phf_map! {
  "WAFERS" => InorganicTag::Wafers,
  "DEEP_SPECIAL" => InorganicTag::DeepSpecial,
  "METAL_ORE" => InorganicTag::MetalOre,
  "THREAD_METAL" => InorganicTag::ThreadMetal,
  "DEEP_SURFACE" => InorganicTag::DeepSurface,
  "AQUIFER" => InorganicTag::Aquifer,
  "METAMORPHIC" => InorganicTag::Metamorphic,
  "SEDIMENTARY" => InorganicTag::Sedimentary,
  "SOIL" => InorganicTag::Soil,
  "SOIL_OCEAN" => InorganicTag::SoilOcean,
  "SOIL_SAND" => InorganicTag::SoilSand,
  "SEDIMENTARY_OCEAN_SHALLOW" => InorganicTag::SedimentaryOceanShallow,
  "SEDIMENTARY_OCEAN_DEEP" => InorganicTag::SedimentaryOceanDeep,
  "IGNEOUS_EXTRUSIVE" => InorganicTag::IgneousExtrusive,
  "IGNEOUS_INTRUSIVE" => InorganicTag::IgneousIntrusive,
  "ENVIRONMENT" => InorganicTag::Environment,
  "ENVIRONMENT_SPEC" => InorganicTag::EnvironmentSpecific,
  "LAVA" => InorganicTag::Lava,
  "SPECIAL" => InorganicTag::Special,
  "GENERATED" => InorganicTag::Generated,
  "DIVINE" => InorganicTag::Divine,
  "SPHERE" => InorganicTag::Sphere,
};
