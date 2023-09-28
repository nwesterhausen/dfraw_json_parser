use super::tokens::{CreatureEffectProperty, CreatureEffectToken};

pub static CREATURE_EFFECT_TOKENS: phf::Map<&'static str, CreatureEffectToken> = phf::phf_map! {
    "CE_PAIN" => CreatureEffectToken::Pain,
    "CE_SWELLING" => CreatureEffectToken::Swelling,
    "CE_OOZING" => CreatureEffectToken::Oozing,
    "CE_BRUISING" => CreatureEffectToken::Bruising,
    "CE_BLISTERS" => CreatureEffectToken::Blisters,
    "CE_NUMBNESS" => CreatureEffectToken::Numbness,
    "CE_PARALYSIS" => CreatureEffectToken::Paralysis,
    "CE_FEVER" => CreatureEffectToken::Fever,
    "CE_BLEEDING" => CreatureEffectToken::Bleeding,
    "CE_COUGHING_BLOOD" => CreatureEffectToken::CoughingBlood,
    "CE_VOMITING_BLOOD" => CreatureEffectToken::VomitingBlood,
    "CE_NAUSEA" => CreatureEffectToken::Nausea,
    "CE_UNCONSCIOUSNESS" => CreatureEffectToken::Unconsciousness,
    "CE_NECROSIS" => CreatureEffectToken::Necrosis,
    "CE_IMPAIR_FUNCTION" => CreatureEffectToken::ImpairFunction,
    "CE_DROWSINESS" => CreatureEffectToken::Drowsiness,
    "CE_DIZZINESS" => CreatureEffectToken::Dizziness,
};

pub static CREATURE_EFFECT_PROPERTY_TOKENS: phf::Map<&'static str, CreatureEffectProperty> = phf::phf_map! {
    "SEV" => CreatureEffectProperty::Severity,
    "PROB" => CreatureEffectProperty::Probability,
    "RESISTABLE" => CreatureEffectProperty::Resistible,
    "RESISTIBLE" => CreatureEffectProperty::Resistible,
    "SIZE_DILUTES" => CreatureEffectProperty::SizeDilutes,

    "LOCALIZED" => CreatureEffectProperty::Localized,
    "VASCULAR_ONLY" => CreatureEffectProperty::VascularOnly,
    "MUSCULAR_ONLY" => CreatureEffectProperty::MuscularOnly,

    "BP" => CreatureEffectProperty::BodyPart,
    "BY_CATEGORY" => CreatureEffectProperty::ByCategory,
    "BY_TYPE" => CreatureEffectProperty::ByType,
    "BY_TOKEN" => CreatureEffectProperty::ByToken,

    "START" => CreatureEffectProperty::Start,
    "PEAK" => CreatureEffectProperty::Peak,
    "END" => CreatureEffectProperty::End,

    "DWF_STRETCH" => CreatureEffectProperty::DwfStretch,
};
