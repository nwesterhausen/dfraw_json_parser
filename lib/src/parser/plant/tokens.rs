use serde::{Deserialize, Serialize};

/// The tags of a plant
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum PlantTag {
    /// The plant grows in dry conditions
    Dry,
    /// The plant grows in evil conditions
    Evil,
    /// The plant grows in good conditions
    Good,
    /// The plant grows in savage conditions
    Savage,
    /// The plant grows in wet conditions
    Wet,
    /// The plant grows at a specific underground depth
    UndergroundDepth,
    /// The plant grows in a specific biome
    Biome,
    /// The frequency of the plant
    Frequency,
    /// The material template to use for the plant
    UseMaterialTemplate,
    /// The basic material to use for the plant
    BasicMaterial,
    /// The material to use for the plant
    UseMaterial,
    /// The material of the plant
    Material,
    /// What dwarves prefer about the plant
    PrefString,
    /// All names of the plant
    AllNames,
    /// The adjective name of the plant
    NameAdjective,
    /// The plural name of the plant
    NamePlural,
    /// The singular name of the plant
    NameSingular,
    /// An unknown plant tag
    #[default]
    Unknown,
}

impl std::fmt::Display for PlantTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Dry => write!(f, "Dry"),
            Self::Evil => write!(f, "Evil"),
            Self::Good => write!(f, "Good"),
            Self::Savage => write!(f, "Savage"),
            Self::Wet => write!(f, "Wet"),
            Self::UndergroundDepth => write!(f, "UndergroundDepth"),
            Self::Biome => write!(f, "Biome"),
            Self::Frequency => write!(f, "Frequency"),
            Self::UseMaterialTemplate => write!(f, "UseMaterialTemplate"),
            Self::BasicMaterial => write!(f, "BasicMaterial"),
            Self::UseMaterial => write!(f, "UseMaterial"),
            Self::Material => write!(f, "Material"),
            Self::PrefString => write!(f, "PrefString"),
            Self::AllNames => write!(f, "AllNames"),
            Self::NameAdjective => write!(f, "NameAdjective"),
            Self::NamePlural => write!(f, "NamePlural"),
            Self::NameSingular => write!(f, "NameSingular"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
