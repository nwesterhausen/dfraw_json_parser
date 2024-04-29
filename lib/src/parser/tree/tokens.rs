use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum TreeToken {
    Tree,
    TrunkName,
    MaxTrunkHeight,
    MaxTrunkDiameter,
    TrunkPeriod,
    TrunkWidthPeriod,
    BranchName,
    BranchDensity,
    BranchRadius,
    HeavyBranchesName,
    HeavyBranchDensity,
    HeavyBranchRadius,
    TrunkBranching,
    RootName,
    RootDensity,
    RootRadius,
    TwigsName,
    TwigsSideBranches,
    TwigsAboveBranches,
    TwigsBelowBranches,
    TwigsSideHeavyBranches,
    TwigsAboveHeavyBranches,
    TwigsBelowHeavyBranches,
    TwigsSideTrunk,
    TwigsAboveTrunk,
    TwigsBelowTrunk,
    CapName,
    CapPeriod,
    CapRadius,
    TreeTile,
    DeadTreeTile,
    SaplingTile,
    DeadSaplingTile,
    TreeColor,
    DeadTreeColor,
    SaplingColor,
    DeadSaplingColor,
    SaplingDrownLevel,
    TreeDrownLevel,
    // Actual Tags
    /// The tree has a rounded cap-hood like a giant mushroom. This severely stunts a tree's maximum height (known bug)
    TreeHasMushroomCap,
    /// Uses the standard names for the tree components (roots, trunk, branches, etc.)
    StandardTileNames,
    /// Makes young versions of the tree be called "[tree name] sapling"; otherwise, they are called "young [tree name]".
    Sapling,
    #[default]
    Unknown,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum TwigPlacement {
    SideBranches,
    AboveBranches,
    BelowBranches,
    SideHeavyBranches,
    AboveHeavyBranches,
    BelowHeavyBranches,
    SideTrunk,
    AboveTrunk,
    BelowTrunk,
    #[default]
    Unknown,
}
