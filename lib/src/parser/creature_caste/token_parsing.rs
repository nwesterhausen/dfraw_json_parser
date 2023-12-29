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

    fn parse_token(key: &str, value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let Some(token) = TOKEN_MAP.get(key) else {
            tracing::warn!("Unknown token: {}", key);
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
            let Ok(age) = value.parse::<u32>() else {
                tracing::warn!("parse_simple_token: Cannot parse baby age: {}", value);
                return None;
            };
            Some(CasteTag::Baby { age })
        }
        CasteTag::BeachFrequency { .. } => {
            let Ok(frequency) = value.parse::<u32>() else {
                tracing::warn!(
                    "parse_simple_token: Cannot parse beach frequency: {}",
                    value
                );
                return None;
            };
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
            let Ok(building_destroyer) = value.parse::<u32>() else {
                tracing::warn!(
                    "parse_simple_token: Cannot parse building destroyer: {}",
                    value
                );
                return None;
            };
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
            let Ok(percent) = value.parse::<u32>() else {
                tracing::warn!(
                    "parse_simple_token: Cannot parse body size percent: {}",
                    value
                );
                return None;
            };
            Some(CasteTag::ChangeBodySizePercent { percent })
        }
        CasteTag::Child { .. } => {
            let Ok(age) = value.parse::<u32>() else {
                tracing::warn!("parse_simple_token: Cannot parse child age: {}", value);
                return None;
            };
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
            let Ok(size) = value.parse::<u32>() else {
                tracing::warn!("parse_simple_token: Cannot parse egg size: {}", value);
                return None;
            };
            Some(CasteTag::EggSize { size })
        }
        CasteTag::Extract { .. } => {
            let material = String::from(value);
            Some(CasteTag::Extract { material })
        }
        CasteTag::FixedTemp { .. } => {
            let Ok(temperature) = value.parse::<i32>() else {
                tracing::warn!("parse_simple_token: Cannot parse fixed temp: {}", value);
                return None;
            };
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
            let Ok(target) = value.parse::<u32>() else {
                tracing::warn!(
                    "parse_simple_token: Cannot parse gravitate body size: {}",
                    value
                );
                return None;
            };
            Some(CasteTag::GravitateBodySize { target })
        }
        CasteTag::Grazer { .. } => {
            let Ok(grazer) = value.parse::<u32>() else {
                tracing::warn!("parse_simple_token: Cannot parse grazer: {}", value);
                return None;
            };
            Some(CasteTag::Grazer { grazer })
        }
        CasteTag::Habit { .. } => {
            let habit = String::from(value);
            Some(CasteTag::Habit { habit })
        }
        CasteTag::HabitNumber { .. } => {
            let Ok(number) = value.parse::<u32>() else {
                tracing::warn!("parse_simple_token: Cannot parse habit number: {}", value);
                return None;
            };
            Some(CasteTag::HabitNumber { number })
        }
        CasteTag::Homeotherm { .. } => {
            let temperature = match value.parse::<u32>() {
                Ok(temperature) => Some(temperature),
                Err(_) => None,
            };
            Some(CasteTag::Homeotherm { temperature })
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
            let Ok(odor_level) = value.parse::<u32>() else {
                tracing::warn!("parse_simple_token: Cannot parse odor level: {}", value);
                return None;
            };
            Some(CasteTag::OdorLevel { odor_level })
        }
        CasteTag::OdorString { .. } => {
            let odor_string = String::from(value);
            Some(CasteTag::OdorString { odor_string })
        }
        CasteTag::PenetratePower { .. } => {
            let Ok(penetrate_power) = value.parse::<u32>() else {
                tracing::warn!(
                    "parse_simple_token: Cannot parse penetrate power: {}",
                    value
                );
                return None;
            };
            Some(CasteTag::PenetratePower { penetrate_power })
        }
        CasteTag::PetValue { .. } => {
            let Ok(pet_value) = value.parse::<u32>() else {
                tracing::warn!("parse_simple_token: Cannot parse pet value: {}", value);
                return None;
            };
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
            tracing::debug!(
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
    match token {
        CasteTag::ApplyCreatureVariation { .. } => {
            // This token has an identifier and then a list of argument values, delimited by `:`
            // The first argument is the identifier, and the rest are the arguments
            let mut args = value.split(':');
            let Some(identifier) = args.next() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse apply creature variation 'id': {}",
                    value
                );
                return None;
            };
            let arguments = args.collect::<Vec<&str>>();
            Some(CasteTag::ApplyCreatureVariation {
                id: String::from(identifier),
                args: arguments.iter().map(|s| String::from(*s)).collect(),
            })
        }
        CasteTag::Attack { .. } => {
            // Appears as `ATTACK:NAME:BODYPART:BY_CATEGORY:HORN`
            // The first argument is the name, the rest is the body part (we include just as a string)
            let mut args = value.split(':');
            let Some(name) = args.next() else {
                tracing::warn!("parse_complex_token: Cannot parse attack name: {}", value);
                return None;
            };
            let body_part = args.collect::<Vec<&str>>().join(":");
            Some(CasteTag::Attack {
                name: String::from(name),
                body_part,
            })
        }
        CasteTag::AttackTrigger { .. } => {
            // Appears as `ATTACK_TRIGGER:0:1:2` for population, exported_wealth and created_wealth
            // The arguments are u32s, so we parse them as such
            let mut args = value.split(':');
            let Some(population) = args.next() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse attack trigger population: {}",
                    value
                );
                return None;
            };
            let Some(exported_wealth) = args.next() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse attack trigger exported wealth: {}",
                    value
                );
                return None;
            };
            let Some(created_wealth) = args.next() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse attack trigger created wealth: {}",
                    value
                );
                return None;
            };
            let Ok(population) = population.parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse attack trigger population: {}",
                    value
                );
                return None;
            };
            let Ok(exported_wealth) = exported_wealth.parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse attack trigger exported wealth: {}",
                    value
                );
                return None;
            };
            let Ok(created_wealth) = created_wealth.parse::<u32>() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse attack trigger created wealth: {}",
                    value
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
            // Arguments become a singular name and plural name, separated by `:`
            let mut args = value.split(':');
            let Some(singular) = args.next() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse baby name singular: {}",
                    value
                );
                return None;
            };
            let Some(plural) = args.next() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse baby name plural: {}",
                    value
                );
                return None;
            };
            Some(CasteTag::BabyName {
                singular: String::from(singular),
                plural: String::from(plural),
            })
        }
        CasteTag::Body { .. } => {
            // Arguments need to be turned into a vector of strings
            let body_parts = value.split(':').map(String::from).collect();
            Some(CasteTag::Body { body_parts })
        }
        CasteTag::Blood { .. } => {
            // Arguments become two strings, material and state; separated by `:`
            let mut args = value.split(':');
            let Some(material) = args.next() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse blood material: {}",
                    value
                );
                return None;
            };
            let Some(state) = args.next() else {
                tracing::warn!("parse_complex_token: Cannot parse blood state: {}", value);
                return None;
            };
            Some(CasteTag::Blood {
                material: String::from(material),
                state: String::from(state),
            })
        }
        CasteTag::BodyAppearanceModifier { .. } => {
            // Arguments become a string (attribute) and 7 i32s, separated by `:`
            let mut args = value.split(':');
            let Some(attribute) = args.next() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse body appearance modifier attribute: {}",
                    value
                );
                return None;
            };
            let values = args
                .filter_map(|s| match s.parse::<i32>() {
                    Ok(value) => Some(value),
                    Err(_) => None,
                })
                .collect::<Vec<i32>>();
            // Confirm we have 7 values
            if values.len() != 7 {
                tracing::warn!("parse_complex_token: Cannot parse body appearance modifier values {} instead of 7: {}", values.len(), value);
                return None;
            }
            let Ok(values) = values.try_into() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse body appearance modifier values: {}",
                    value
                );
                return None;
            };
            Some(CasteTag::BodyAppearanceModifier {
                attribute: String::from(attribute),
                values,
            })
        }
        CasteTag::BodyDetailPlan { .. } => {
            // Arguments become a string (body_plan) and string arguments (arguments); separated by `:`
            let mut args = value.split(':');
            let Some(body_plan) = args.next() else {
                tracing::warn!(
                    "parse_complex_token: Cannot parse body detail plan body plan: {}",
                    value
                );
                return None;
            };
            let arguments = args.map(String::from).collect::<Vec<String>>();
            Some(CasteTag::BodyDetailPlan {
                body_plan: String::from(body_plan),
                arguments,
            })
        }
        _ => todo!(),
    }
}
