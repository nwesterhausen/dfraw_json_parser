use crate::tags::InclusionTypeTag;

/// Map of inclusion types to their string representation.
pub static INCLUSION_TYPE_TOKENS: phf::Map<&'static str, InclusionTypeTag> = phf::phf_map! {
  "CLUSTER" => InclusionTypeTag::Cluster,
  "CLUSTER_SMALL" => InclusionTypeTag::ClusterSmall,
  "CLUSTER_ONE" => InclusionTypeTag::ClusterOne,
  "VEIN" => InclusionTypeTag::Vein,
};
