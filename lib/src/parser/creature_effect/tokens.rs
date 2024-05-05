use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum CreatureEffectToken {
    // Negative Effects
    /// Afflicts the targeted body part with intense pain. If no target is specified this applies to all body parts.
    Pain,
    /// Causes the targeted body part to swell up. Extreme swelling may lead to necrosis.
    Swelling,
    /// Causes pus to ooze from the afflicted body part.
    Oozing,
    /// Causes the targeted body part to undergo bruising.
    Bruising,
    /// Covers the targeted body part with blisters.
    Blisters,
    /// Causes numbness in the affected body part, blocking pain. Extreme numbness may lead to sensory nerve damage.
    /// If no target is specified this applies to all body parts.
    Numbness,
    /// Causes complete paralysis of the affected body part. Paralysis on a limb may lead to motor nerve damage.
    /// If no target is specified this causes total paralysis, which can lead to suffocation of smaller creatures.
    Paralysis,
    /// Causes the Fever condition.
    Fever,
    /// Causes the targeted body part to start bleeding, with heavy enough bleeding resulting in the death of the sufferer.
    /// Some conditions seem to cause bleeding to be fatal no matter how weak.
    Bleeding,
    /// This effect results in the sufferer periodically coughing blood, which stains the tile they're on and requires cleanup.
    /// It doesn't appear to be lethal, but may cause minor bleeding damage.
    CoughingBlood,
    /// This effect results in the sufferer periodically vomiting blood, which stains the tile they're on and requires cleanup.
    /// It doesn't appear to be lethal, but may cause minor bleeding damage.
    VomitingBlood,
    /// Causes the Nausea condition, and heavy vomiting. Can eventually lead to dehydration and death.
    Nausea,
    /// Renders the creature unconscious.
    Unconsciousness,
    /// Causes the targeted body part to rot, with associated tissue damage, miasma emission and bleeding.
    /// The victim slowly bleeds to death if the wound is not treated. Badly necrotic limbs will require amputation.
    Necrosis,
    /// An organ afflicted with this effect is rendered inoperable.
    /// E.g., if both lungs are impaired the creature can't breathe and will suffocate. This token only affects organs, not limbs.
    ImpairFunction,
    /// Causes the Drowsiness condition
    Drowsiness,
    /// Inflicts the Dizziness condition, occasional fainting and a general slowdown in movement and work speed.
    Dizziness,
    // Healing Effects
    /// Decreases the severity of pain produced by wounds or syndrome effects on the targeted body part.
    /// The SEV value probably controls by how much the pain is decreased.
    ReducePain,
    /// Decreases the severity of swelling on the targeted body part.
    ReduceSwelling,
    /// Decreases the severity of any paralysis effects on the targeted body part.
    ReduceParalysis,
    /// Decreases the severity of any dizziness the creature has.
    ReduceDizziness,
    /// Decreases the severity of any nausea the creature has.
    ReduceNausea,
    /// Decreases the severity of any fever the creature has.
    ReduceFever,
    /// Decreases the severity of the bleeding of any wounds or syndrome effects on the targeted body part.
    /// The SEV value probably controls by how much the bleeding is decreased.
    StopBleeding,
    /// Closes any wounds on the targeted body part with speed depending on the SEV value.
    CloseOpenWounds,
    /// Probably decreases the severity of the infection from infected wounds over time.
    CureInfection,
    /// Heals the tissues of the targeted body part with speed depending on the SEV value.
    HealTissues,
    /// Heals the nerves of the targeted body part with speed depending on the SEV value.
    HealNerves,
    /// Causes missing body parts to regrow. SEV controls how quickly body parts are regrown.
    RegrowParts,
    // Special Effects
    AddTag,
    RemoveTag,
    DisplayName,
    DisplayTile,
    FlashTile,
    PhysAttChange,
    MentAttChange,
    SpeedChange,
    SkillRollAdjust,
    BodyAppearanceModifier,
    BodyPartAppearanceModifier,
    BodyTransformation,
    MaterialForceMultiplier,
    CanDoInteraction,
    SpecialAttackInteraction,
    BodyMatInteraction,
    SenseCreatureClass,
    FeelEmotion,
    ChangePersonality,
    ErraticBehavior,
    // Unknown
    #[default]
    Unknown,
}



