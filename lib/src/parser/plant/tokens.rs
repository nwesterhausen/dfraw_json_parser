use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum PlantTag {
    Dry,
    Evil,
    Good,
    Savage,
    Wet,
    UndergroundDepth,
    Biome,
    Frequency,
    UseMaterialTemplate,
    BasicMaterial,
    UseMaterial,
    Material,
    PrefString,
    AllNames,
    NameAdjective,
    NamePlural,
    NameSingular,
    #[default]
    Unknown,
}

impl std::fmt::Display for PlantTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PlantTag::Dry => write!(f, "Dry"),
            PlantTag::Evil => write!(f, "Evil"),
            PlantTag::Good => write!(f, "Good"),
            PlantTag::Savage => write!(f, "Savage"),
            PlantTag::Wet => write!(f, "Wet"),
            PlantTag::UndergroundDepth => write!(f, "UndergroundDepth"),
            PlantTag::Biome => write!(f, "Biome"),
            PlantTag::Frequency => write!(f, "Frequency"),
            PlantTag::UseMaterialTemplate => write!(f, "UseMaterialTemplate"),
            PlantTag::BasicMaterial => write!(f, "BasicMaterial"),
            PlantTag::UseMaterial => write!(f, "UseMaterial"),
            PlantTag::Material => write!(f, "Material"),
            PlantTag::PrefString => write!(f, "PrefString"),
            PlantTag::AllNames => write!(f, "AllNames"),
            PlantTag::NameAdjective => write!(f, "NameAdjective"),
            PlantTag::NamePlural => write!(f, "NamePlural"),
            PlantTag::NameSingular => write!(f, "NameSingular"),
            PlantTag::Unknown => write!(f, "Unknown"),
        }
    }
}
