use super::{Token as CasteTag, TOKEN_MAP};
use crate::parser::metadata::{RawObjectToken, TokenComplexity};
use crate::parser::object_types::OBJECT_TOKEN_MAP;

#[typetag::serde]
impl RawObjectToken for CasteTag {
    #[allow(clippy::too_many_lines)]
    fn get_complexity(&self) -> TokenComplexity {
        match self {
          Self::ApplyCreatureVariation { .. } |
          Self::Attack { .. } |
          Self::AttackTrigger { .. } |
          Self::BabyName {.. } |
          Self::Body { .. } |
          Self::Blood {..}|
          Self::BodyAppearanceModifier { .. }|
          Self::BodyDetailPlan { .. } |
          Self::BodyPartAppearanceModifier {..}|
          Self::BodySize {..}|
            Self::ChildName { .. } |
            Self::ClutchSize { .. } |
            Self::Color { .. } |
            Self::EggMaterial { .. } |
            Self::ExtraButcherObjectItem { .. } |
            Self::ExtraButcherObject { .. } |
            Self::GeneralMaterialForceMultiplier { .. } |
            Self::GlowColor { .. } |
            Self::GobbleVerminCreature { .. } |
            Self::InteractionDetail { .. } |
            Self::ItemCorpse { .. } |
            Self::Lair { .. } |
            Self::LaysUnusualEggs {..}|
            Self::Ligaments { .. } |
            Self::LitterSize { .. } |
            Self::MannerismFingers { .. } |
            Self::MaxAge { .. } |
            Self::MentalAttributeCapPercentage { .. } |
            Self::MentalAttributeRange { .. } |
            Self::MentalAttributeRate { .. } |
            Self::Milkable { .. } |
            Self::Name { .. } |
            Self::NaturalSkill { .. } |
            Self::Orientation { .. } |
            Self::Personality { .. } |
            Self::PhysicalAttributeCapPercentage { .. } |
            Self::PhysicalAttributeRange { .. } |
            Self::PhysicalAttributeRate { .. } |
            Self::ProfessionName { .. } |
            Self::Pus { .. } |
            Self::RelativeSize { .. } |
            Self::Remains { .. } |
            Self::RetractIntoBodyPart { .. } |
            Self::RootAround { .. } |
            Self::Secretion { .. } |
            Self::SenseCreatureClass { .. } |
            Self::SetBodyPartGroup { .. } |
            Self::SkillLearnRate { .. } |
            Self::SkillRate { .. } |
            Self::SkillRates { .. } |
            Self::SkillRustRate { .. } |
            Self::SkillRustRates { .. } |
            Self::Sound { .. } |
            Self::SpecificFood { .. } |
            Self::SyndromeDilutionFactor { .. } |
            Self::Tendons { .. } |
            Self::TissueLayer { .. } |
            Self::TissueLayerUnder { .. } |
            Self::VerminBite { .. } |
            Self::VisionArc { .. }
          => {
                tracing::trace!("get_complexity: {self:?} is 'Complex'");
                TokenComplexity::Complex
        }
          Self::AltTile { .. } |
          Self::Baby { .. } |
          Self::BeachFrequency { .. } |
          Self::BodyGloss { .. } |
          Self::BodyPartAddType { .. } |
          Self::BodyPartRemoveType { .. } |
          Self::BuildingDestroyer { .. } |
          Self::CanDoInteraction { .. } |
          Self::ChangeBodySizePercent { .. } |
          Self::Child { .. } |
          Self::CreatureClass { .. } |
          Self::CreatureVariationAddTag { .. } |
          Self::CreatureVariationRemoveTag { .. } |
          Self::Description { .. } |
          Self::Difficulty { .. } |
          Self::ExtraButcherObjectShape { .. } |
          Self::EggSize { .. } |
          Self::Extract { .. } |
          Self::FixedTemp { .. } |
          Self::Gait { .. } | // This isn't really simple..
          Self::GlowTile { .. } |
          Self::Gnawer { .. } |
          Self::GobbleVerminClass { .. } |
          Self::GrassTrample { .. } |
          Self::GravitateBodySize { .. } |
          Self::Grazer { .. } |
          Self::Habit { ..}|
          Self::HabitNumber { .. } |
          Self::Homeotherm { .. } |
          Self::ItemCorpseQuality { .. }|
          Self::LairCharacteristic { .. }|
          Self::LairHunterSpeech { .. }|
          Self::LowLightVision {.. }|
          Self::MannerismArms { .. }|
          Self::MannerismCheek { .. }|
          Self::MannerismEar { .. }|
          Self::MannerismEyes { .. }|
          Self::MannerismFeet { .. }|
          Self::MannerismHair { .. }|
          Self::MannerismKnuckles {.. }|
          Self::MannerismLips { .. }|
          Self::MannerismHands { .. }|
          Self::MannerismHead { .. }|
          Self::MannerismLeg { .. }|
          Self::MannerismMouth { .. }|
          Self::MannerismNose { .. }|
          Self::MannerismTongue { .. }|
          Self::MannerismNails { .. }|
          Self::ModValue { .. }|
          Self::OdorLevel { .. }|
          Self::OdorString { .. }|
          Self::PenetratePower { .. }|
          Self::PetValue { .. }|
          Self::PetValueDivisor { .. }|
          Self::PopulationRatio { .. }|
          Self::ProneToRage { .. }|
          Self::RemainsColor { .. }|
          Self::SkillLearnRates { .. }|
          Self::SlainSpeech { .. }|
          Self::SoldierAltTile { .. }|
          Self::SoldierTile { .. }|
          Self::Tile { .. }|
          Self::TradeCapacity { .. }|
          Self::ViewRange { .. }|
          Self::Webber { .. } => {
                tracing::trace!("get_complexity: {self:?} is 'Simple'");
                TokenComplexity::Simple
          }
          _ => {
                tracing::trace!("get_complexity: {self:?} is 'None'");
                TokenComplexity::None
          }
        }
    }

