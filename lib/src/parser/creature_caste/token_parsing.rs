use crate::parser::metadata::{RawObjectToken, TokenComplexity};

use super::{Token as CasteTag, TOKEN_MAP};

#[typetag::serde]
impl RawObjectToken for CasteTag {
    #[allow(clippy::too_many_lines)]
    fn get_complexity(&self) -> TokenComplexity {
        match self {
          CasteTag::ApplyCreatureVariation { .. } |
          CasteTag::Attack { .. } |
          CasteTag::AttackTrigger { .. } |
          CasteTag::BabyName {.. } |
          CasteTag::Body { .. } |
          CasteTag::Blood {..}|
          CasteTag::BodyAppearanceModifier { .. }|
          CasteTag::BodyDetailPlan { .. } |
          CasteTag::BodyPartAppearanceModifier {..}|
            CasteTag::ChildName { .. } |
            CasteTag::ClutchSize { .. } |
            CasteTag::Color { .. } |
            CasteTag::ExtraButcherObjectItem { .. } |
            CasteTag::ExtraButcherObject { .. } |
            CasteTag::GeneralMaterialForceMultiplier { .. } |
            CasteTag::GlowColor { .. } |
            CasteTag::GobbleVerminCreature { .. } |
            CasteTag::InteractionDetail { .. } |
            CasteTag::Lair { .. } |
            CasteTag::LaysUnusualEggs {..}|
            CasteTag::Ligaments { .. } |
            CasteTag::LitterSize { .. } |
            CasteTag::MannerismFingers { .. } |
            CasteTag::MaxAge { .. } |
            CasteTag::MentalAttributeCapPercentage { .. } |
            CasteTag::MentalAttributeRange { .. } |
            CasteTag::MentalAttributeRate { .. } |
            CasteTag::Milkable { .. } |
            CasteTag::Name { .. } |
            CasteTag::NaturalSkill { .. } |
            CasteTag::Orientation { .. } |
            CasteTag::Personality { .. } |
            CasteTag::PhysicalAttributeCapPercentage { .. } |
            CasteTag::PhysicalAttributeRange { .. } |
            CasteTag::PhysicalAttributeRate { .. } |
            CasteTag::ProfessionName { .. } |
            CasteTag::Pus { .. } |
            CasteTag::RelativeSize { .. } |
            CasteTag::Remains { .. } |
            CasteTag::RetractIntoBodyPart { .. } |
            CasteTag::RootAround { .. } |
            CasteTag::Secretion { .. } |
            CasteTag::SenseCreatureClass { .. } |
            CasteTag::SetBodyPartGroup { .. } |
            CasteTag::SkillLearnRate { .. } |
            CasteTag::SkillRate { .. } |
            CasteTag::SkillRates { .. } |
            CasteTag::SkillRustRate { .. } |
            CasteTag::SkillRustRates { .. } |
            CasteTag::Sound { .. } |
            CasteTag::SyndromeDilutionFactor { .. } |
            CasteTag::Tendons { .. } |
            CasteTag::TissueLayer { .. } |
            CasteTag::TissueLayerUnder { .. } |
            CasteTag::VerminBite { .. } |
            CasteTag::VisionArc { .. }
          => TokenComplexity::Complex,
          CasteTag::AltTile { .. } |
          CasteTag::Baby { .. } |
          CasteTag::BeachFrequency { .. } |
          CasteTag::BodyGloss { .. } |
          CasteTag::BodyPartAddType { .. } |
          CasteTag::BodyPartRemoveType { .. } |
          CasteTag::BuildingDestroyer { .. } |
          CasteTag::CanDoInteraction { .. } |
          CasteTag::ChangeBodySizePercent { .. } |
          CasteTag::Child { .. } |
          CasteTag::CreatureClass { .. } |
          CasteTag::CreatureVariationAddTag { .. } |
          CasteTag::CreatureVariationRemoveTag { .. } |
          CasteTag::Description { .. } |
          CasteTag::ExtraButcherObjectShape { .. } |
          CasteTag::EggSize { .. } |
          CasteTag::Extract { .. } |
          CasteTag::FixedTemp { .. } |
          CasteTag::Gait { .. } | // This isn't really simple..
          CasteTag::GlowTile { .. } |
          CasteTag::Gnawer { .. } |
          CasteTag::GobbleVerminClass { .. } |
          CasteTag::GrassTrample { .. } |
          CasteTag::GravitateBodySize { .. } |
          CasteTag::Grazer { .. } |
          CasteTag::Habit { ..}|
          CasteTag::HabitNumber { .. } |
          CasteTag::Homeotherm { .. } |
          CasteTag::ItemCorpseQuality { .. }|
          CasteTag::LairCharacteristic { .. }|
          CasteTag::LairHunterSpeech { .. }|
          CasteTag::LowLightVision {.. }|
          CasteTag::MannerismArms { .. }|
          CasteTag::MannerismCheek { .. }|
          CasteTag::MannerismEar { .. }|
          CasteTag::MannerismEyes { .. }|
          CasteTag::MannerismFeet { .. }|
          CasteTag::MannerismHair { .. }|
          CasteTag::MannerismKnuckles {.. }|
          CasteTag::MannerismLips { .. }|
          CasteTag::MannerismHands { .. }|
          CasteTag::MannerismHead { .. }|
          CasteTag::MannerismLeg { .. }|
          CasteTag::MannerismMouth { .. }|
          CasteTag::MannerismNose { .. }|
          CasteTag::MannerismTongue { .. }|
          CasteTag::MannerismNails { .. }|
          CasteTag::ModValue { .. }|
          CasteTag::OdorLevel { .. }|
          CasteTag::OdorString { .. }|
          CasteTag::PenetratePower { .. }|
          CasteTag::PetValue { .. }|
          CasteTag::PetValueDivisor { .. }|
          CasteTag::PopulationRatio { .. }|
          CasteTag::ProneToRage { .. }|
          CasteTag::RemainsColor { .. }|
          CasteTag::SkillLearnRates { .. }|
          CasteTag::SlainSpeech { .. }|
          CasteTag::SoldierAltTile { .. }|
          CasteTag::SoldierTile { .. }|
          CasteTag::Tile { .. }|
          CasteTag::TradeCapacity { .. }|
          CasteTag::ViewRange { .. }|
          CasteTag::Webber { .. } => TokenComplexity::Simple,
          _ => TokenComplexity::None,
        }
    }

    fn parse_token(key: &str, _value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let Some(token) = TOKEN_MAP.get(key) else {
            tracing::warn!("Unknown token: {}", key);
            return None;
        };

        match token.get_complexity() {
            TokenComplexity::None => Some(token.clone()),
            _ => todo!(),
        }
    }
}
