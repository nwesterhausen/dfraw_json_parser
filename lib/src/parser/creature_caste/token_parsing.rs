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
          CasteTag::BodySize {..}|
            CasteTag::ChildName { .. } |
            CasteTag::ClutchSize { .. } |
            CasteTag::Color { .. } |
            CasteTag::EggMaterial { .. } |
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
          => {
                tracing::trace!("get_complexity: {self:?} is 'Complex'");
                TokenComplexity::Complex
        }
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
          CasteTag::Webber { .. } => {
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
                let simple_value = key.split(':').nth(1).unwrap_or_default();
                parse_simple_token(token, simple_value)
            }
            TokenComplexity::Complex => {
                // These tokens have a variable number of arguments, so we need to parse them differently
                // We pass this off to the token's `complex_parse` method to handle the parsing
                let simple_value = key.split(':').nth(1).unwrap_or_default();
                parse_complex_token(token, simple_value, value)
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
fn parse_complex_token(token: &CasteTag, simple_value: &str, value: &str) -> Option<CasteTag> {
    match token {
        CasteTag::ApplyCreatureVariation { .. } => {
            // This token has an identifier and then a list of argument values, delimited by `:`
            // Parse first argument as `String`
            let identifier = simple_value.to_string();
            // Parse remaining arguments as `Vec<String>`
            let remaining_args = value.split(':').map(String::from).collect::<Vec<String>>();
            Some(CasteTag::ApplyCreatureVariation {
                id: identifier,
                args: remaining_args,
            })
        }
        CasteTag::Attack { .. } => {
            // Appears as `ATTACK:NAME:BODYPART:BY_CATEGORY:HORN`
            // Parse first argument as `String`
            let name = simple_value.to_string();
            // Parse second argument as `String`
            let body_part = value.to_string();
            Some(CasteTag::Attack { name, body_part })
        }
        CasteTag::AttackTrigger { .. } => {
            // Appears as `ATTACK_TRIGGER:0:1:2` for population, exported_wealth and created_wealth
            // Parse first argument as `u32`
            let population = simple_value.parse::<u32>().unwrap_or_default();
            // Parse the remaining as `Vec<u32>` by splitting on `:` and parsing each value
            let mut values = value
                .split(':')
                .map(|s| s.parse::<u32>().unwrap_or_default())
                .collect::<Vec<u32>>();
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse attack trigger: not enough arguments: {}/3 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // Set the first argument as `u32`
            let exported_wealth = values.remove(0);
            // Set the second argument as `u32`
            let created_wealth = values.remove(0);
            Some(CasteTag::AttackTrigger {
                population,
                exported_wealth,
                created_wealth,
            })
        }
        CasteTag::BabyName { .. } => {
            // Parse first argument as `String`
            let singular = simple_value.to_string();
            // Parse second argument as `String`
            let plural = value.to_string();
            Some(CasteTag::BabyName { singular, plural })
        }
        CasteTag::Body { .. } => {
            // Parse first argument as `String`
            let body_part_0 = simple_value.to_string();
            // Arguments need to be turned into a vector of strings
            let mut body_parts = vec![body_part_0];
            body_parts.extend(value.split(':').map(String::from));
            Some(CasteTag::Body { body_parts })
        }
        CasteTag::Blood { .. } => {
            // Parse first argument as `String`
            let material = simple_value.to_string();
            // Parse second argument as `String`
            let state = value.to_string();
            Some(CasteTag::Blood { material, state })
        }
        CasteTag::BodyAppearanceModifier { .. } => {
            // Arguments become a string (attribute) and 7 i32s, separated by `:`
            // Parse first argument as `String`
            let attribute = simple_value.to_string();
            // Parse remaining arguments as `Vec<i32>`
            let mut values = value
                .split(':')
                .map(|s| s.parse::<i32>().unwrap_or_default())
                .collect::<Vec<i32>>();
            if values.len() < 7 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse body appearance modifier: not enough arguments: {}/8 '{}'",
                    values.len() + 1,
                    value
                );
                return None;
            }
            // Set the first argument as `i32`
            let low = values.remove(0);
            // Set the second argument as `i32`
            let median = values.remove(0);
            // Set the third argument as `i32`
            let high = values.remove(0);
            // Set the fourth argument as `i32`
            let low_variance = values.remove(0);
            // Set the fifth argument as `i32`
            let high_variance = values.remove(0);
            // Set the sixth argument as `i32`
            let low_cap = values.remove(0);
            // Set the seventh argument as `i32`
            let high_cap = values.remove(0);
            Some(CasteTag::BodyAppearanceModifier {
                attribute,
                values: [
                    low,
                    median,
                    high,
                    low_variance,
                    high_variance,
                    low_cap,
                    high_cap,
                ],
            })
        }
        CasteTag::BodyDetailPlan { .. } => {
            // Arguments become a string (body_plan) and string arguments (arguments); separated by `:`
            // Parse first argument as `String`
            let body_plan = simple_value.to_string();
            // Parse remaining arguments as `Vec<String>`
            let arguments = value.split(':').map(String::from).collect::<Vec<String>>();
            Some(CasteTag::BodyDetailPlan {
                body_plan,
                arguments,
            })
        }
        CasteTag::Name { .. } => {
            // Arguments become a singular name, plural name, and adjective, separated by `:`
            // Parse first argument as `String`
            let singular = simple_value.to_string();
            // Parse second argument as `Vec<String>`
            let mut args = value.split(':').map(String::from).collect::<Vec<String>>();
            if args.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse name: not enough arguments: {}/3 '{}'",
                    args.len() + 1,
                    value
                );
                return None;
            }
            // Set the second argument as `String`
            let plural = args.remove(0);
            // Set the third argument as `String`
            let adjective = args.join(":");
            Some(CasteTag::Name {
                singular,
                plural,
                adjective,
            })
        }
        CasteTag::NaturalSkill { .. } => {
            // Parse first argument as `String`
            let skill = simple_value.to_string();
            // Parse second argument as `u32`
            let level = value.parse::<u32>().unwrap_or_default();
            Some(CasteTag::NaturalSkill { skill, level })
        }
        CasteTag::Personality { .. } => {
            // Parse first argument as `String`
            let personality_trait = simple_value.to_string();
            // Parse second argument as `Vec<u32>`
            let values = value
                .split(':')
                .map(|s| s.parse::<u32>().unwrap_or_default())
                .collect::<Vec<u32>>();
            // Set the first argument as `u32`
            let low = values.first().copied().unwrap_or_default();
            // Set the second argument as `u32`
            let median = values.get(1).copied().unwrap_or_default();
            // Set the third argument as `u32`
            let high = values.get(2).copied().unwrap_or_default();
            Some(CasteTag::Personality {
                personality_trait,
                low,
                median,
                high,
            })
        }
        CasteTag::SkillRates { .. } => {
            // Parse first argument as `u32`
            let improvement_rate = simple_value.parse::<u32>().unwrap_or_default();
            // Parse second argument as `Vec<u32>`
            let values = value
                .split(':')
                .map(|s| s.parse::<u32>().unwrap_or_default())
                .collect::<Vec<u32>>();
            // Set the first argument as `u32`
            let decay_rate_unused = values.first().copied().unwrap_or_default();
            // Set the second argument as `u32`
            let decay_rate_rusty = values.get(1).copied().unwrap_or_default();
            // Set the third argument as `u32`
            let decay_rate_demotion = values.get(2).copied().unwrap_or_default();
            Some(CasteTag::SkillRates {
                improvement_rate,
                decay_rate_unused,
                decay_rate_rusty,
                decay_rate_demotion,
            })
        }
        CasteTag::SkillRustRates { .. } => {
            // Parse first argument as `u32`
            let decay_rate_unused = simple_value.parse::<u32>().unwrap_or_default();
            // Parse second argument as `Vec<u32>`
            let mut values = value
                .split(':')
                .map(|s| s.parse::<u32>().unwrap_or_default())
                .collect::<Vec<u32>>();
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse skill rust rates: not enough arguments: {}/2 '{}'",
                    values.len(),
                    value
                );
                return None;
            }
            // Set the first argument as `u32`
            let decay_rate_rusty = values.remove(0);
            // Set the second argument as `u32`
            let decay_rate_demotion = values.remove(0);
            Some(CasteTag::SkillRustRates {
                decay_rate_unused,
                decay_rate_rusty,
                decay_rate_demotion,
            })
        }
        CasteTag::Sound { .. } => {
            // Parse first argument as `String`
            let sound_type = String::from(simple_value);
            // Parse second argument as `Vec<String>`
            let mut args = value.split(':').map(String::from).collect::<Vec<String>>();
            if args.len() < 6 {
                tracing::warn!(
                    "parse_complex_token: Cannot parse sound: not enough arguments: {}/7 '{}'",
                    args.len() + 1,
                    value
                );
                return None;
            }
            // Convert the first argument to a `u32`
            let sound_interval = args.remove(0).parse::<u32>().unwrap_or_default();
            // Convert the second argument to a `u32`
            let sound_range = args.remove(0).parse::<u32>().unwrap_or_default();
            // Convert the third argument to a `bool`
            let requires_breathing = args.remove(0).parse::<bool>().unwrap_or_default();
            // Convert the fourth argument to a `String`
            let third_person = args.remove(0);
            // Convert the fifth argument to a `String`
            let first_person = args.remove(0);
            // Convert the sixth argument to a `String`
            let out_of_sight = args.join(":");
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
        CasteTag::SyndromeDilutionFactor { .. } => {
            // Parse first argument as `String`
            let syndrome = String::from(simple_value);
            // Parse second argument as `u32`
            let percentage = value.parse::<u32>().ok().unwrap_or_default();
            Some(CasteTag::SyndromeDilutionFactor {
                syndrome,
                percentage,
            })
        }
        CasteTag::TissueLayer { .. } => {
            // Parse first argument as `String`
            let body_part_selector = String::from(simple_value);
            // Parse second argument as `Vec<String>`
            let mut args = value.split(':').map(String::from).collect::<Vec<String>>();
            let body_part = args.remove(0);
            let tissue = args.remove(0);
            let location = args.join(":");
            Some(CasteTag::TissueLayer {
                body_part_selector,
                body_part,
                tissue,
                location,
            })
        }
        CasteTag::TissueLayerUnder { .. } => {
            // Parse first argument as `String`
            let body_part_selector = String::from(simple_value);
            // Parse second argument as `Vec<String>`
            let mut args = value.split(':').map(String::from).collect::<Vec<String>>();
            let body_part = args.remove(0);
            let tissue = args.join(":");
            Some(CasteTag::TissueLayerUnder {
                body_part_selector,
                body_part,
                tissue,
            })
        }
        CasteTag::VerminBite { .. } => {
            // Parse first argument as `u32`
            let chance = simple_value.parse::<u32>().ok().unwrap_or_default();
            // Parse second argument as `Vec<String>`
            let mut args = value.split(':').map(String::from).collect::<Vec<String>>();
            let verb = args.remove(0);
            let material = args.remove(0);
            let material_state = args.join(":");
            Some(CasteTag::VerminBite {
                chance,
                verb,
                material,
                material_state,
            })
        }
        CasteTag::VisionArc { .. } => {
            // Parse first argument as `u32`
            let binocular = simple_value.parse::<u32>().ok().unwrap_or_default();
            // Parse second argument as `u32`
            let non_binocular = value.parse::<u32>().ok().unwrap_or_default();
            Some(CasteTag::VisionArc {
                binocular,
                non_binocular,
            })
        }
        CasteTag::MaxAge { .. } => {
            // Parse first argument as `u32`
            let min = simple_value.parse::<u32>().ok().unwrap_or_default();
            // Parse second argument as `u32`
            let max = value.parse::<u32>().ok().unwrap_or_default();
            Some(CasteTag::MaxAge { min, max })
        }
        CasteTag::ClutchSize { .. } => {
            // Parse first argument as `u32`
            let min = simple_value.parse::<u32>().ok().unwrap_or_default();
            // Parse second argument as `u32`
            let max = value.parse::<u32>().ok().unwrap_or_default();
            Some(CasteTag::ClutchSize { min, max })
        }
        CasteTag::EggMaterial { .. } => {
            // Need to combine the two arguments into a single `Vec<String>`
            let mut raw_args: Vec<String> = format!("{simple_value}:{value}")
                .split(':')
                .map(String::from)
                .collect();
            // The state is the last element in the vector
            let state = raw_args.pop().unwrap_or_default();
            // The material is a combined string of the remaining elements
            let material = raw_args.join(":");
            Some(CasteTag::EggMaterial { material, state })
        }
        _ => {
            tracing::error!("parse_complex_token: cannot parse {:?}", token);
            None
        }
    }
}
