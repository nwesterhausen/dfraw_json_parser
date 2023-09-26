use super::tokens::CVTag;

pub static CV_TOKENS: phf::Map<&'static str, CVTag> = phf::phf_map! {
    "CV_NEW_TAG" => CVTag::NewTag,
    "CV_ADD_TAG" => CVTag::AddTag,
    "CV_REMOVE_TAG" => CVTag::RemoveTag,
    "CV_CONVERT_TAG" => CVTag::ConvertTag,
    "CVCT_MASTER" => CVTag::ConvertTagMaster,
    "CVCT_TARGET" => CVTag::ConvertTagTarget,
    "CVCT_REPLACEMENT" => CVTag::ConvertTagReplacement,
    "CV_NEW_CTAG" => CVTag::ConditionalNewTag,
    "CV_ADD_CTAG" => CVTag::ConditionalAddTag,
    "CV_REMOVE_CTAG" => CVTag::ConditionalRemoveTag,
    "CV_CONVERT_CTAG" => CVTag::ConditionalConvertTag,
};
