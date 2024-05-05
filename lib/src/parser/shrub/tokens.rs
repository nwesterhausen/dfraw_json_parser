use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum ShrubToken {
    Spring,
    Summer,
    Autumn,
    Winter,
    GrowDuration,
    Value,
    PickedTile,
    DeadPickedTile,
    ShrubTile,
    DeadShrubTile,
    ClusterSize,
    PickedColor,
    DeadPickedColor,
    ShrubColor,
    DeadShrubColor,
    ShrubDrownLevel,
    Drink,
    Mill,
    Thread,
    Seed,
    ExtractStillVial,
    ExtractVial,
    ExtractBarrel,
    #[default]
    Unknown,
}



#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum SeasonToken {
    Spring,
    Summer,
    Autumn,
    Winter,
    #[default]
    Unknown,
}
