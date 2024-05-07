use super::{Token as CreatureTag, TOKEN_MAP};
use crate::parser::metadata::{RawObjectToken, TokenComplexity};

#[typetag::serde]
impl RawObjectToken for CreatureTag {
    fn get_complexity(&self) -> TokenComplexity {
        // Implement the logic for getting the complexity of the token.
        // Return the appropriate `TokenComplexity` variant.
        match self {
          Self::ApplyCurrentCreatureVariation |
          Self::ArtificialHiveable |
          Self::DoesNotExist |
          Self::EquipmentWagon |
          Self::Evil |
          Self::Fanciful |
          Self::Generated |
          Self::Good |
          Self::GoToEnd |
          Self::GoToStart |
          Self::LargeRoaming |
          Self::LocalPopsControllable |
          Self::LocalPopsProduceHeroes |
          Self::LooseClusters |
          Self::Mundane |
          Self::Savage |
          Self::Ubiquitous |
          Self::Utterances |
          Self::VerminEater |
          Self::VerminFish |
          Self::VerminGrounder |
          Self::VerminRotter |
          Self::VerminSoil |
          Self::VerminSoilColony |
          // Other tags
          Self::Unknown |
          Self::AllCastesAlive |
          Self::Equipment |
          Self::MatesToBreed |
          Self::OccursAsEntityRace |
          Self::SmallRace |
          Self::TwoGenders
          => {
                tracing::trace!("get_complexity: {self:?} is 'None'");
                TokenComplexity::None
        }
          Self::AltTile{..} |
          Self::Biome {..} |
          Self::Caste {..} |
          Self::ChangeFrequencyPercent {..} |
          Self::CopyTagsFrom {..} |
          Self::CreatureSoldierTile {..} |
          Self::CreatureTile {..} |
          Self::Frequency { .. } |
          Self::GlowTile {..} |
          Self::GoToTag {..} |
          Self::PlusMaterial {..} |
          Self::PrefString {..} |
          Self::RemoveMaterial {..} |
          Self::RemoveTissue {..} |
          Self::SelectAdditionalCaste {..} |
          Self::SelectCaste {..} |
          Self::SelectMaterial {..} |
          Self::SelectTissue {..} |
          Self::SlainSpeech {..} |
          Self::SmellTrigger {..} |
          Self::SoldierAltTile {..} |
          Self::SourceHfid {..} |
          Self::Sphere {..} |
          Self::Tissue {..}
          => {
                tracing::trace!("get_complexity: {self:?} is 'Simple'");
                TokenComplexity::Simple
        }
          Self::ApplyCreatureVariation{..} |
          Self::ClusterNumber {..} |
          Self::Color {..} |
          Self::GlowColor {..} |
          Self::GeneralBabyName {..} |
          Self::GeneralChildName {..} |
          Self::HarvestProduct {..} |
          Self::Name {..} |
          Self::PopulationNumber {..} |
          Self::ProfessionName {..} |
Self::TriggerableGroup { .. } |
          Self::UndergroundDepth {..} |
          Self::UseCaste {..} |
          Self::UseMaterial {..} |
          Self::UseMaterialTemplate {..} |
          Self::UseTissue {..} |
          Self::UseTissueTemplate {..}
          => {
                tracing::trace!("get_complexity: {self:?} is 'Complex'");
                TokenComplexity::Complex
        }
        }
    }

