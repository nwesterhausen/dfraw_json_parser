use crate::{
    metadata::{TagComplexity, OBJECT_TOKEN_MAP},
    raw_definitions::CASTE_TOKENS,
    tags::CasteTag,
    traits::TagOperations,
};

impl TagOperations for CasteTag {
    #[allow(clippy::too_many_lines)]
    fn get_complexity(&self) -> TagComplexity {
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
                TagComplexity::Complex
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
                TagComplexity::Simple
          }
          _ => {
                tracing::trace!("get_complexity: {self:?} is 'None'");
                TagComplexity::None
          }
        }
    }

    fn parse(key: &str, value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let Some(token) = CASTE_TOKENS.get(key) else {
            tracing::error!("parse_token: unknown token: {}", key);
            return None;
        };

        match token.get_complexity() {
            TagComplexity::None => Some(token.clone()),
            TagComplexity::Simple => {
                // All of these tokens have a pattern of `key:value` so we can parse `value` as appropriate
                // We just pass this off to the token's `simple_parse` method to handle the parsing
                token.parse_simple_token(value)
            }
            TagComplexity::Complex => {
                // These tokens have a variable number of arguments, so we need to parse them differently
                // We pass this off to the token's `complex_parse` method to handle the parsing
                let values = value.split(':').collect::<Vec<&str>>();
                token.parse_complex_token(&values)
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
    fn parse_simple_token(&self, value: &str) -> Option<CasteTag> {
        match self {
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
                tracing::error!("parse_simple_token: Cannot parse token (not simple): {self:?}");
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
    fn parse_complex_token(&self, values: &[&str]) -> Option<CasteTag> {
        match self {
            CasteTag::ApplyCreatureVariation { .. } => {
                // check if there are enough arguments
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse ApplyCreatureVariation: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let id = values[0].to_string();
                let args = values[1..]
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
                    "parse_complex_token: Cannot parse Attack: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let name = values[0].to_string();
                let body_part = values[1..].join(":").to_string();
                Some(CasteTag::Attack { name, body_part })
            }
            CasteTag::AttackTrigger { .. } => {
                // Appears as `ATTACK_TRIGGER:0:1:2` for population, exported_wealth and created_wealth
                // check if there are enough arguments
                if values.len() < 3 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse AttackTrigger: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(population) = values[0].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse AttackTrigger: population: {values:?}"
                    );
                    return None;
                };
                let Ok(exported_wealth) = values[1].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse AttackTrigger: exported wealth: {values:?}"
                );
                    return None;
                };
                let Ok(created_wealth) = values[2].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse AttackTrigger: created wealth: {values:?}"
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
                    "parse_complex_token: Cannot parse BabyName: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let singular = values[0].to_string();
                let plural = values[1..].join(":").to_string();
                Some(CasteTag::BabyName { singular, plural })
            }
            CasteTag::Blood { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse Blood: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                let material = values[0].to_string();
                let state = values[1..].join(":").to_string();
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
                    "parse_complex_token: Cannot parse BodyAppearanceModifier: not enough arguments: {}/8 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let attribute = values[0].to_string();
                let Ok(lowest) = values[1].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lowest: {values:?}"
                );
                    return None;
                };
                let Ok(lower) = values[2].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower: {values:?}"
                );
                    return None;
                };
                let Ok(lower_median) = values[3].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower medium: {values:?}"
                );
                    return None;
                };
                let Ok(median) = values[4].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper_median) = values[5].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper) = values[6].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper: {values:?}"
                );
                    return None;
                };
                let Ok(highest) = values[7].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: highest: {values:?}"
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
                    "parse_complex_token: Cannot parse BodyPartAppearanceModifier: not enough arguments: {}/8 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let quality = values[0].to_string();
                let Ok(lowest) = values[1].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lowest: {values:?}"
                );
                    return None;
                };
                let Ok(lower) = values[2].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower: {values:?}"
                );
                    return None;
                };
                let Ok(lower_median) = values[3].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower medium: {values:?}"
                );
                    return None;
                };
                let Ok(median) = values[4].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper_median) = values[5].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper) = values[6].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper: {values:?}"
                );
                    return None;
                };
                let Ok(highest) = values[7].parse::<i32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: highest: {values:?}"
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
                let body_plan = values[0].to_string();
                let arguments = values[1..]
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
                    "parse_complex_token: Cannot parse body size: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(year) = values[0].parse::<u32>() else {
                    tracing::warn!("parse_complex_token: Cannot parse body size: year: {values:?}");
                    return None;
                };
                let Ok(days) = values[1].parse::<u32>() else {
                    tracing::warn!("parse_complex_token: Cannot parse body size: days: {values:?}");
                    return None;
                };
                let Ok(size) = values[2].parse::<u32>() else {
                    tracing::warn!("parse_complex_token: Cannot parse body size: size: {values:?}");
                    return None;
                };
                Some(CasteTag::BodySize { year, days, size })
            }
            CasteTag::ChildName { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse ChildName: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let singular = values[0].to_string();
                let plural = values[1..].join(":").to_string();
                Some(CasteTag::ChildName { singular, plural })
            }
            CasteTag::ClutchSize { .. } => {
                // Two `u32`s
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse clutch size: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(min) = values[0].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse clutch size: min: {values:?}"
                    );
                    return None;
                };
                let Ok(max) = values[1].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse clutch size: max: {values:?}"
                    );
                    return None;
                };
                Some(CasteTag::ClutchSize { min, max })
            }
            CasteTag::Color { .. } => {
                if values.len() < 3 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse color: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(foreground) = values[0].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse color: foreground: {values:?}"
                    );
                    return None;
                };
                let Ok(background) = values[1].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse color: background: {values:?}"
                    );
                    return None;
                };
                let Ok(brightness) = values[2].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse color: brightness: {values:?}"
                    );
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
                    "parse_complex_token: Cannot parse egg material: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                // Take the last `String` as the `state` and the rest as the `material`
                let state = values[values.len() - 1].to_string();
                let material = values[..values.len() - 1].join(":").to_string();
                Some(CasteTag::EggMaterial { material, state })
            }
            CasteTag::ExtraButcherObject { .. } => {
                // `String` and `Vec<String>`
                let object_type = values[0].to_string();
                let arguments = values[1..]
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
                    "parse_complex_token: Cannot parse extra butcher object item: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                // Two strings
                let item = values[0].to_string();
                let material = values[1..].join(":").to_string();
                Some(CasteTag::ExtraButcherObjectItem { item, material })
            }
            CasteTag::GeneralMaterialForceMultiplier { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse general material force multiplier: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                // Two `u32`s
                let Ok(value_a) = values[0].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse general material force multiplier: {values:?}"
                );
                    return None;
                };
                let Ok(value_b) = values[1].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse general material force multiplier: {values:?}"
                );
                    return None;
                };
                Some(CasteTag::GeneralMaterialForceMultiplier { value_a, value_b })
            }
            CasteTag::GlowColor { .. } => {
                // Arguments become 3 `u32`s, separated by `:`
                if values.len() < 3 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse glow color: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(foreground) = values[0].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse glow color: foreground: {values:?}"
                    );
                    return None;
                };
                let Ok(background) = values[1].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse glow color: background: {values:?}"
                    );
                    return None;
                };
                let Ok(brightness) = values[2].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse glow color: brightness: {values:?}"
                    );
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
                    "parse_complex_token: Cannot parse gobble vermin creature: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                // Two strings
                let vermin_creature = values[0].to_string();
                let vermin_caste = values[1..].join(":").to_string();
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
                let item = values[0].to_string();
                let material = values[1..].join(":").to_string();
                Some(CasteTag::ItemCorpse { item, material })
            }
            CasteTag::Lair { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse lair: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                // `String` and `u32`
                let lair = values[0].to_string();
                let Ok(probability) = values[1].parse::<u32>() else {
                    tracing::warn!("parse_complex_token: Cannot parse lair proability: {values:?}");
                    return None;
                };
                Some(CasteTag::Lair { lair, probability })
            }
            CasteTag::LaysUnusualEggs { .. } => {
                // Two strings
                let item = values[0].to_string();
                let material = values[1..].join(":").to_string();
                Some(CasteTag::LaysUnusualEggs { item, material })
            }
            CasteTag::Ligaments { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse ligaments: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Grab `healing_rate` from the end of `value`
                let Ok(healing_rate) = values[values.len() - 1].parse() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse ligaments: healing rate: {values:?}"
                    );
                    return None;
                };
                // The rest of the arguments are the `material`
                let material = values[..values.len() - 1].join(":").to_string();
                Some(CasteTag::Ligaments {
                    material,
                    healing_rate,
                })
            }
            CasteTag::LitterSize { .. } => {
                // Two `u32`s
                let Ok(min) = values[0].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse litter size: min: {values:?}"
                    );
                    return None;
                };
                let Ok(max) = values[1].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse litter size: max: {values:?}"
                    );
                    return None;
                };
                Some(CasteTag::LitterSize { min, max })
            }
            CasteTag::MannerismFingers { .. } => {
                let finger = values[0].to_string();
                let fingers = values[1..].join(":").to_string();
                Some(CasteTag::MannerismFingers { finger, fingers })
            }
            CasteTag::MaxAge { .. } => {
                // Two `u32`s
                let Ok(min) = values[0].parse::<u32>() else {
                    tracing::warn!("parse_complex_token: Cannot parse max age: min: {values:?}");
                    return None;
                };
                let Ok(max) = values[1].parse::<u32>() else {
                    tracing::warn!("parse_complex_token: Cannot parse max age: max: {values:?}");
                    return None;
                };
                Some(CasteTag::MaxAge { min, max })
            }
            CasteTag::MentalAttributeCapPercentage { .. } => {
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute cap percentage: not enough arguments: {}/2 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                let attribute = values[0].to_string();
                let Ok(percentage) = values[1].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute cap percentage: {values:?}"
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
                    "parse_complex_token: Cannot parse mental attribute range: not enough arguments: {}/8 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let attribute = values[0].to_string();
                let Ok(lowest) = values[1].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lowest: {values:?}"
                );
                    return None;
                };
                let Ok(lower) = values[2].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower: {values:?}"
                );
                    return None;
                };
                let Ok(lower_median) = values[3].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: lower medium: {values:?}"
                );
                    return None;
                };
                let Ok(median) = values[4].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper_median) = values[5].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper) = values[6].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: upper: {values:?}"
                );
                    return None;
                };
                let Ok(highest) = values[7].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute range: highest: {values:?}"
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
                    "parse_complex_token: Cannot parse mental attribute rate: not enough arguments: {}/5 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let attribute = values[0].to_string();
                let Ok(improvement_cost) = values[1].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: improvement cost: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_unused) = values[2].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: decay rate unused: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_rusty) = values[3].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: decay rate rusty: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_demotion) = values[4].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse mental attribute rate: decay rate demotion: {values:?}"
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
                    "parse_complex_token: Cannot parse milkable: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Frequency is the last argument, parsed as `u32`
                let Ok(frequency) = values[values.len() - 1].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse milkable: frequency: {values:?}"
                    );
                    return None;
                };
                // Material is the rest of the arguments, joined as a `String`
                let material = values[..values.len() - 1].join(":").to_string();
                Some(CasteTag::Milkable {
                    material,
                    frequency,
                })
            }
            CasteTag::Name { .. } => {
                if values.len() < 3 {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse name: not enough arguments: {}/3 '{:?}'",
                        values.len(),
                        values
                    );
                    return None;
                }
                let singular = values[0].to_string();
                let plural = values[1].to_string();
                let adjective = values[2..].join(":").to_string();
                Some(CasteTag::Name {
                    singular,
                    plural,
                    adjective,
                })
            }
            CasteTag::NaturalSkill { .. } => {
                // Grab `level` from the end of `value`
                let Ok(level) = values[values.len() - 1].parse() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse natural skill: level: {values:?}"
                    );
                    return None;
                };
                // The rest of the arguments are the `skill`
                let skill = values[..values.len() - 1].join(":").to_string();
                Some(CasteTag::NaturalSkill { skill, level })
            }
            CasteTag::Personality { .. } => {
                // Check if there are enough arguments to parse
                if values.len() < 4 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse personality: not enough arguments: {}/4 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let personality_trait = values[0].to_string();
                let Ok(low) = values[1].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse personality: low: {values:?}"
                    );
                    return None;
                };
                let Ok(median) = values[2].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse personality: median: {values:?}"
                    );
                    return None;
                };
                let Ok(high) = values[3].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse personality: high: {values:?}"
                    );
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
                    "parse_complex_token: Cannot parse physical attribute cap percentage: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let attribute = values[0].to_string();
                let Ok(percentage) = values[1].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute cap percentage: {values:?}"
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
                    "parse_complex_token: Cannot parse physical attribute range: not enough arguments: {}/8 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let attribute = values[0].to_string();
                let Ok(lowest) = values[1].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: lowest: {values:?}"
                );
                    return None;
                };
                let Ok(lower) = values[2].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: lower: {values:?}"
                );
                    return None;
                };
                let Ok(lower_median) = values[3].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: lower medium: {values:?}"
                );
                    return None;
                };
                let Ok(median) = values[4].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper_median) = values[5].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: upper medium: {values:?}"
                );
                    return None;
                };
                let Ok(upper) = values[6].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: upper: {values:?}"
                );
                    return None;
                };
                let Ok(highest) = values[7].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute range: highest: {values:?}"
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
                    "parse_complex_token: Cannot parse physical attribute rate: not enough arguments: {}/5 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let attribute = values[0].to_string();
                let Ok(improvement_cost) = values[1].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: improvement cost: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_unused) = values[2].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: decay rate unused: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_rusty) = values[3].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: decay rate rusty: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_demotion) = values[4].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse physical attribute rate: decay rate demotion: {values:?}"
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
                    "parse_complex_token: Cannot parse profession name: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let profession = values[0].to_string();
                let singular = values[1].to_string();
                let plural = values[2].to_string();
                Some(CasteTag::ProfessionName {
                    profession,
                    singular,
                    plural,
                })
            }
            CasteTag::Pus { .. } => {
                // Grab `material_state` from the end
                let material_state = values[values.len() - 1].to_string();
                // Set `material` to `simple_value` + the remains of `value`
                let material = values[..values.len() - 1].join(":").to_string();
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
                    "parse_complex_token: Cannot parse RelativeSize: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // `relative_size` is the last argument, parsed as `u32`
                let Ok(relative_size) = values[values.len() - 1].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse RelativeSize: relative size: {values:?}"
                    );
                    return None;
                };
                let body_part_selector = values[0].to_string();
                let body_part = values[1..values.len() - 1].join(":").to_string();
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
                    "parse_complex_token: Cannot parse Remains: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let singular = values[0].to_string();
                let plural = values[1..].join(":").to_string();
                Some(CasteTag::Remains { singular, plural })
            }
            CasteTag::RetractIntoBodyPart { .. } => {
                // check if there are enough arguments
                if values.len() < 6 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse RetractIntoBodyPart: not enough arguments: {}/6 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // We grab the strings from the end first
                let third_person_cancel = values[values.len() - 1].to_string();
                let second_person_cancel = values[values.len() - 2].to_string();
                let third_person = values[values.len() - 3].to_string();
                let second_person = values[values.len() - 4].to_string();
                // Then the body_part_selector
                let body_part_selector = values[0].to_string();
                // And finally the body_part
                let body_part = values[1..values.len() - 4].join(":").to_string();
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
                    "parse_complex_token: Cannot parse RootAround: not enough arguments: {}/4 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let third_person_verb = values[values.len() - 1].to_string();
                let second_person_verb = values[values.len() - 2].to_string();
                let body_part_selector = values[0].to_string();
                let body_part = values[1..values.len() - 2].join(":").to_string();
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
                    "parse_complex_token: Cannot parse Secretion: not enough arguments: {}/6 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let material_token = values[0].to_string();
                let material_state = values[0].to_string();
                let body_part_selector = values[0].to_string();
                // Grab from the end
                let trigger = values[values.len() - 1].to_string();
                let tissue_layer = values[values.len() - 2].to_string();
                let body_part = values[1..values.len() - 2].join(":").to_string();
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
                    "parse_complex_token: Cannot parse SenseCreatureClass: not enough arguments: {}/5 '{values:?}'",
                    values.len(),
                );
                    return None;
                }
                let creature_class = values[0].to_string();
                let tile = values[1].to_string();
                let Ok(foreground) = values[2].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse SenseCreatureClass: foreground: {values:?}"
                );
                    return None;
                };
                let Ok(background) = values[3].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse SenseCreatureClass: background: {values:?}"
                );
                    return None;
                };
                let Ok(brightness) = values[4].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse SenseCreatureClass: brightness: {values:?}"
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
                    "parse_complex_token: Cannot parse set body part group: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                // Parse first argument as `String`
                let body_part_selector = values[0].to_string();
                let body_part = values[1..].join(":").to_string();
                Some(CasteTag::SetBodyPartGroup {
                    body_part_selector,
                    body_part,
                })
            }
            CasteTag::SkillRates { .. } => {
                // Check if there are enough arguments to parse
                if values.len() < 4 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: not enough arguments: {}/4 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(improvement_rate) = values[0].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: improvement rate: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_unused) = values[1].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: decay rate unused: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_rusty) = values[2].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: decay rate rusty: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_demotion) = values[3].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rates: decay rate demotion: {values:?}"
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
                    "parse_complex_token: Cannot parse skill rust rates: not enough arguments: {}/3 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let Ok(decay_rate_unused) = values[0].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rust rates: decay rate unused: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_rusty) = values[1].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rust rates: decay rate rusty: {values:?}"
                );
                    return None;
                };
                let Ok(decay_rate_demotion) = values[2].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse skill rust rates: decay rate demotion: {values:?}"
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
                    "parse_complex_token: Cannot parse sound: not enough arguments: {}/6 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let requires_breathing = values.len() == 7;
                let sound_type = values[0].to_string();
                let Ok(sound_range) = values[1].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse sound: sound range: {values:?}"
                    );
                    return None;
                };
                let Ok(sound_interval) = values[2].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse sound: sound interval: {values:?}"
                    );
                    return None;
                };
                let mut breathing_bump = 0;
                if requires_breathing {
                    // Remove the breathing value
                    breathing_bump = 1;
                }
                let third_person = values[3 + breathing_bump].to_string();
                let first_person = values[4 + breathing_bump].to_string();
                let out_of_sight = values[5 + breathing_bump].to_string();
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
                let Some(food_type) = OBJECT_TOKEN_MAP.get(values[0]) else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse SpecificFood: object type: {values:?}"
                    );
                    return None;
                };
                let identifier = values[1..].join(":").to_string();
                Some(CasteTag::SpecificFood {
                    food_type: food_type.clone(),
                    identifier,
                })
            }
            CasteTag::SyndromeDilutionFactor { .. } => {
                // Check if there are enough arguments to parse
                if values.len() < 2 {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse syndrome dilution factor: not enough arguments: {}/2 '{:?}'",
                    values.len(),
                    values
                );
                    return None;
                }
                let syndrome = values[0].to_string();
                let Ok(percentage) = values[1].parse::<u32>() else {
                    tracing::warn!(
                    "parse_complex_token: Cannot parse syndrome dilution factor: percentage: {values:?}"
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
                let Ok(healing_rate) = values[values.len() - 1].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse tendons: healing rate: {values:?}"
                    );
                    return None;
                };
                // Set `material` to `simple_value` + the remains of `value`
                let material = values[..values.len() - 1].join(":").to_string();
                Some(CasteTag::Tendons {
                    material,
                    healing_rate,
                })
            }
            CasteTag::TissueLayer { .. } => {
                let body_part_selector = values[0].to_string();
                let body_part = values[1].to_string();
                let tissue = values[2].to_string();
                let location = values[3..].join(":").to_string();
                Some(CasteTag::TissueLayer {
                    body_part_selector,
                    body_part,
                    tissue,
                    location,
                })
            }
            CasteTag::TissueLayerUnder { .. } => {
                let body_part_selector = values[0].to_string();
                let body_part = values[1].to_string();
                let tissue = values[2..].join(":").to_string();
                Some(CasteTag::TissueLayerUnder {
                    body_part_selector,
                    body_part,
                    tissue,
                })
            }
            CasteTag::VerminBite { .. } => {
                let Ok(chance) = values[0].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse vermin bite: chance: {values:?}"
                    );
                    return None;
                };
                let verb = values[1].to_string();
                let material = values[2].to_string();
                let material_state = values[3..].join(":").to_string();
                Some(CasteTag::VerminBite {
                    chance,
                    verb,
                    material,
                    material_state,
                })
            }
            CasteTag::VisionArc { .. } => {
                let Ok(binocular) = values[0].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse vision arc: binocular: {values:?}"
                    );
                    return None;
                };
                let Ok(non_binocular) = values[1].parse::<u32>() else {
                    tracing::warn!(
                        "parse_complex_token: Cannot parse vision arc: non binocular: {values:?}"
                    );
                    return None;
                };
                Some(CasteTag::VisionArc {
                    binocular,
                    non_binocular,
                })
            }
            _ => {
                tracing::error!("parse_complex_token: cannot parse {self:?}");
                None
            }
        }
    }
}
