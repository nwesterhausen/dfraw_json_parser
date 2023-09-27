use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum GrowthTag {
    GrowthHasSeed,
    GrowthDropsOff,
    GrowthDropsOffNoCloud,
    #[default]
    Unknown,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum PlantPart {
    Twigs,
    Branches,
    BranchesAndTwigs,
    LightBranches,
    LightBranchesAndTwigs,
    AllBranchesAndTwigs,
    HeavyBranches,
    HeavyBranchesAndTrunk,
    DirectedBranches,
    DirectedBranchesAndTrunk,
    Trunk,
    Roots,
    Cap,
    Sapling,
    #[default]
    Unknown,
}
