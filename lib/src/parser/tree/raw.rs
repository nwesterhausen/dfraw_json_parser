use serde::{Deserialize, Serialize};
use tracing::{error, warn};

use crate::parser::{serializer_helper, Color, Name};

use super::{
    phf_table::TREE_TOKENS,
    tokens::{TreeToken, TwigPlacement},
};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Tree {
    /// Tree will yield logs made of that material. Instead, if it's `[TREE:NONE]`, no logs will result.
    /// Materials are typically found in other raws..
    material: String,
    /// What the trunk of the tree is named
    #[serde(skip_serializing_if = "Name::is_empty")]
    trunk_name: Name,
    /// The maximum z-level height of the trunk, starting from +2 z-levels above the ground.
    /// Valid values: 1-8
    #[serde(skip_serializing_if = "serializer_helper::is_one_u8")]
    max_trunk_height: u8,
    /// Upper limit of trunk thickness, in tiles. Has a geometric effect on log yield.
    /// Valid values: 1-3
    #[serde(skip_serializing_if = "serializer_helper::is_one_u8")]
    max_trunk_diameter: u8,
    /// The number of years the trunk takes to grow one z-level upward.
    #[serde(skip_serializing_if = "serializer_helper::is_one_u8")]
    trunk_period: u8,
    /// The number of years the trunk takes to grow one tile wider.
    #[serde(skip_serializing_if = "serializer_helper::is_one_u8")]
    trunk_width_period: u8,
    /// What thin branches of the tree are named.
    #[serde(skip_serializing_if = "Name::is_empty")]
    branch_name: Name,
    /// How dense the branches grow on this tree.
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    branch_density: u8,
    /// The radius to which branches can reach. Appears to never reach further than seven tiles from the centre.
    /// Does not depend on the trunk branching amount or where trunks are.
    /// The values used in the game go from 0-3. Higher values than that can cause crashes.
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    branch_radius: u8,
    /// What thick branches of the tree are named.
    #[serde(skip_serializing_if = "Name::is_empty")]
    heavy_branches_name: Name,
    /// Similar to BRANCH_DENSITY for thick branches.
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    heavy_branch_density: u8,
    /// Similar as BRANCH_DENSITY for thick branches. Values outside 0-3 can cause crashes.
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    heavy_branch_radius: u8,
    /// How much the trunk branches out. 0 makes the trunk straight.
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    trunk_branching: u8,
    /// What the roots of the tree are named.
    #[serde(skip_serializing_if = "Name::is_empty")]
    root_name: Name,
    /// Density of the root growth.
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    root_density: u8,
    /// How wide the roots reach out.
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    root_radius: u8,
    /// What the twigs of the tree are named.
    #[serde(skip_serializing_if = "Name::is_empty")]
    twigs_name: Name,
    /// Where twigs appear, defaults to [SideBranches, AboveBranches]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    twigs_placement: Vec<TwigPlacement>,
    /// What this mushroom-cap is called. Only makes sense with TREE_HAS_MUSHROOM_CAP.
    #[serde(skip_serializing_if = "Name::is_empty")]
    cap_name: Name,
    /// Similar to the other PERIOD tags, influences the rate of the mushroom cap growth. Only makes sense with TREE_HAS_MUSHROOM_CAP.
    #[serde(skip_serializing_if = "serializer_helper::is_one_u8")]
    cap_period: u8,
    /// The radius of a mushroom cap. Only makes sense with TREE_HAS_MUSHROOM_CAP.
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    cap_radius: u8,
    /// The tile used for trees of this type on the world map. Defaults to 24 (↑).
    #[serde(skip_serializing_if = "String::is_empty")]
    tree_tile: String,
    /// The tile used for (un)dead trees and deciduous trees (generally in winter) of this type. Defaults to 198 (╞).
    #[serde(skip_serializing_if = "String::is_empty")]
    dead_tree_tile: String,
    /// The tile used for saplings of this tree. Defaults to 231 (τ).
    #[serde(skip_serializing_if = "String::is_empty")]
    sapling_tile: String,
    /// The tile used for dead saplings of this tree. Defaults to 231 (τ).
    #[serde(skip_serializing_if = "String::is_empty")]
    dead_sapling_tile: String,
    /// The color of the tree on the map. Defaults to 2:0:0 (dark green).
    #[serde(skip_serializing_if = "Color::is_default")]
    tree_color: Color,
    /// The color of the tree on the map when (un)dead. Defaults to 0:0:1 (dark gray).
    #[serde(skip_serializing_if = "Color::is_default")]
    dead_tree_color: Color,
    /// The color of saplings of this tree. Defaults to 2:0:0 (dark green).
    #[serde(skip_serializing_if = "Color::is_default")]
    sapling_color: Color,
    /// The color of dead saplings of this tree. Defaults to 0:0:1 (dark gray).
    #[serde(skip_serializing_if = "Color::is_default")]
    dead_sapling_color: Color,
    /// The sapling of this tree will drown once the water on its tile reaches this level. Defaults to 4.
    #[serde(skip_serializing_if = "serializer_helper::is_default_sapling_drown_level")]
    sapling_drown_level: u8,
    /// The water depth at which this tree will drown. Exact behavior is unknown. Defaults to 7.
    #[serde(skip_serializing_if = "serializer_helper::is_default_tree_drown_level")]
    tree_drown_level: u8,
    /// Token tags for the tree.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<TreeToken>,
}

