use super::tokens::{EnvironmentClass, InclusionType, InorganicToken};

/// Map of inorganic tokens to their string representation.
pub static INORGANIC_TOKENS: phf::Map<&'static str, InorganicToken> = phf::phf_map! {
    "WAFERS" => InorganicToken::Wafers,
    "DEEP_SPECIAL" => InorganicToken::DeepSpecial,
    "METAL_ORE" => InorganicToken::MetalOre,
    "THREAD_METAL" => InorganicToken::ThreadMetal,
    "DEEP_SURFACE" => InorganicToken::DeepSurface,
    "AQUIFER" => InorganicToken::Aquifer,
    "METAMORPHIC" => InorganicToken::Metamorphic,
    "SEDIMENTARY" => InorganicToken::Sedimentary,
    "SOIL" => InorganicToken::Soil,
    "SOIL_OCEAN" => InorganicToken::SoilOcean,
    "SOIL_SAND" => InorganicToken::SoilSand,
    "SEDIMENTARY_OCEAN_SHALLOW" => InorganicToken::SedimentaryOceanShallow,
    "SEDIMENTARY_OCEAN_DEEP" => InorganicToken::SedimentaryOceanDeep,
    "IGNEOUS_EXTRUSIVE" => InorganicToken::IgneousExtrusive,
    "IGNEOUS_INTRUSIVE" => InorganicToken::IgneousIntrusive,
    "ENVIRONMENT" => InorganicToken::Environment,
    "ENVIRONMENT_SPEC" => InorganicToken::EnvironmentSpecific,
    "LAVA" => InorganicToken::Lava,
    "SPECIAL" => InorganicToken::Special,
    "GENERATED" => InorganicToken::Generated,
    "DIVINE" => InorganicToken::Divine,
    "SPHERE" => InorganicToken::Sphere,
};

/// Map of environment classes to their string representation.
pub static ENVIRONMENT_CLASS_TOKENS: phf::Map<&'static str, EnvironmentClass> = phf::phf_map! {
    "ALL_STONE" => EnvironmentClass::AllStone,
    "IGNEOUS_ALL" => EnvironmentClass::IgneousAll,
    "IGNEOUS_EXTRUSIVE" => EnvironmentClass::IgneousExtrusive,
    "IGNEOUS_INTRUSIVE" => EnvironmentClass::IgneousIntrusive,
    "SOIL" => EnvironmentClass::Soil,
    "SOIL_SAND" => EnvironmentClass::SoilSand,
    "SOIL_OCEAN" => EnvironmentClass::SoilOcean,
    "SEDIMENTARY" => EnvironmentClass::Sedimentary,
    "METAMORPHIC" => EnvironmentClass::Metamorphic,
    "ALLUVIAL" => EnvironmentClass::Alluvial,
};

/// Map of inclusion types to their string representation.
pub static INCLUSION_TYPE_TOKENS: phf::Map<&'static str, InclusionType> = phf::phf_map! {
    "CLUSTER" => InclusionType::Cluster,
    "CLUSTER_SMALL" => InclusionType::ClusterSmall,
    "CLUSTER_ONE" => InclusionType::ClusterOne,
    "VEIN" => InclusionType::Vein,
};
