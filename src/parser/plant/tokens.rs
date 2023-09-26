use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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
    BasicMat,
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
