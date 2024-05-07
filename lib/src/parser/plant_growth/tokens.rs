use serde::{Deserialize, Serialize};

/// The types of growths
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum GrowthType {
    /// The growth is a leaf
    Leaves,
    /// The growth is a flower cluster
    Spathes,
    /// The growth is a fruit
    Fruit,
    /// The growth is a flower
    Flowers,
    /// The growth is a nut
    Nut,
    /// The growth is a seed catkin
    SeedCatkins,
    /// The growth is a pollen catkin
    PollenCatkins,
    /// The growth is a cone
    Cone,
    /// The growth is a seed cone
    SeedCone,
    /// The growth is a pollen cone
    PollenCone,
    /// The growth is a feather
    Feathers,
    /// The growth is an egg
    Eggs,
    /// The growth is a pod
    Pod,
    /// An unknown growth type
    #[default]
    None,
}

/// The growth tag of a plant
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum GrowthTag {
    /// The beginning of a growth tag
    Growth,
    /// The name of the growth
    GrowthName,
    /// The item from the growth
    GrowthItem,
    /// The host tile the growth grows on
    GrowthHostTile,
    /// The trunk height percent of the growth
    GrowthTrunkHeightPercent,
    /// The growth density
    GrowthDensity,
    /// The growth timing
    GrowthTiming,
    /// The growth print
    GrowthPrint,
    /// If the growth has a seed
    GrowthHasSeed,
    /// If the growth drops off the plant
    GrowthDropsOff,
    /// If the growth drops off the plant and there is no cloud
    GrowthDropsOffNoCloud,
    /// An unknown growth tag
    #[default]
    Unknown,
}

/// Parts of a plant
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum PlantPart {
    /// Twigs
    Twigs,
    /// Branches
    Branches,
    /// Branches and twigs
    BranchesAndTwigs,
    /// All branches and twigs
    AllBranchesAndTwigs,
    /// Heavy branches
    HeavyBranches,
    /// Heavy branches and twigs
    HeavyBranchesAndTrunk,
    /// Trunk
    Trunk,
    /// Roots
    Roots,
    /// Cap (canopy)
    Cap,
    /// Sapling
    Sapling,
    /// An unknown part of the plant
    #[default]
    Unknown,
}
