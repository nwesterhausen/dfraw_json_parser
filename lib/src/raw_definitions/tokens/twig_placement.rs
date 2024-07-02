//! String token to parsed tag map for twig placement tokens.

use crate::tags::TwigPlacementTag;

/// The mapping of tree tokens to their string representation
pub static TWIG_PLACEMENT_TOKENS: phf::Map<&'static str, TwigPlacementTag> = phf::phf_map! {
  "SIDE_BRANCHES" => TwigPlacementTag::SideBranches,
  "ABOVE_BRANCHES" => TwigPlacementTag::AboveBranches,
  "BELOW_BRANCHES" => TwigPlacementTag::BelowBranches,
  "SIDE_HEAVY_BRANCHES" => TwigPlacementTag::SideHeavyBranches,
  "ABOVE_HEAVY_BRANCHES" => TwigPlacementTag::AboveHeavyBranches,
  "BELOW_HEAVY_BRANCHES" => TwigPlacementTag::BelowHeavyBranches,
  "SIDE_TRUNK" => TwigPlacementTag::SideTrunk,
  "ABOVE_TRUNK" => TwigPlacementTag::AboveTrunk,
  "BELOW_TRUNK" => TwigPlacementTag::BelowTrunk,
};