impl Tree {
    pub fn new(material: &str) -> Self {
        Self {
            material: material.to_string(),
            max_trunk_height: 1,
            max_trunk_diameter: 1,
            trunk_period: 1,
            trunk_width_period: 1,
            cap_period: 1,
            sapling_drown_level: 4,
            tree_drown_level: 7,
            twigs_placement: vec![TwigPlacement::SideBranches, TwigPlacement::AboveBranches],
            ..Default::default()
        }
    }
    #[allow(clippy::too_many_lines)]
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(tag) = TREE_TOKENS.get(key) else {
            warn!(
                "TreeParsing: called `Option::unwrap()` on a `None` value for presumed tree tag: {}",
                key
            );
            return;
        };

        if tag == &TreeToken::Tree {
            // Skip because it's the root tag
            return;
        }

        match tag {
            TreeToken::TrunkName => {
                self.trunk_name = Name::from_value(value);
            }
            TreeToken::MaxTrunkHeight => {
                self.max_trunk_height = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("max_trunk_height parsing error\n{:?}", e);
                        return;
                    }
                };
                if self.max_trunk_height > 8 {
                    warn!(
                        "max_trunk_height parsing error: value {} is greater than 8",
                        self.max_trunk_height
                    );
                    self.max_trunk_height = 8;
                }
                if self.max_trunk_height == 0 {
                    warn!(
                        "max_trunk_height parsing error: value {} is 0",
                        self.max_trunk_height
                    );
                    self.max_trunk_height = 1;
                }
            }
            TreeToken::MaxTrunkDiameter => {
                self.max_trunk_diameter = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("max_trunk_diameter parsing error\n{:?}", e);
                        return;
                    }
                };
                if self.max_trunk_diameter > 3 {
                    warn!(
                        "max_trunk_diameter parsing error: value {} is greater than 3",
                        self.max_trunk_diameter
                    );
                    self.max_trunk_diameter = 3;
                }
                if self.max_trunk_diameter == 0 {
                    warn!(
                        "max_trunk_diameter parsing error: value {} is 0",
                        self.max_trunk_diameter
                    );
                    self.max_trunk_diameter = 1;
                }
            }
            TreeToken::TrunkPeriod => {
                self.trunk_period = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("trunk_period parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            TreeToken::TrunkWidthPeriod => {
                self.trunk_width_period = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("trunk_width_period parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            TreeToken::BranchName => {
                self.branch_name = Name::from_value(value);
            }
            TreeToken::BranchDensity => {
                self.branch_density = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("branch_density parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            TreeToken::BranchRadius => {
                self.branch_radius = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("branch_radius parsing error\n{:?}", e);
                        return;
                    }
                };
                if self.branch_radius > 3 {
                    warn!(
                        "branch_radius parsing error: value {} is greater than 3",
                        self.branch_radius
                    );
                    self.branch_radius = 3;
                }
            }
            TreeToken::HeavyBranchesName => {
                self.heavy_branches_name = Name::from_value(value);
            }
            TreeToken::HeavyBranchDensity => {
                self.heavy_branch_density = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("heavy_branch_density parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            TreeToken::HeavyBranchRadius => {
                self.heavy_branch_radius = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("heavy_branch_radius parsing error\n{:?}", e);
                        return;
                    }
                };
                if self.heavy_branch_radius > 3 {
                    warn!(
                        "heavy_branch_radius parsing error: value {} is greater than 3",
                        self.heavy_branch_radius
                    );
                    self.heavy_branch_radius = 3;
                }
            }
            TreeToken::TrunkBranching => {
                self.trunk_branching = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("trunk_branching parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            TreeToken::RootName => {
                self.root_name = Name::from_value(value);
            }
            TreeToken::RootDensity => {
                self.root_density = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("root_density parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            TreeToken::RootRadius => {
                self.root_radius = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("root_radius parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            TreeToken::TwigsName => {
                self.twigs_name = Name::from_value(value);
            }
            TreeToken::TwigsSideBranches => {
                self.twigs_placement.push(TwigPlacement::SideBranches);
            }
            TreeToken::TwigsAboveBranches => {
                self.twigs_placement.push(TwigPlacement::AboveBranches);
            }
            TreeToken::TwigsBelowBranches => {
                self.twigs_placement.push(TwigPlacement::BelowBranches);
            }
            TreeToken::TwigsSideHeavyBranches => {
                self.twigs_placement.push(TwigPlacement::SideHeavyBranches);
            }
            TreeToken::TwigsAboveHeavyBranches => {
                self.twigs_placement.push(TwigPlacement::AboveHeavyBranches);
            }
            TreeToken::TwigsBelowHeavyBranches => {
                self.twigs_placement.push(TwigPlacement::BelowHeavyBranches);
            }
            TreeToken::TwigsSideTrunk => {
                self.twigs_placement.push(TwigPlacement::SideTrunk);
            }
            TreeToken::TwigsAboveTrunk => {
                self.twigs_placement.push(TwigPlacement::AboveTrunk);
            }
            TreeToken::TwigsBelowTrunk => {
                self.twigs_placement.push(TwigPlacement::BelowTrunk);
            }
            TreeToken::CapName => {
                self.cap_name = Name::from_value(value);
            }
            TreeToken::CapPeriod => {
                self.cap_period = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("cap_period parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            TreeToken::CapRadius => {
                self.cap_radius = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("cap_radius parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            TreeToken::TreeTile => {
                self.tree_tile = String::from(value);
            }
            TreeToken::DeadTreeTile => {
                self.dead_tree_tile = String::from(value);
            }
            TreeToken::SaplingTile => {
                self.sapling_tile = String::from(value);
            }
            TreeToken::DeadSaplingTile => {
                self.dead_sapling_tile = String::from(value);
            }
            TreeToken::TreeColor => {
                self.tree_color = Color::from_value(value);
            }
            TreeToken::DeadTreeColor => {
                self.dead_tree_color = Color::from_value(value);
            }
            TreeToken::SaplingColor => {
                self.sapling_color = Color::from_value(value);
            }
            TreeToken::DeadSaplingColor => {
                self.dead_sapling_color = Color::from_value(value);
            }
            TreeToken::SaplingDrownLevel => {
                self.sapling_drown_level = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("sapling_drown_level parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            TreeToken::TreeDrownLevel => {
                self.tree_drown_level = match value.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        error!("tree_drown_level parsing error\n{:?}", e);
                        return;
                    }
                };
            }
            _ => {
                self.tags.push(tag.clone());
            }
        }
    }
}