    fn parse_token(key: &str, value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let Some(token) = TOKEN_MAP.get(key) else {
            tracing::error!("parse_token: unknown token: {}", key);
            return None;
        };

        match token.get_complexity() {
            TokenComplexity::None => Some(token.clone()),
            TokenComplexity::Simple => {
                // All of these tokens have a pattern of `key:value` so we can parse `value` as appropriate
                // We just pass this off to the token's `simple_parse` method to handle the parsing
                parse_simple_token(token, value)
            }
            TokenComplexity::Complex => {
                // These tokens have a variable number of arguments, so we need to parse them differently
                // We pass this off to the token's `complex_parse` method to handle the parsing
                parse_complex_token(token, value)
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
/// Parses a token with the simple pattern of `key:value`
/// This is used for tokens that have a single value, such as `BABY:1`
/// This is also used for tokens that have a single value with a single argument, such as `BODY:BODY_HUMANOID`
///
/// # Arguments
///
/// * `token` - The token to parse
/// * `value` - The value of the token (the part after the `:`)
///
/// # Returns
///
/// * `Some(Self)` - The parsed token
/// * `None` - The token could not be parsed
fn parse_simple_token(token: &CasteTag, value: &str) -> Option<CasteTag> {
    match token {
        CasteTag::AltTile { .. } => {
            let tile = String::from(value);
            Some(CasteTag::AltTile { tile })
        }
        CasteTag::Baby { .. } => {
            let age: u32 = value.parse().ok().unwrap_or_default();
            Some(CasteTag::Baby { age })
        }
        CasteTag::BeachFrequency { .. } => {
            let frequency: u32 = value.parse().ok().unwrap_or_default();
            Some(CasteTag::BeachFrequency { frequency })
        }
        CasteTag::BodyGloss { .. } => {
            let gloss = String::from(value);
            Some(CasteTag::BodyGloss { gloss })
        }
        CasteTag::BodyPartAddType { .. } => {
            let body_part_type = String::from(value);
            Some(CasteTag::BodyPartAddType { body_part_type })
        }
        CasteTag::BodyPartRemoveType { .. } => {
            let body_part_type = String::from(value);
            Some(CasteTag::BodyPartRemoveType { body_part_type })
        }
        CasteTag::BuildingDestroyer { .. } => {
            let building_destroyer: u32 = value.parse().ok().unwrap_or_default();
            let door_and_furniture_focused = building_destroyer == 1;
            Some(CasteTag::BuildingDestroyer {
                door_and_furniture_focused,
            })
        }
        CasteTag::CanDoInteraction { .. } => {
            let interaction = String::from(value);
            Some(CasteTag::CanDoInteraction { interaction })
        }
        CasteTag::ChangeBodySizePercent { .. } => {
            let percent: u32 = value.parse().ok().unwrap_or_default();
            Some(CasteTag::ChangeBodySizePercent { percent })
        }
        CasteTag::Child { .. } => {
            let age: u32 = value.parse().ok().unwrap_or_default();
            Some(CasteTag::Child { age })
        }
        CasteTag::CreatureClass { .. } => {
            let class = String::from(value);
            Some(CasteTag::CreatureClass { class })
        }
        CasteTag::CreatureVariationAddTag { .. } => {
            let tag = String::from(value);
            Some(CasteTag::CreatureVariationAddTag { tag })
        }
        CasteTag::CreatureVariationRemoveTag { .. } => {
            let tag = String::from(value);
            Some(CasteTag::CreatureVariationRemoveTag { tag })
        }
        CasteTag::Description { .. } => {
            let description = String::from(value);
            Some(CasteTag::Description { description })
        }
        CasteTag::Difficulty { .. } => {
            let Ok(difficulty) = value.parse::<u32>() else {
                tracing::warn!("parse_simple_token: Cannot parse difficulty: {}", value);
                return None;
            };
            Some(CasteTag::Difficulty { difficulty })
        }
        CasteTag::ExtraButcherObjectShape { .. } => {
            let shape = String::from(value);
            Some(CasteTag::ExtraButcherObjectShape { shape })
        }
        CasteTag::EggSize { .. } => {
            let size: u32 = value.parse().ok().unwrap_or_default();
            Some(CasteTag::EggSize { size })
        }
        CasteTag::Extract { .. } => {
            let material = String::from(value);
            Some(CasteTag::Extract { material })
        }
        CasteTag::FixedTemp { .. } => {
            let temperature: i32 = value.parse().ok().unwrap_or_default();
            Some(CasteTag::FixedTemp { temperature })
        }
        CasteTag::Gait { .. } => {
            let gait = String::from(value);
            Some(CasteTag::Gait { gait })
        }
        CasteTag::GlowTile { .. } => {
            let tile = String::from(value);
            Some(CasteTag::GlowTile { tile })
        }
        CasteTag::Gnawer { .. } => {
            let verb = String::from(value);
            Some(CasteTag::Gnawer { verb })
        }
        CasteTag::GobbleVerminClass { .. } => {
            let vermin_class = String::from(value);
            Some(CasteTag::GobbleVerminClass { vermin_class })
        }
        CasteTag::GrassTrample { .. } => {
            let trample = value.parse::<u32>().ok()?;
            Some(CasteTag::GrassTrample { trample })
        }
        CasteTag::GravitateBodySize { .. } => {
            let target: u32 = value.parse().ok().unwrap_or_default();
            Some(CasteTag::GravitateBodySize { target })
        }
        CasteTag::Grazer { .. } => {
            let grazer: u32 = value.parse().ok().unwrap_or_default();
            Some(CasteTag::Grazer { grazer })
        }
        CasteTag::Habit { .. } => {
            let habit = String::from(value);
            Some(CasteTag::Habit { habit })
        }
        CasteTag::HabitNumber { .. } => {
            let number: u32 = value.parse().ok().unwrap_or_default();
            Some(CasteTag::HabitNumber { number })
        }
        CasteTag::Homeotherm { .. } => {
            let temperature: u32 = value.parse().ok().unwrap_or_default();
            Some(CasteTag::Homeotherm {
                temperature: Some(temperature),
            })
        }
        CasteTag::ItemCorpseQuality { .. } => {
            let quality = value.parse::<u32>().ok()?;
            Some(CasteTag::ItemCorpseQuality { quality })
        }
        CasteTag::LairCharacteristic { .. } => {
            let characteristic = String::from(value);
            Some(CasteTag::LairCharacteristic { characteristic })
        }
        CasteTag::LairHunterSpeech { .. } => {
            let speech_file = String::from(value);
            Some(CasteTag::LairHunterSpeech { speech_file })
        }
        CasteTag::LowLightVision { .. } => {
            let vision = value.parse::<u32>().ok()?;
            Some(CasteTag::LowLightVision { vision })
        }
        CasteTag::MannerismArms { .. } => {
            let arms = String::from(value);
            Some(CasteTag::MannerismArms { arms })
        }
        CasteTag::MannerismCheek { .. } => {
            let cheek = String::from(value);
            Some(CasteTag::MannerismCheek { cheek })
        }
        CasteTag::MannerismEar { .. } => {
            let ear = String::from(value);
            Some(CasteTag::MannerismEar { ear })
        }
        CasteTag::MannerismEyes { .. } => {
            let eyes = String::from(value);
            Some(CasteTag::MannerismEyes { eyes })
        }
        CasteTag::MannerismFeet { .. } => {
            let feet = String::from(value);
            Some(CasteTag::MannerismFeet { feet })
        }
        CasteTag::MannerismHair { .. } => {
            let hair = String::from(value);
            Some(CasteTag::MannerismHair { hair })
        }
        CasteTag::MannerismKnuckles { .. } => {
            let knuckles = String::from(value);
            Some(CasteTag::MannerismKnuckles { knuckles })
        }
        CasteTag::MannerismLips { .. } => {
            let lips = String::from(value);
            Some(CasteTag::MannerismLips { lips })
        }
        CasteTag::MannerismHands { .. } => {
            let hands = String::from(value);
            Some(CasteTag::MannerismHands { hands })
        }
        CasteTag::MannerismHead { .. } => {
            let head = String::from(value);
            Some(CasteTag::MannerismHead { head })
        }
        CasteTag::MannerismLeg { .. } => {
            let leg = String::from(value);
            Some(CasteTag::MannerismLeg { leg })
        }
        CasteTag::MannerismMouth { .. } => {
            let mouth = String::from(value);
            Some(CasteTag::MannerismMouth { mouth })
        }
        CasteTag::MannerismNose { .. } => {
            let nose = String::from(value);
            Some(CasteTag::MannerismNose { nose })
        }
        CasteTag::MannerismTongue { .. } => {
            let tongue = String::from(value);
            Some(CasteTag::MannerismTongue { tongue })
        }
        CasteTag::MannerismNails { .. } => {
            let nails = String::from(value);
            Some(CasteTag::MannerismNails { nails })
        }
        CasteTag::ModValue { .. } => {
            let value = String::from(value);
            Some(CasteTag::ModValue { value })
        }
        CasteTag::OdorLevel { .. } => {
            let odor_level: u32 = value.parse().ok().unwrap_or_default();
            Some(CasteTag::OdorLevel { odor_level })
        }
        CasteTag::OdorString { .. } => {
            let odor_string = String::from(value);
            Some(CasteTag::OdorString { odor_string })
        }
        CasteTag::PenetratePower { .. } => {
            let penetrate_power: u32 = value.parse().ok().unwrap_or_default();
            Some(CasteTag::PenetratePower { penetrate_power })
        }
        CasteTag::PetValue { .. } => {
            let pet_value: u32 = value.parse().ok().unwrap_or_default();
            Some(CasteTag::PetValue { pet_value })
        }
        CasteTag::PetValueDivisor { .. } => {
            let Ok(divisor) = value.parse::<u32>() else {
                tracing::warn!(
                    "parse_simple_token: Cannot parse pet value divisor: {}",
                    value
                );
                return None;
            };
            Some(CasteTag::PetValueDivisor { divisor })
        }
        CasteTag::PopulationRatio { .. } => {
            let Ok(pop_ratio) = value.parse::<u32>() else {
                tracing::warn!(
                    "parse_simple_token: Cannot parse population ratio: {}",
                    value
                );
                return None;
            };
            Some(CasteTag::PopulationRatio { pop_ratio })
        }
        CasteTag::ProneToRage { .. } => {
            let Ok(rage_chance) = value.parse::<u32>() else {
                tracing::warn!("parse_simple_token: Cannot parse rage chance: {}", value);
                return None;
            };
            Some(CasteTag::ProneToRage { rage_chance })
        }
        CasteTag::RemainsColor { .. } => {
            let remains_color = String::from(value);
            Some(CasteTag::RemainsColor { remains_color })
        }
        CasteTag::SkillLearnRates { .. } => {
            let Ok(rate) = value.parse::<u32>() else {
                tracing::warn!(
                    "parse_simple_token: Cannot parse skill learn rate: {}",
                    value
                );
                return None;
            };
            Some(CasteTag::SkillLearnRates { rate })
        }
        CasteTag::SlainSpeech { .. } => {
            let speech_file = String::from(value);
            Some(CasteTag::SlainSpeech { speech_file })
        }
        CasteTag::SoldierAltTile { .. } => {
            let tile = String::from(value);
            Some(CasteTag::SoldierAltTile { tile })
        }
        CasteTag::SoldierTile { .. } => {
            let tile = String::from(value);
            Some(CasteTag::SoldierTile { tile })
        }
        CasteTag::Tile { .. } => {
            let tile = String::from(value);
            Some(CasteTag::Tile { tile })
        }
        CasteTag::TradeCapacity { .. } => {
            let Ok(capacity) = value.parse::<u32>() else {
                tracing::warn!("parse_simple_token: Cannot parse trade capacity: {}", value);
                return None;
            };
            Some(CasteTag::TradeCapacity { capacity })
        }
        CasteTag::ViewRange { .. } => {
            let Ok(view_range) = value.parse::<u32>() else {
                tracing::warn!("parse_simple_token: Cannot parse view range: {}", value);
                return None;
            };
            Some(CasteTag::ViewRange { view_range })
        }
        CasteTag::Webber { .. } => {
            let material = String::from(value);
            Some(CasteTag::Webber { material })
        }
        _ => {
            tracing::error!(
                "parse_simple_token: Cannot parse token (not simple): {:?}",
                token
            );
            None
        }
    }
}

#[allow(clippy::too_many_lines)]
/// Parses a token with the complex pattern of `key:value1:value2:value3` or similar. Each complex token
/// has a different number of arguments, so we need to parse them differently.
///
/// # Arguments
///
/// * `token` - The token to parse
/// * `value` - The value of the token (the part after the first `:`)
///
/// # Returns
///
/// * `Some(Self)` - The parsed token
/// * `None` - The token could not be parsed
fn parse_complex_token(token: &CasteTag, value: &str) -> Option<CasteTag> {
    let mut values = value.split(':').collect::<Vec<&str>>();
    match token {
        CasteTag::ApplyCreatureVariation { .. } => {
            // check if there are enough arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse ApplyCreatureVariation: not enough arguments: {}/2 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let id = values.remove(0).to_string();
            let args = values
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>();
            Some(CasteTag::ApplyCreatureVariation { id, args })
        }
        CasteTag::Attack { .. } => {
            // Appears as `ATTACK:NAME:BODYPART:BY_CATEGORY:HORN`
            // check if there are enough arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse Attack: not enough arguments: {}/2 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let name = values.remove(0).to_string();
            let body_part = values.join(":").to_string();
            Some(CasteTag::Attack { name, body_part })
        }
        CasteTag::AttackTrigger { .. } => {
            // Appears as `ATTACK_TRIGGER:0:1:2` for population, exported_wealth and created_wealth
            // check if there are enough arguments
            if values.len() < 3 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse AttackTrigger: not enough arguments: {}/3 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let Ok(population) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse AttackTrigger: population: {value}"
                );
                return None;
            };
            let Ok(exported_wealth) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse AttackTrigger: exported wealth: {value}"
                );
                return None;
            };
            let Ok(created_wealth) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse AttackTrigger: created wealth: {value}"
                );
                return None;
            };
            Some(CasteTag::AttackTrigger {
                population,
                exported_wealth,
                created_wealth,
            })
        }
        CasteTag::BabyName { .. } => {
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse BabyName: not enough arguments: {}/2 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let singular = values.remove(0).to_string();
            let plural = values.join(":").to_string();
            Some(CasteTag::BabyName { singular, plural })
        }
        CasteTag::Blood { .. } => {
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse Blood: not enough arguments: {}/2 '{value}'",
                    values.len(),
                );
                return None;
            }
            let material = values.remove(0).to_string();
            let state = values.join(":").to_string();
            Some(CasteTag::Blood { material, state })
        }
        CasteTag::Body { .. } => {
            let body_parts = values
                .iter()
                .map(std::string::ToString::to_string)
                .collect();
            Some(CasteTag::Body { body_parts })
        }
        CasteTag::BodyAppearanceModifier { .. } => {
            // Arguments become a string (attribute) and 7 i32s, separated by `:`
            if values.len() < 8 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse BodyAppearanceModifier: not enough arguments: {}/8 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // Parse first argument as `String`
            let attribute = values.remove(0).to_string();
            let Ok(lowest) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lowest: {value}"
                );
                return None;
            };
            let Ok(lower) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower: {value}"
                );
                return None;
            };
            let Ok(lower_median) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower medium: {value}"
                );
                return None;
            };
            let Ok(median) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: medium: {value}"
                );
                return None;
            };
            let Ok(upper_median) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper medium: {value}"
                );
                return None;
            };
            let Ok(upper) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper: {value}"
                );
                return None;
            };
            let Ok(highest) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: highest: {value}"
                );
                return None;
            };
            Some(CasteTag::BodyAppearanceModifier {
                attribute,
                values: [
                    lowest,
                    lower,
                    lower_median,
                    median,
                    upper_median,
                    upper,
                    highest,
                ],
            })
        }
        CasteTag::BodyPartAppearanceModifier { .. } => {
            // Arguments become a string (attribute) and 7 i32s, separated by `:`
            if values.len() < 8 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse BodyPartAppearanceModifier: not enough arguments: {}/8 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // Parse first argument as `String`
            let quality = values.remove(0).to_string();
            let Ok(lowest) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lowest: {value}"
                );
                return None;
            };
            let Ok(lower) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower: {value}"
                );
                return None;
            };
            let Ok(lower_median) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower medium: {value}"
                );
                return None;
            };
            let Ok(median) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: medium: {value}"
                );
                return None;
            };
            let Ok(upper_median) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper medium: {value}"
                );
                return None;
            };
            let Ok(upper) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper: {value}"
                );
                return None;
            };
            let Ok(highest) = values.remove(0).parse::<i32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: highest: {value}"
                );
                return None;
            };
            Some(CasteTag::BodyPartAppearanceModifier {
                quality,
                spread: [
                    lowest,
                    lower,
                    lower_median,
                    median,
                    upper_median,
                    upper,
                    highest,
                ],
            })
        }
        CasteTag::BodyDetailPlan { .. } => {
            let body_plan = values.remove(0).to_string();
            let arguments = values
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>();
            Some(CasteTag::BodyDetailPlan {
                body_plan,
                arguments,
            })
        }
        CasteTag::BodySize { .. } => {
            // Body size is [YEAR:DAYS:SIZE], all are u32s
            if values.len() < 3 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse body size: not enough arguments: {}/3 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let Ok(year) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse body size: year: {value}");
                return None;
            };
            let Ok(days) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse body size: days: {value}");
                return None;
            };
            let Ok(size) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse body size: size: {value}");
                return None;
            };
            Some(CasteTag::BodySize { year, days, size })
        }
        CasteTag::ChildName { .. } => {
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse ChildName: not enough arguments: {}/2 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let singular = values.remove(0).to_string();
            let plural = values.join(":").to_string();
            Some(CasteTag::ChildName { singular, plural })
        }
        CasteTag::ClutchSize { .. } => {
            // Two `u32`s
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse clutch size: not enough arguments: {}/2 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let Ok(min) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse clutch size: min: {value}");
                return None;
            };
            let Ok(max) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse clutch size: max: {value}");
                return None;
            };
            Some(CasteTag::ClutchSize { min, max })
        }
        CasteTag::Color { .. } => {
            if values.len() < 3 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse color: not enough arguments: {}/3 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let Ok(foreground) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse color: foreground: {value}");
                return None;
            };
            let Ok(background) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse color: background: {value}");
                return None;
            };
            let Ok(brightness) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse color: brightness: {value}");
                return None;
            };
            Some(CasteTag::Color {
                foreground,
                background,
                brightness,
            })
        }
        CasteTag::EggMaterial { .. } => {
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse egg material: not enough arguments: {}/2 '{value}'",
                    values.len(),
                );
                return None;
            }
            // Take the last `String` as the `state` and the rest as the `material`
            let state = values.pop().unwrap_or_default().to_string();
            let material = values.join(":").to_string();
            Some(CasteTag::EggMaterial { material, state })
        }
        CasteTag::ExtraButcherObject { .. } => {
            // `String` and `Vec<String>`
            let object_type = values.remove(0).to_string();
            let arguments = values
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>();
            Some(CasteTag::ExtraButcherObject {
                object_type,
                arguments,
            })
        }
        CasteTag::ExtraButcherObjectItem { .. } => {
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse extra butcher object item: not enough arguments: {}/2 '{value}'",
                    values.len(),
                );
                return None;
            }
            // Two strings
            let item = values.remove(0).to_string();
            let material = values.join(":").to_string();
            Some(CasteTag::ExtraButcherObjectItem { item, material })
        }
        CasteTag::GeneralMaterialForceMultiplier { .. } => {
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse general material force multiplier: not enough arguments: {}/2 '{value}'",
                    values.len(),
                );
                return None;
            }
            // Two `u32`s
            let Ok(value_a) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse general material force multiplier: {value}"
                );
                return None;
            };
            let Ok(value_b) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse general material force multiplier: {value}"
                );
                return None;
            };
            Some(CasteTag::GeneralMaterialForceMultiplier { value_a, value_b })
        }
        CasteTag::GlowColor { .. } => {
            // Arguments become 3 `u32`s, separated by `:`
            if values.len() < 3 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse glow color: not enough arguments: {}/3 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let Ok(foreground) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse glow color: foreground: {value}");
                return None;
            };
            let Ok(background) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse glow color: background: {value}");
                return None;
            };
            let Ok(brightness) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse glow color: brightness: {value}");
                return None;
            };
            Some(CasteTag::GlowColor {
                foreground,
                background,
                brightness,
            })
        }
        CasteTag::GobbleVerminCreature { .. } => {
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse gobble vermin creature: not enough arguments: {}/2 '{value}'",
                    values.len(),
                );
                return None;
            }
            // Two strings
            let vermin_creature = values.remove(0).to_string();
            let vermin_caste = values.join(":").to_string();
            Some(CasteTag::GobbleVerminCreature {
                vermin_creature,
                vermin_caste,
            })
        }
        CasteTag::InteractionDetail { .. } => {
            let args = values
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>();
            Some(CasteTag::InteractionDetail { args })
        }
        CasteTag::ItemCorpse { .. } => {
            // Two strings
            let item = values.remove(0).to_string();
            let material = values.join(":").to_string();
            Some(CasteTag::ItemCorpse { item, material })
        }
        CasteTag::Lair { .. } => {
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse lair: not enough arguments: {}/2 '{value}'",
                    values.len(),
                );
                return None;
            }
            // `String` and `u32`
            let lair = values.remove(0).to_string();
            let Ok(probability) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse lair proability: {value}");
                return None;
            };
            Some(CasteTag::Lair { lair, probability })
        }
        CasteTag::LaysUnusualEggs { .. } => {
            // Two strings
            let item = values.remove(0).to_string();
            let material = values.join(":").to_string();
            Some(CasteTag::LaysUnusualEggs { item, material })
        }
        CasteTag::Ligaments { .. } => {
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse ligaments: not enough arguments: {}/2 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // Grab `healing_rate` from the end of `value`
            let Ok(healing_rate) = values.pop().unwrap_or_default().parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse ligaments: healing rate: {value}"
                );
                return None;
            };
            let material = values.join(":").to_string();
            Some(CasteTag::Ligaments {
                material,
                healing_rate,
            })
        }
        CasteTag::LitterSize { .. } => {
            // Two `u32`s
            let mut values = value.split(':').collect::<Vec<&str>>();
            let Ok(min) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse litter size: min: {value}");
                return None;
            };
            let Ok(max) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse litter size: max: {value}");
                return None;
            };
            Some(CasteTag::LitterSize { min, max })
        }
        CasteTag::MannerismFingers { .. } => {
            let finger = values.remove(0).to_string();
            let fingers = values.join(":").to_string();
            Some(CasteTag::MannerismFingers { finger, fingers })
        }
        CasteTag::MaxAge { .. } => {
            // Two `u32`s
            let mut values = value.split(':').collect::<Vec<&str>>();
            let Ok(min) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse max age: min: {value}");
                return None;
            };
            let Ok(max) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse max age: max: {value}");
                return None;
            };
            Some(CasteTag::MaxAge { min, max })
        }
        CasteTag::MentalAttributeCapPercentage { .. } => {
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute cap percentage: not enough arguments: {}/2 '{value}'",
                    values.len(),
                );
                return None;
            }
            let attribute = values.remove(0).to_string();
            let Ok(percentage) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute cap percentage: {value}"
                );
                return None;
            };
            Some(CasteTag::MentalAttributeCapPercentage {
                attribute,
                percentage,
            })
        }
        CasteTag::MentalAttributeRange { .. } => {
            // Arguments become a `String` and 7 `u32`s
            if values.len() < 8 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: not enough arguments: {}/8 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // Parse first argument as `String`
            let attribute = values.remove(0).to_string();
            let Ok(lowest) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lowest: {value}"
                );
                return None;
            };
            let Ok(lower) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower: {value}"
                );
                return None;
            };
            let Ok(lower_median) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower medium: {value}"
                );
                return None;
            };
            let Ok(median) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: medium: {value}"
                );
                return None;
            };
            let Ok(upper_median) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper medium: {value}"
                );
                return None;
            };
            let Ok(upper) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper: {value}"
                );
                return None;
            };
            let Ok(highest) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: highest: {value}"
                );
                return None;
            };
            Some(CasteTag::MentalAttributeRange {
                attribute,
                ranges: [
                    lowest,
                    lower,
                    lower_median,
                    median,
                    upper_median,
                    upper,
                    highest,
                ],
            })
        }
        CasteTag::MentalAttributeRate { .. } => {
            // Arguments become a `String` and 4 `u32`s
            if values.len() < 5 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: not enough arguments: {}/5 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // Parse first argument as `String`
            let attribute = values.remove(0).to_string();
            let Ok(improvement_cost) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: improvement cost: {value}"
                );
                return None;
            };
            let Ok(decay_rate_unused) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: decay rate unused: {value}"
                );
                return None;
            };
            let Ok(decay_rate_rusty) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: decay rate rusty: {value}"
                );
                return None;
            };
            let Ok(decay_rate_demotion) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: decay rate demotion: {value}"
                );
                return None;
            };
            Some(CasteTag::MentalAttributeRate {
                attribute,
                improvement_cost,
                decay_rate_unused,
                decay_rate_rusty,
                decay_rate_demotion,
            })
        }
        CasteTag::Milkable { .. } => {
            // Arguments become a `String` and `u32`
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse milkable: not enough arguments: {}/2 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // Frequency is the last argument, parsed as `u32`
            let Ok(frequency) = values.pop().unwrap_or_default().parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse milkable: frequency: {value}");
                return None;
            };
            // Material is the rest of the arguments, joined as a `String`
            let material = values.join(":").to_string();
            Some(CasteTag::Milkable {
                material,
                frequency,
            })
        }
        CasteTag::Name { .. } => {
            // Arguments become a singular name, plural name, and adjective, separated by `:`
            let mut values = value.split(':').collect::<Vec<&str>>();
            if values.len() < 3 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse name: not enough arguments: {}/3 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let singular = values.remove(0).to_string();
            let plural = values.remove(0).to_string();
            let adjective = values.join(":").to_string();
            Some(CasteTag::Name {
                singular,
                plural,
                adjective,
            })
        }
        CasteTag::NaturalSkill { .. } => {
            let mut values = value.split(':').collect::<Vec<&str>>();
            // Grab `level` from the end of `value`
            let Ok(level) = values.pop().unwrap_or_default().parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse natural skill: level: {value}");
                return None;
            };
            let skill = values.join(":").to_string();
            Some(CasteTag::NaturalSkill { skill, level })
        }
        CasteTag::Personality { .. } => {
            let mut values = value.split(':').collect::<Vec<&str>>();
            // Check if there are enough arguments to parse
            if values.len() < 4 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse personality: not enough arguments: {}/4 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // Parse first argument as `String`
            let personality_trait = values.remove(0).to_string();
            let Ok(low) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse personality: low: {value}");
                return None;
            };
            let Ok(median) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse personality: median: {value}");
                return None;
            };
            let Ok(high) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse personality: high: {value}");
                return None;
            };
            Some(CasteTag::Personality {
                personality_trait,
                low,
                median,
                high,
            })
        }
        CasteTag::PhysicalAttributeCapPercentage { .. } => {
            // Arguments become a `String` and 1 `u32`s
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute cap percentage: not enough arguments: {}/2 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // Parse first argument as `String`
            let attribute = values.remove(0).to_string();
            let Ok(percentage) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute cap percentage: {value}"
                );
                return None;
            };
            Some(CasteTag::PhysicalAttributeCapPercentage {
                attribute,
                percentage,
            })
        }
        CasteTag::PhysicalAttributeRange { .. } => {
            // Arguments become a `String` and 7 `u32`s
            if values.len() < 8 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: not enough arguments: {}/8 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // Parse first argument as `String`
            let attribute = values.remove(0).to_string();
            let Ok(lowest) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: lowest: {value}"
                );
                return None;
            };
            let Ok(lower) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: lower: {value}"
                );
                return None;
            };
            let Ok(lower_median) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: lower medium: {value}"
                );
                return None;
            };
            let Ok(median) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: medium: {value}"
                );
                return None;
            };
            let Ok(upper_median) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: upper medium: {value}"
                );
                return None;
            };
            let Ok(upper) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: upper: {value}"
                );
                return None;
            };
            let Ok(highest) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: highest: {value}"
                );
                return None;
            };
            Some(CasteTag::PhysicalAttributeRange {
                attribute,
                ranges: [
                    lowest,
                    lower,
                    lower_median,
                    median,
                    upper_median,
                    upper,
                    highest,
                ],
            })
        }
        CasteTag::PhysicalAttributeRate { .. } => {
            // Arguments become a `String` and 4 `u32`s
            if values.len() < 5 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: not enough arguments: {}/5 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // Parse first argument as `String`
            let attribute = values.remove(0).to_string();
            let Ok(improvement_cost) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: improvement cost: {value}"
                );
                return None;
            };
            let Ok(decay_rate_unused) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: decay rate unused: {value}"
                );
                return None;
            };
            let Ok(decay_rate_rusty) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: decay rate rusty: {value}"
                );
                return None;
            };
            let Ok(decay_rate_demotion) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: decay rate demotion: {value}"
                );
                return None;
            };
            Some(CasteTag::PhysicalAttributeRate {
                attribute,
                improvement_cost,
                decay_rate_unused,
                decay_rate_rusty,
                decay_rate_demotion,
            })
        }
        CasteTag::ProfessionName { .. } => {
            // Arguments become a singular name, plural name, and adjective, separated by `:`
            if values.len() < 3 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse profession name: not enough arguments: {}/3 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let profession = values.remove(0).to_string();
            let singular = values.remove(0).to_string();
            let plural = values.remove(0).to_string();
            Some(CasteTag::ProfessionName {
                profession,
                singular,
                plural,
            })
        }
        CasteTag::Pus { .. } => {
            // `material_state` is a `String` and is at the end of `value`
            // `material` is `simple_value` + the beginning of `value` until we get to `material_state`
            // split `value` into a vector of strings
            let mut args = value.split(':').collect::<Vec<&str>>();
            // Grab `material_state` from the end of `value`
            let material_state = args.pop().unwrap_or_default().to_string();
            // Set `material` to `simple_value` + the remains of `value`
            let material = args.join(":").to_string();
            Some(CasteTag::Pus {
                material,
                material_state,
            })
        }
        CasteTag::RelativeSize { .. } => {
            // Appears as `RELATIVE_SIZE:SomeBodyPartSelector:SomeBodyPart:100`
            // check if there are enough arguments
            if values.len() < 3 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse RelativeSize: not enough arguments: {}/3 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // `relative_size` is the last argument, parsed as `u32`
            let Ok(relative_size) = values.pop().unwrap_or_default().parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse RelativeSize: relative size: {value}"
                );
                return None;
            };
            let body_part_selector = values.remove(0).to_string();
            let body_part = values.join(":").to_string();
            Some(CasteTag::RelativeSize {
                body_part_selector,
                body_part,
                relative_size,
            })
        }
        CasteTag::Remains { .. } => {
            // Appears as `REMAINS:SomeRemain:SomeRemains`
            // check if there are enough arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse Remains: not enough arguments: {}/2 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let singular = values.remove(0).to_string();
            let plural = values.join(":").to_string();
            Some(CasteTag::Remains { singular, plural })
        }
        CasteTag::RetractIntoBodyPart { .. } => {
            // check if there are enough arguments
            if values.len() < 6 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse RetractIntoBodyPart: not enough arguments: {}/6 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // We grab the strings from the end first
            let third_person_cancel = values.pop().unwrap_or_default().to_string();
            let second_person_cancel = values.pop().unwrap_or_default().to_string();
            let third_person = values.pop().unwrap_or_default().to_string();
            let second_person = values.pop().unwrap_or_default().to_string();
            // Then the body_part_selector
            let body_part_selector = values.remove(0).to_string();
            // And finally the body_part
            let body_part = values.join(":").to_string();
            Some(CasteTag::RetractIntoBodyPart {
                body_part_selector,
                body_part,
                second_person,
                third_person,
                second_person_cancel,
                third_person_cancel,
            })
        }
        CasteTag::RootAround { .. } => {
            if values.len() < 4 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse RootAround: not enough arguments: {}/4 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let third_person_verb = values.pop().unwrap_or_default().to_string();
            let second_person_verb = values.pop().unwrap_or_default().to_string();
            let body_part_selector = values.remove(0).to_string();
            let body_part = values.join(":").to_string();
            Some(CasteTag::RootAround {
                body_part_selector,
                body_part,
                second_person_verb,
                third_person_verb,
            })
        }
        CasteTag::Secretion { .. } => {
            if values.len() < 6 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse Secretion: not enough arguments: {}/6 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let material_token = values.remove(0).to_string();
            let material_state = values.remove(0).to_string();
            let body_part_selector = values.remove(0).to_string();
            // Grab from the end
            let trigger = values.pop().unwrap_or_default().to_string();
            let tissue_layer = values.pop().unwrap_or_default().to_string();
            let body_part = values.join(":").to_string();
            Some(CasteTag::Secretion {
                material_token,
                material_state,
                body_part_selector,
                body_part,
                tissue_layer,
                trigger,
            })
        }
        CasteTag::SenseCreatureClass { .. } => {
            // Appears as `SENSE_CREATURE_CLASS:SomeCreatureClass:SomeTile:0:0:0`
            if values.len() < 5 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse SenseCreatureClass: not enough arguments: {}/5 '{value}'",
                    values.len(),
                );
                return None;
            }
            let creature_class = values.remove(0).to_string();
            let tile = values.remove(0).to_string();
            let Ok(foreground) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse SenseCreatureClass: foreground: {value}"
                );
                return None;
            };
            let Ok(background) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse SenseCreatureClass: background: {value}"
                );
                return None;
            };
            let Ok(brightness) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse SenseCreatureClass: brightness: {value}"
                );
                return None;
            };

            Some(CasteTag::SenseCreatureClass {
                creature_class,
                tile,
                foreground,
                background,
                brightness,
            })
        }
        CasteTag::SetBodyPartGroup { .. } => {
            // Check if there are enough arguments to parse
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse set body part group: not enough arguments: {}/2 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // Parse first argument as `String`
            let body_part_selector = values.remove(0).to_string();
            let body_part = values.join(":").to_string();
            Some(CasteTag::SetBodyPartGroup {
                body_part_selector,
                body_part,
            })
        }
        CasteTag::SkillRates { .. } => {
            // Check if there are enough arguments to parse
            if values.len() < 4 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: not enough arguments: {}/4 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let Ok(improvement_rate) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: improvement rate: {value}"
                );
                return None;
            };
            let Ok(decay_rate_unused) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: decay rate unused: {value}"
                );
                return None;
            };
            let Ok(decay_rate_rusty) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: decay rate rusty: {value}"
                );
                return None;
            };
            let Ok(decay_rate_demotion) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: decay rate demotion: {value}"
                );
                return None;
            };
            Some(CasteTag::SkillRates {
                improvement_rate,
                decay_rate_unused,
                decay_rate_rusty,
                decay_rate_demotion,
            })
        }
        CasteTag::SkillRustRates { .. } => {
            // Check if there are enough arguments to parse
            if values.len() < 3 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse skill rust rates: not enough arguments: {}/3 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let Ok(decay_rate_unused) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse skill rust rates: decay rate unused: {value}"
                );
                return None;
            };
            let Ok(decay_rate_rusty) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse skill rust rates: decay rate rusty: {value}"
                );
                return None;
            };
            let Ok(decay_rate_demotion) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse skill rust rates: decay rate demotion: {value}"
                );
                return None;
            };
            Some(CasteTag::SkillRustRates {
                decay_rate_unused,
                decay_rate_rusty,
                decay_rate_demotion,
            })
        }
        CasteTag::Sound { .. } => {
            // Check if there are enough arguments to parse
            if values.len() < 6 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse sound: not enough arguments: {}/6 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let requires_breathing = values.len() == 7;
            let sound_type = values.remove(0).to_string();
            let Ok(sound_range) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse sound: sound range: {value}");
                return None;
            };
            let Ok(sound_interval) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse sound: sound interval: {value}");
                return None;
            };
            if requires_breathing {
                // Remove the breathing value
                values.remove(0);
            }
            let third_person = values.remove(0).to_string();
            let first_person = values.remove(0).to_string();
            let out_of_sight = values.remove(0).to_string();
            Some(CasteTag::Sound {
                sound_type,
                sound_range,
                sound_interval,
                requires_breathing,
                third_person,
                first_person,
                out_of_sight,
            })
        }
        CasteTag::SpecificFood { .. } => {
            let Some(food_type) = OBJECT_TOKEN_MAP.get(values.remove(0)) else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse SpecificFood: object type: {value}"
                );
                return None;
            };
            let identifier = values.join(":").to_string();
            Some(CasteTag::SpecificFood {
                food_type: food_type.clone(),
                identifier,
            })
        }
        CasteTag::SyndromeDilutionFactor { .. } => {
            // Check if there are enough arguments to parse
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse syndrome dilution factor: not enough arguments: {}/2 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            let syndrome = values.remove(0).to_string();
            let Ok(percentage) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse syndrome dilution factor: percentage: {value}"
                );
                return None;
            };
            Some(CasteTag::SyndromeDilutionFactor {
                syndrome,
                percentage,
            })
        }
        CasteTag::Tendons { .. } => {
            // `material_state` is a `String` and is at the end of `value`
            // `material` is `simple_value` + the remains of `value`
            // Grab `healing_rate` from the end of `value`
            let Ok(healing_rate) = values.pop().unwrap_or_default().parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse tendons: healing rate: {value}");
                return None;
            };
            // Set `material` to `simple_value` + the remains of `value`
            let material = values.join(":").to_string();
            Some(CasteTag::Tendons {
                material,
                healing_rate,
            })
        }
        CasteTag::TissueLayer { .. } => {
            let body_part_selector = values.remove(0).to_string();
            let body_part = values.remove(0).to_string();
            let tissue = values.remove(0).to_string();
            let location = values.join(":").to_string();
            Some(CasteTag::TissueLayer {
                body_part_selector,
                body_part,
                tissue,
                location,
            })
        }
        CasteTag::TissueLayerUnder { .. } => {
            let body_part_selector = values.remove(0).to_string();
            let body_part = values.remove(0).to_string();
            let tissue = values.join(":").to_string();
            Some(CasteTag::TissueLayerUnder {
                body_part_selector,
                body_part,
                tissue,
            })
        }
        CasteTag::VerminBite { .. } => {
            let Ok(chance) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse vermin bite: chance: {value}");
                return None;
            };
            let verb = values.remove(0).to_string();
            let material = values.remove(0).to_string();
            let material_state = values.join(":").to_string();
            Some(CasteTag::VerminBite {
                chance,
                verb,
                material,
                material_state,
            })
        }
        CasteTag::VisionArc { .. } => {
            let Ok(binocular) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: Cannot parse vision arc: binocular: {value}");
                return None;
            };
            let Ok(non_binocular) = values.remove(0).parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse vision arc: non binocular: {value}"
                );
                return None;
            };
            Some(CasteTag::VisionArc {
                binocular,
                non_binocular,
            })
        }
        _ => {
            tracing::error!("parse_complex_token: cannot parse {:?}", token);
            None
        }
    }
}