#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type)]
pub enum CreatureEffectProperty {
    /// The severity of the effect. Higher values appear to be worse, with SEV:1000 CE_NECROSIS causing a part to near-instantly become rotten.
    Severity,
    /// The probability of the effect actually manifesting in the victim, as a percentage. 100 means always, 1 means a 1 in 100 chance.
    Probability,
    ///(Optional) Determines if the effect can be hindered by the target creature's disease resistance attribute.
    /// Without this token, disease resistance is ignored. (yes, it's spelled incorrectly)
    Resistible,
    /// (Optional) This token presumably causes the severity of the effect to scale with the size of the creature compared
    /// to the size of the dose of contagion they received, but has yet to be extensively tested.
    SizeDilutes,
    /// (Optional) As above, this token has yet to be tested but presumably delays the onset of an effect according to the size of the victim.
    SizeDelays,
    /// (Optional; overrides BP tokens) This tag causes an effect to ignore all BP tokens and then forces the game to attempt to apply the effect to
    /// the limb that came into contact with the contagion - i.e. the part that was bitten by the creature injecting the syndrome material,
    /// or the one that was splattered by a contact contagion. If an effect can not be applied to the contacted limb (such as IMPAIR_FUNCTION on a non-organ)
    /// then this token makes the effect do nothing. This token also makes inhaled syndromes have no effect.
    Localized,
    /// (Optional) This effect only affects tissue layers with the VASCULAR token.
    VascularOnly,
    /// (Optional) This effect only affects tissue layers with the MUSCULAR token.
    MuscularOnly,
    /// (Optional; overridden by LOCALIZED) Specifies which body parts and tissues the effect is to be applied to. Not every effect requires a target!
    ///  For example, if you wanted to target the lungs of a creature, you would use BP:BY_CATEGORY:LUNG:ALL. The effect would act on all body parts
    /// within the creature with the CATEGORY tag LUNG and affect all tissue layers. For another example, say you wanted to cause the skin to rot off a creature -
    /// you could use BP:BY_CATEGORY:ALL:SKIN, targeting the SKIN tissue on all body parts. Multiple targets can be given in one effect by placing the BP tokens end to end.
    /// This is one of the most powerful and useful aspects of the syndrome system, as it allows you to selectively target body parts relevant to the contagion,
    /// like lungs for coal dust inhalation, or the eyes for exposure to an acid gas.
    BodyPart,
    /// BY_CATEGORY:X to target body parts with a matching \[CATEGORY:X\] body token (or ALL to affect everything)
    ByCategory,
    /// BY_TYPE:X to target body parts having a particular type (UPPERBODY, LOWERBODY, HEAD, GRASP, or STANCE)
    ByType,
    /// BY_TOKEN:X to target individual body parts by their ID as specified by the \[BP\] token of the body plan definition.
    ByToken,
    /// Determines the time after exposure, in ticks, when the effect starts. Required for all effects.
    Start,
    /// (Optional) Determines the time after exposure, in ticks, when the effect reaches its peak intensity.
    Peak,
    /// (Optional) Determines the time after exposure, in ticks, when the effect ends.
    End,
    /// (Optional) Multiplies the duration values of the effect by the specified amount in Fortress mode.
    DwfStretch,
    /// (Optional) Makes the effect begin immediately rather than ramping up.
    AbruptStart,
    /// (Optional) Makes the effect end immediately rather than ramping down.
    AbruptEnd,
    /// (Optional) Combination of ABRUPT_START and ABRUPT_END.
    Abrupt,
    /// (Optional) Can be hidden by a unit assuming a secret identity, such as a vampire.
    CanBeHidden,
    /// Unknown value for default.
    #[default]
    Unknown,
}

//Todo: add triggers