    fn parse_token(key: &str, value: &str) -> Option<Self> {
        // Implement the logic for parsing the token from the key and value.
        // Create a new `CreatureTag` instance and return it, or return `None` if the token could not be parsed.

        let Some(token) = TOKEN_MAP.get(key) else {
            tracing::warn!("parse_token: unknown token: {key}");
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

/// Parse a simple token
///
/// This function is used to parse tokens that have a simple key:value pattern
///
/// # Arguments
///
/// * `token` - The token to parse
/// * `value` - The value to parse
#[allow(clippy::too_many_lines)]
fn parse_simple_token(token: &CreatureTag, value: &str) -> Option<CreatureTag> {
    match token {
        CreatureTag::AltTile { .. } => {
            // Parse the value as a `u32`
            let value = value.parse::<u32>().unwrap_or_default();
            Some(CreatureTag::AltTile { character: value })
        }
        CreatureTag::Biome { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::Biome {
                id: value.to_string(),
            })
        }
        CreatureTag::Caste { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::Caste {
                name: value.to_string(),
            })
        }
        CreatureTag::ChangeFrequencyPercent { .. } => {
            // Parse the value as a `u32`
            let value = value.parse::<u32>().unwrap_or_default();
            Some(CreatureTag::ChangeFrequencyPercent { percent: value })
        }
        CreatureTag::CopyTagsFrom { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::CopyTagsFrom {
                creature: value.to_string(),
            })
        }
        CreatureTag::CreatureSoldierTile { .. } => {
            // Parse the value as a `u32`
            let value = value.parse::<u32>().unwrap_or_default();
            Some(CreatureTag::CreatureSoldierTile { character: value })
        }
        CreatureTag::CreatureTile { .. } => {
            // Parse the value as a `u32`
            let value = value.parse::<u32>().unwrap_or_default();
            Some(CreatureTag::CreatureTile { character: value })
        }
        CreatureTag::Frequency { .. } => {
            // Parse the value as a `u32`
            let value = value.parse::<u32>().unwrap_or_default();
            Some(CreatureTag::Frequency { frequency: value })
        }
        CreatureTag::GlowTile { .. } => {
            // Parse the value as a `u32`
            let value = value.parse::<u32>().unwrap_or_default();
            Some(CreatureTag::GlowTile { character: value })
        }
        CreatureTag::GoToTag { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::GoToTag {
                tag: value.to_string(),
            })
        }
        CreatureTag::PlusMaterial { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::PlusMaterial {
                material: value.to_string(),
            })
        }
        CreatureTag::PrefString { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::PrefString {
                pref_string: value.to_string(),
            })
        }
        CreatureTag::RemoveMaterial { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::RemoveMaterial {
                material: value.to_string(),
            })
        }
        CreatureTag::RemoveTissue { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::RemoveTissue {
                tissue: value.to_string(),
            })
        }
        CreatureTag::SelectAdditionalCaste { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::SelectAdditionalCaste {
                caste: value.to_string(),
            })
        }
        CreatureTag::SelectCaste { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::SelectCaste {
                caste: value.to_string(),
            })
        }
        CreatureTag::SelectMaterial { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::SelectMaterial {
                material: value.to_string(),
            })
        }
        CreatureTag::SelectTissue { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::SelectTissue {
                tissue: value.to_string(),
            })
        }
        CreatureTag::SlainSpeech { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::SlainSpeech {
                slain_speech: value.to_string(),
            })
        }
        CreatureTag::SmellTrigger { .. } => {
            // Parse the value as a `u32`
            let value = value.parse::<u32>().unwrap_or_default();
            Some(CreatureTag::SmellTrigger {
                smell_trigger: value,
            })
        }
        CreatureTag::SoldierAltTile { .. } => Some(CreatureTag::SoldierAltTile {
            tile: value.to_string(),
        }),
        CreatureTag::SourceHfid { .. } => {
            // Parse the value as a `u32`
            let value = value.parse::<u32>().unwrap_or_default();
            Some(CreatureTag::SourceHfid { hfid: value })
        }
        CreatureTag::Sphere { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::Sphere {
                sphere: value.to_string(),
            })
        }
        CreatureTag::Tissue { .. } => {
            // Parse the value as a `String`
            Some(CreatureTag::Tissue {
                name: value.to_string(),
            })
        }
        _ => {
            tracing::warn!("parse_simple_token: non-simple token: {token}");
            None
        }
    }
}

