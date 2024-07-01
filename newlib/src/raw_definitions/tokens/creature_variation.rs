use crate::tags::CreatureVariationTag;

/// A map of all the creature variation tags to strings
pub static CREATURE_VARIATION_TOKENS: phf::Map<&'static str, CreatureVariationTag> = phf::phf_map! {
    "CV_NEW_TAG" => CreatureVariationTag::NewTag,
    "CV_ADD_TAG" => CreatureVariationTag::AddTag,
    "CV_REMOVE_TAG" => CreatureVariationTag::RemoveTag,
    "CV_CONVERT_TAG" => CreatureVariationTag::ConvertTag,
    "CVCT_MASTER" => CreatureVariationTag::ConvertTagMaster,
    "CVCT_TARGET" => CreatureVariationTag::ConvertTagTarget,
    "CVCT_REPLACEMENT" => CreatureVariationTag::ConvertTagReplacement,
    "CV_NEW_CTAG" => CreatureVariationTag::ConditionalNewTag,
    "CV_ADD_CTAG" => CreatureVariationTag::ConditionalAddTag,
    "CV_REMOVE_CTAG" => CreatureVariationTag::ConditionalRemoveTag,
    "CV_CONVERT_CTAG" => CreatureVariationTag::ConditionalConvertTag,
};
