//! String token to parsed tag map for plant part tokens.

use crate::tags::PlantPartTag;

/// Mapping of plant part tokens to strings
pub static PLANT_PART_TOKENS: phf::Map<&'static str, PlantPartTag> = phf::phf_map! {
  "TWIGS" => PlantPartTag::Twigs,
  "BRANCHES" => PlantPartTag::Branches,
  "BRANCHES_AND_TWIGS" => PlantPartTag::BranchesAndTwigs,
  "LIGHT_BRANCHES" => PlantPartTag::Branches,
  "LIGHT_BRANCHES_AND_TWIGS" => PlantPartTag::BranchesAndTwigs,
  "ALL_BRANCHES_AND_TWIGS" => PlantPartTag::AllBranchesAndTwigs,
  "HEAVY_BRANCHES" => PlantPartTag::HeavyBranches,
  "DIRECTED_BRANCHES" => PlantPartTag::HeavyBranches,
  "HEAVY_BRANCHES_AND_TRUNK" => PlantPartTag::HeavyBranchesAndTrunk,
  "DIRECTED_BRANCHES_AND_TRUNK" => PlantPartTag::HeavyBranchesAndTrunk,
  "TRUNK" => PlantPartTag::Trunk,
  "ROOTS" => PlantPartTag::Roots,
  "CAP" => PlantPartTag::Cap,
  "SAPLING" => PlantPartTag::Sapling,
};