/// Parse a complex token
///
/// This function is used to parse tokens that have a complex key:value pattern
///
/// # Arguments
///
/// * `token` - The token to parse
/// * `argument_1` - The first argument to parse
/// * `remaining_args` - The remaining arguments to parse (as a ':' separated string)
#[allow(clippy::too_many_lines)]
fn parse_complex_token(
    token: &CreatureTag,
    argument_1: &str,
    remaining_args: &str,
) -> Option<CreatureTag> {
    match token {
        CreatureTag::ApplyCreatureVariation { .. } => {
            // Parse the first argument as a `String`
            // Parse the remaining arguments as a `String`
            Some(CreatureTag::ApplyCreatureVariation {
                id: argument_1.to_string(),
                args: remaining_args
                    .split(':')
                    .map(std::string::ToString::to_string)
                    .collect(),
            })
        }
        CreatureTag::ClusterNumber { .. } => {
            // Parse the first argument as a `u32`
            // Parse the remaining arguments as a `u32`
            let value = argument_1.parse::<u32>().unwrap_or_default();
            let value2 = remaining_args.parse::<u32>().unwrap_or_default();
            Some(CreatureTag::ClusterNumber {
                min: value,
                max: value2,
            })
        }
        CreatureTag::Color { .. } => {
            let foreground = argument_1.parse::<u32>().unwrap_or_default();
            // Parse a total of 2 arguments as `u32`
            let values_23: Vec<u32> = remaining_args
                .split(':')
                .map(|s| s.parse::<u32>().unwrap_or_default())
                .collect();
            let background = values_23.first().copied().unwrap_or_default();
            let brightness = values_23.get(1).copied().unwrap_or_default();
            Some(CreatureTag::Color {
                foreground,
                background,
                brightness,
            })
        }
        CreatureTag::GlowColor { .. } => {
            let foreground = argument_1.parse::<u32>().unwrap_or_default();
            // Parse a total of 2 arguments as `u32`
            let values_23: Vec<u32> = remaining_args
                .split(':')
                .map(|s| s.parse::<u32>().unwrap_or_default())
                .collect();
            let background = values_23.first().copied().unwrap_or_default();
            let brightness = values_23.get(1).copied().unwrap_or_default();
            Some(CreatureTag::GlowColor {
                foreground,
                background,
                brightness,
            })
        }
        CreatureTag::GeneralBabyName { .. } => {
            // Parse the first argument as a `String`
            let singular = argument_1.to_string();
            // Parse the remaining arguments as a `String`
            let plural = remaining_args.to_string();
            Some(CreatureTag::GeneralBabyName { singular, plural })
        }
        CreatureTag::GeneralChildName { .. } => {
            // Parse the first argument as a `String`
            let singular = argument_1.to_string();
            // Parse the remaining arguments as a `String`
            let plural = remaining_args.to_string();
            Some(CreatureTag::GeneralChildName { singular, plural })
        }
        CreatureTag::HarvestProduct { .. } => {
            // Parse the first argument as a `u32`
            let number = argument_1.parse::<u32>().unwrap_or_default();
            // Parse the remaining arguments as a `u32` then `Vec<String>`
            let values: Vec<&str> = remaining_args.split(':').collect();
            let time = values
                .first()
                .copied()
                .unwrap_or_default()
                .parse::<u32>()
                .unwrap_or_default();
            let item_tokens = values
                .get(1..)
                .map(|s| s.iter().map(std::string::ToString::to_string).collect())
                .unwrap_or_default();
            Some(CreatureTag::HarvestProduct {
                number,
                time,
                item_tokens,
            })
        }
        CreatureTag::Name { .. } => {
            // Parse the first argument as a `String`
            let name = argument_1.to_string();
            // Parse the remaining arguments as a `Vec<String>`
            let values: Vec<String> = remaining_args
                .split(':')
                .map(std::string::ToString::to_string)
                .collect();
            let plural_name = values.first().cloned().unwrap_or_default();
            let adjective = values.get(1).cloned().unwrap_or_default();
            Some(CreatureTag::Name {
                name,
                plural_name,
                adjective,
            })
        }
        CreatureTag::PopulationNumber { .. } => {
            // Parse the first argument as a `u32`
            // Parse the remaining arguments as a `u32`
            let min = argument_1.parse::<u32>().ok().unwrap_or_default();
            let max = remaining_args.parse::<u32>().ok().unwrap_or_default();
            Some(CreatureTag::PopulationNumber { min, max })
        }
        CreatureTag::ProfessionName { .. } => {
            // Parse the first argument as a `String`
            let id = argument_1.to_string();
            // Parse the remaining arguments as a `Vec<String>`
            let values: Vec<String> = remaining_args
                .split(':')
                .map(std::string::ToString::to_string)
                .collect();
            let name = values.first().cloned().unwrap_or_default();
            let plural_name = values.get(1).cloned().unwrap_or_default();
            Some(CreatureTag::ProfessionName {
                id,
                name,
                plural_name,
            })
        }
        CreatureTag::TriggerableGroup { .. } => {
            // Parse the first argument as a `u32`
            let min = argument_1.parse::<u32>().ok().unwrap_or_default();
            // Parse the remaining arguments as a `u32`
            let max = remaining_args.parse::<u32>().ok().unwrap_or_default();
            Some(CreatureTag::TriggerableGroup { min, max })
        }
        CreatureTag::UndergroundDepth { .. } => {
            // Parse the first argument as a `u32`
            let min = argument_1.parse::<u32>().ok().unwrap_or_default();
            // Parse the remaining arguments as a `u32`
            let max = remaining_args.parse::<u32>().ok().unwrap_or_default();
            Some(CreatureTag::UndergroundDepth { min, max })
        }
        CreatureTag::UseCaste { .. } => {
            // Parse the first argument as a `String`
            let caste = argument_1.to_string();
            // Parse the remaining arguments as a `String`
            let original_caste = remaining_args.to_string();
            Some(CreatureTag::UseCaste {
                caste,
                original_caste,
            })
        }
        CreatureTag::UseMaterial { .. } => {
            // Parse the first argument as a `String`
            let material = argument_1.to_string();
            // Parse the remaining arguments as a `String`
            let original_material = remaining_args.to_string();
            Some(CreatureTag::UseMaterial {
                material,
                original_material,
            })
        }
        CreatureTag::UseMaterialTemplate { .. } => {
            // Parse the first argument as a `String`
            let material = argument_1.to_string();
            // Parse the remaining arguments as a `String`
            let template = remaining_args.to_string();
            Some(CreatureTag::UseMaterialTemplate { material, template })
        }
        CreatureTag::UseTissue { .. } => {
            // Parse the first argument as a `String`
            let tissue = argument_1.to_string();
            // Parse the remaining arguments as a `String`
            let original_tissue = remaining_args.to_string();
            Some(CreatureTag::UseTissue {
                tissue,
                original_tissue,
            })
        }
        CreatureTag::UseTissueTemplate { .. } => {
            // Parse the first argument as a `String`
            let tissue = argument_1.to_string();
            // Parse the remaining arguments as a `String`
            let template = remaining_args.to_string();
            Some(CreatureTag::UseTissueTemplate { tissue, template })
        }
        _ => {
            tracing::warn!("parse_complex_token: non-complex token: {token}");
            None
        }
    }
}
