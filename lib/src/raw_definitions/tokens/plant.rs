//! String token to parsed tag map for plant tokens.

use crate::tags::PlantTag;

/// Mapping of plant tokens to strings
pub static PLANT_TOKENS: phf::Map<&'static str, PlantTag> = phf::phf_map! {
    "NAME" => PlantTag::NameSingular,
    "NAME_PLURAL" => PlantTag::NamePlural,
    "ADJ" => PlantTag::NameAdjective,
    "ALL_NAMES" => PlantTag::AllNames,
    "PREFSTRING" => PlantTag::PrefString,
    "MATERIAL" => PlantTag::Material,
    "USE_MATERIAL" => PlantTag::UseMaterial,
    "BASIC_MAT" => PlantTag::BasicMaterial,
    "USE_MATERIAL_TEMPLATE" => PlantTag::UseMaterialTemplate,
    "UNDERGROUND_DEPTH" => PlantTag::UndergroundDepth,
    "GOOD" => PlantTag::Good,
    "EVIL" => PlantTag::Evil,
    "SAVAGE" => PlantTag::Savage,
    "FREQUENCY" => PlantTag::Frequency,
    "WET" => PlantTag::Wet,
    "DRY" => PlantTag::Dry,
    "BIOME" => PlantTag::Biome,
};
