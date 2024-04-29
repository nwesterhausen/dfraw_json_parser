use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum GrowthType {
    Leaves,
    Spathes,
    Fruit,
    Flowers,
    Nut,
    SeedCatkins,
    PollenCatkins,
    Cone,
    SeedCone,
    PollenCone,
    Feathers,
    Eggs,
    Pod,
    #[default]
    None,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum GrowthTag {
    Growth,
    GrowthName,
    GrowthItem,
    GrowthHostTile,
    GrowthTrunkHeightPercent,
    GrowthDensity,
    GrowthTiming,
    GrowthPrint,
    GrowthHasSeed,
    GrowthDropsOff,
    GrowthDropsOffNoCloud,
    #[default]
    Unknown,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum PlantPart {
    Twigs,
    Branches,
    BranchesAndTwigs,
    AllBranchesAndTwigs,
    HeavyBranches,
    HeavyBranchesAndTrunk,
    Trunk,
    Roots,
    Cap,
    Sapling,
    #[default]
    Unknown,
}
