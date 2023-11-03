use super::tokens::{CreatureEffectProperty, CreatureEffectToken};

/// This is a PHF map of all the creature effect tokens that are used in the raws.
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
    "CE_VOMIT_BLOOD" => CreatureEffectToken::VomitingBlood,
    "CE_NAUSEA" => CreatureEffectToken::Nausea,
    "CE_UNCONSCIOUSNESS" => CreatureEffectToken::Unconsciousness,
    "CE_NECROSIS" => CreatureEffectToken::Necrosis,
    "CE_IMPAIR_FUNCTION" => CreatureEffectToken::ImpairFunction,
    "CE_DROWSINESS" => CreatureEffectToken::Drowsiness,
    "CE_DIZZINESS" => CreatureEffectToken::Dizziness,

    "CE_REDUCE_PAIN" => CreatureEffectToken::ReducePain,
    "CE_REDUCE_SWELLING" => CreatureEffectToken::ReduceSwelling,
    "CE_REDUCE_PARALYSIS" => CreatureEffectToken::ReduceParalysis,
    "CE_REDUCE_DIZZINESS" => CreatureEffectToken::ReduceDizziness,
    "CE_REDUCE_NAUSEA" => CreatureEffectToken::ReduceNausea,
    "CE_REDUCE_FEVER" => CreatureEffectToken::ReduceFever,
    "CE_STOP_BLEEDING" => CreatureEffectToken::StopBleeding,
    "CE_CLOSE_OPEN_WOUNDS" => CreatureEffectToken::CloseOpenWounds,
    "CE_CURE_INFECTION" => CreatureEffectToken::CureInfection,
    "CE_HEAL_TISSUES" => CreatureEffectToken::HealTissues,
    "CE_HEAL_NERVES" => CreatureEffectToken::HealNerves,
    "CE_REGROW_PARTS" => CreatureEffectToken::RegrowParts,

    "CE_ADD_TAG" => CreatureEffectToken::AddTag,
    "CE_REMOVE_TAG" => CreatureEffectToken::RemoveTag,
    "CE_DISPLAY_NAME" => CreatureEffectToken::DisplayName,
    "CE_DISPLAY_TILE" => CreatureEffectToken::DisplayTile,
    "CE_FLASH_TILE" => CreatureEffectToken::FlashTile,
    "CE_PHYS_ATT_CHANGE" => CreatureEffectToken::PhysAttChange,
    "CE_MENT_ATT_CHANGE" => CreatureEffectToken::MentAttChange,
    "CE_SPEED_CHANGE" => CreatureEffectToken::SpeedChange,
    "CE_SKILL_ROLL_ADJUST" => CreatureEffectToken::SkillRollAdjust,
    "CE_BODY_APPEARANCE_MODIFIER" => CreatureEffectToken::BodyAppearanceModifier,
    "CE_BODYPART_APPEARANCE_MODIFIER" => CreatureEffectToken::BodyPartAppearanceModifier,
    "CE_BODY_TRANSFORMATION" => CreatureEffectToken::BodyTransformation,
    "CE_MATERIAL_FORCE_MULTIPLIER" => CreatureEffectToken::MaterialForceMultiplier,
    "CE_CAN_DO_INTERACTION" => CreatureEffectToken::CanDoInteraction,
    "CE_SPECIAL_ATTACK_INTERACTION" => CreatureEffectToken::SpecialAttackInteraction,
    "CE_BODY_MAT_INTERACTION" => CreatureEffectToken::BodyMatInteraction,
    "CE_SENSE_CREATURE_CLASS" => CreatureEffectToken::SenseCreatureClass,
    "CE_FEEL_EMOTION" => CreatureEffectToken::FeelEmotion,
    "CE_CHANGE_PERSONALITY" => CreatureEffectToken::ChangePersonality,
    "CE_ERRATIC_BEHAVIOR" => CreatureEffectToken::ErraticBehavior,
};

/// This is a PHF map of all the creature effect property tokens that are used in the raws.
pub static CREATURE_EFFECT_PROPERTY_TOKENS: phf::Map<&'static str, CreatureEffectProperty> = phf::phf_map! {
    "SEV" => CreatureEffectProperty::Severity,
    "PROB" => CreatureEffectProperty::Probability,
    "RESISTABLE" => CreatureEffectProperty::Resistible,
    "RESISTIBLE" => CreatureEffectProperty::Resistible,
    "SIZE_DILUTES" => CreatureEffectProperty::SizeDilutes,
    "SIZE_DELAYS" => CreatureEffectProperty::SizeDelays,

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

    "ABRUPT" => CreatureEffectProperty::Abrupt,
    "ABRUPT_END" => CreatureEffectProperty::AbruptEnd,
    "ABRUPT_START" => CreatureEffectProperty::AbruptStart,

    "CAN_BE_HIDDEN" => CreatureEffectProperty::CanBeHidden,
    "PROBABILITY" => CreatureEffectProperty::Probability,
};
