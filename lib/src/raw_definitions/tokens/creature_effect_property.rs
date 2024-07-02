//! String token to parsed tag map for creature effect property tokens.

use crate::tags::CreatureEffectPropertyTag;

/// A map of creature effect properties to their strings.
pub static CREATURE_EFFECT_PROPERTY_TOKENS: phf::Map<&'static str, CreatureEffectPropertyTag> = phf::phf_map! {
    "SEV" => CreatureEffectPropertyTag::Severity,
    "PROB" => CreatureEffectPropertyTag::Probability,
    "RESISTABLE" => CreatureEffectPropertyTag::Resistible,
    "RESISTIBLE" => CreatureEffectPropertyTag::Resistible,
    "SIZE_DILUTES" => CreatureEffectPropertyTag::SizeDilutes,
    "SIZE_DELAYS" => CreatureEffectPropertyTag::SizeDelays,

    "LOCALIZED" => CreatureEffectPropertyTag::Localized,
    "VASCULAR_ONLY" => CreatureEffectPropertyTag::VascularOnly,
    "MUSCULAR_ONLY" => CreatureEffectPropertyTag::MuscularOnly,

    "BP" => CreatureEffectPropertyTag::BodyPart,
    "BY_CATEGORY" => CreatureEffectPropertyTag::ByCategory,
    "BY_TYPE" => CreatureEffectPropertyTag::ByType,
    "BY_TOKEN" => CreatureEffectPropertyTag::ByToken,

    "START" => CreatureEffectPropertyTag::Start,
    "PEAK" => CreatureEffectPropertyTag::Peak,
    "END" => CreatureEffectPropertyTag::End,

    "DWF_STRETCH" => CreatureEffectPropertyTag::DwfStretch,

    "ABRUPT" => CreatureEffectPropertyTag::Abrupt,
    "ABRUPT_END" => CreatureEffectPropertyTag::AbruptEnd,
    "ABRUPT_START" => CreatureEffectPropertyTag::AbruptStart,

    "CAN_BE_HIDDEN" => CreatureEffectPropertyTag::CanBeHidden,
    "PROBABILITY" => CreatureEffectPropertyTag::Probability,
};
