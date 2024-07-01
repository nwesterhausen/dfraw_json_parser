use crate::tags::MaterialTypeTag;

/// Mapping of material type tokens to strings
pub static MATERIAL_TYPE_TOKENS: phf::Map<&'static str, MaterialTypeTag> = phf::phf_map! {
  "INORGANIC" => MaterialTypeTag::Inorganic,
  "STONE" => MaterialTypeTag::Stone,
  "METAL" => MaterialTypeTag::Metal,
  "COAL" => MaterialTypeTag::Coal,
  "CREATURE_MAT" => MaterialTypeTag::CreatureMaterial,
  "LOCAL_CREATURE_MAT" => MaterialTypeTag::LocalCreatureMaterial,
  "PLANT_MAT" => MaterialTypeTag::PlantMaterial,
  "LOCAL_PLANT_MAT" => MaterialTypeTag::LocalPlantMaterial,
  "GET_MATERIAL_FROM_REAGENT" => MaterialTypeTag::GetMaterialFromReagent,

  // Special "Hardcoded" Materials
  "AMBER" => MaterialTypeTag::Amber,
  "CORAL" => MaterialTypeTag::Coral,
  "GLASS_GREEN" => MaterialTypeTag::GlassGreen,
  "GLASS_CLEAR" => MaterialTypeTag::GlassClear,
  "GLASS_CRYSTAL" => MaterialTypeTag::GlassCrystal,
  "WATER" => MaterialTypeTag::Water,
  "POTASH" => MaterialTypeTag::Potash,
  "ASH" => MaterialTypeTag::Ash,
  "PEARLASH" => MaterialTypeTag::PearlAsh,
  "LYE" => MaterialTypeTag::Lye,
  "MUD" => MaterialTypeTag::Mud,
  "VOMIT" => MaterialTypeTag::Vomit,
  "SALT" => MaterialTypeTag::Salt,
  "FILTH_B" => MaterialTypeTag::FilthB,
  "FILTH_Y" => MaterialTypeTag::FilthY,
  "UNKNOWN_SUBSTANCE" => MaterialTypeTag::UnknownSubstance,
  "GRIME" => MaterialTypeTag::Grime,
};
