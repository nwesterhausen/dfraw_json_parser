//! String token to parsed tag map for material property tokens.

use crate::tags::MaterialPropertyTag;

/// Mapping of material property tokens to strings
pub static MATERIAL_PROPERTY_TOKENS: phf::Map<&'static str, MaterialPropertyTag> = phf::phf_map! {
  "USE_MATERIAL_TEMPLATE" => MaterialPropertyTag::UseMaterialTemplate,
  "PREFIX" => MaterialPropertyTag::Prefix,
  "STONE_NAME" => MaterialPropertyTag::StoneName,
  "IS_GEM" => MaterialPropertyTag::IsGem,
  "TEMP_DIET_INFO" => MaterialPropertyTag::TempDietInfo,
  "POWDER_DYE" => MaterialPropertyTag::PowderDye,
  "TILE" => MaterialPropertyTag::Tile,
  "ITEM_SYMBOL" => MaterialPropertyTag::ItemSymbol,
  "DISPLAY_COLOR" => MaterialPropertyTag::DisplayColor,
  "BUILD_COLOR" => MaterialPropertyTag::BuildColor,
  "TILE_COLOR" => MaterialPropertyTag::TileColor,
  "BASIC_COLOR" => MaterialPropertyTag::BasicColor,
  "STATE_COLOR" => MaterialPropertyTag::StateColor,
  "STATE_NAME" => MaterialPropertyTag::StateName,
  "STATE_ADJ" => MaterialPropertyTag::StateAdjective,
  "STATE_NAME_ADJ" => MaterialPropertyTag::StateNameAdjective,
  "ABSORPTION" => MaterialPropertyTag::Absorption,
  "IMPACT_YIELD" => MaterialPropertyTag::ImpactYield,
  "IMPACT_FRACTURE" => MaterialPropertyTag::ImpactFracture,
  "IMPACT_STRAIN_AT_YIELD" => MaterialPropertyTag::ImpactElasticity,
  "IMPACT_ELASTICITY" => MaterialPropertyTag::ImpactElasticity,
  "COMPRESSIVE_YIELD" => MaterialPropertyTag::CompressiveYield,
  "COMPRESSIVE_FRACTURE" => MaterialPropertyTag::CompressiveFracture,
  "COMPRESSIVE_STRAIN_AT_YIELD" => MaterialPropertyTag::CompressiveElasticity,
  "COMPRESSIVE_ELASTICITY" => MaterialPropertyTag::CompressiveElasticity,
  "TENSILE_YIELD" => MaterialPropertyTag::TensileYield,
  "TENSILE_FRACTURE" => MaterialPropertyTag::TensileFracture,
  "TENSILE_STRAIN_AT_YIELD" => MaterialPropertyTag::TensileElasticity,
  "TENSILE_ELASTICITY" => MaterialPropertyTag::TensileElasticity,
  "TORSION_YIELD" => MaterialPropertyTag::TorsionYield,
  "TORSION_FRACTURE" => MaterialPropertyTag::TorsionFracture,
  "TORSION_STRAIN_AT_YIELD" => MaterialPropertyTag::TorsionElasticity,
  "TORSION_ELASTICITY" => MaterialPropertyTag::TorsionElasticity,
  "SHEAR_YIELD" => MaterialPropertyTag::ShearYield,
  "SHEAR_FRACTURE" => MaterialPropertyTag::ShearFracture,
  "SHEAR_STRAIN_AT_YIELD" => MaterialPropertyTag::ShearElasticity,
  "SHEAR_ELASTICITY" => MaterialPropertyTag::ShearElasticity,
  "BENDING_YIELD" => MaterialPropertyTag::BendingYield,
  "BENDING_FRACTURE" => MaterialPropertyTag::BendingFracture,
  "BENDING_STRAIN_AT_YIELD" => MaterialPropertyTag::BendingElasticity,
  "BENDING_ELASTICITY" => MaterialPropertyTag::BendingElasticity,
  "MAX_EDGE" => MaterialPropertyTag::MaxEdge,
  "MATERIAL_VALUE" => MaterialPropertyTag::MaterialValue,
  "MULTIPLY_VALUE" => MaterialPropertyTag::MultiplyValue,
  "SPEC_HEAT" => MaterialPropertyTag::SpecificHeat,
  "HEATDAM_POINT" => MaterialPropertyTag::HeatDamagePoint,
  "COLDDAM_POINT" => MaterialPropertyTag::ColdDamagePoint,
  "IGNITE_POINT" => MaterialPropertyTag::IgnitionPoint,
  "MELTING_POINT" => MaterialPropertyTag::MeltingPoint,
  "BOILING_POINT" => MaterialPropertyTag::BoilingPoint,
  "MAT_FIXED_TEMP" => MaterialPropertyTag::MaterialFixedTemperature,
  "IF_EXISTS_SET_HEATDAM_POINT" => MaterialPropertyTag::IfExistsSetHeatDamagePoint,
  "IF_EXISTS_SET_COLDDAM_POINT" => MaterialPropertyTag::IfExistsSetColdDamagePoint,
  "IF_EXISTS_SET_IGNITE_POINT" => MaterialPropertyTag::IfExistsSetIgnitePoint,
  "IF_EXISTS_SET_MELTING_POINT" => MaterialPropertyTag::IfExistsSetMeltingPoint,
  "IF_EXISTS_SET_BOILING_POINT" => MaterialPropertyTag::IfExistsSetBoilingPoint,
  "IF_EXISTS_SET_MAT_FIXED_TEMP" => MaterialPropertyTag::IfExistsSetMatFixedTemp,
  "SOLID_DENSITY" => MaterialPropertyTag::SolidDensity,
  "LIQUID_DENSITY" => MaterialPropertyTag::LiquidDensity,
  "MOLAR_MASS" => MaterialPropertyTag::MolarMass,
  "EXTRACT_STORAGE" => MaterialPropertyTag::ExtractStorage,
  "BUTCHER_SPECIAL" => MaterialPropertyTag::ButcherSpecial,
  "MEAT_NAME" => MaterialPropertyTag::MeatName,
  "BLOCK_NAME" => MaterialPropertyTag::BlockName,
  "WAFERS" => MaterialPropertyTag::Wafers,
  "MATERIAL_REACTION_PRODUCT" => MaterialPropertyTag::MaterialReactionProduct,
  "ITEM_REACTION_PRODUCT" => MaterialPropertyTag::ItemReactionProduct,
  "REACTION_CLASS" => MaterialPropertyTag::ReactionClass,
  "METAL_ORE" => MaterialPropertyTag::MetalOre,
  "THREAD_METAL" => MaterialPropertyTag::ThreadMetal,
  "HARDENS_WITH_WATER" => MaterialPropertyTag::HardensWithWater,
  "SOAP_LEVEL" => MaterialPropertyTag::SoapLevel,
  "SYNDROME" => MaterialPropertyTag::Syndrome,
  "ANTLER" => MaterialPropertyTag::Antler,
  "CARTILAGE" => MaterialPropertyTag::Cartilage,
  "FEATHER" => MaterialPropertyTag::Feather,
  "HOOF" => MaterialPropertyTag::Hoof,
  "CHITIN" => MaterialPropertyTag::Chitin,
  "SCALE" => MaterialPropertyTag::Scale,
  "NERVOUS_TISSUE" => MaterialPropertyTag::NervousTissue,
  "MEAT_CATEGORY" => MaterialPropertyTag::MeatCategory,
};
