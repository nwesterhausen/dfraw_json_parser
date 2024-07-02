//! String token to parsed tag map for shrub tokens.

use crate::tags::ShrubTag;

/// The mapping of shrub tokens to their string representation
pub static SHRUB_TOKENS: phf::Map<&'static str, ShrubTag> = phf::phf_map! {
  "SPRING" => ShrubTag::Spring,
  "SUMMER" => ShrubTag::Summer,
  "AUTUMN" => ShrubTag::Autumn,
  "WINTER" => ShrubTag::Winter,
  "GROWDUR" => ShrubTag::GrowDuration,
  "VALUE" => ShrubTag::Value,
  "PICKED_TILE" => ShrubTag::PickedTile,
  "DEAD_PICKED_TILE" => ShrubTag::DeadPickedTile,
  "SHRUB_TILE" => ShrubTag::ShrubTile,
  "DEAD_SHRUB_TILE" => ShrubTag::DeadShrubTile,
  "CLUSTER_SIZE" => ShrubTag::ClusterSize,
  "PICKED_COLOR" => ShrubTag::PickedColor,
  "DEAD_PICKED_COLOR" => ShrubTag::DeadPickedColor,
  "SHRUB_COLOR" => ShrubTag::ShrubColor,
  "DEAD_SHRUB_COLOR" => ShrubTag::DeadShrubColor,
  "SHRUB_DROWN_LEVEL" => ShrubTag::ShrubDrownLevel,
  "DRINK" => ShrubTag::Drink,
  "MILL" => ShrubTag::Mill,
  "THREAD" => ShrubTag::Thread,
  "SEED" => ShrubTag::Seed,
  "EXTRACT_STILL_VIAL" => ShrubTag::ExtractStillVial,
  "EXTRACT_VIAL" => ShrubTag::ExtractVial,
  "EXTRACT_BARREL" => ShrubTag::ExtractBarrel,
};
