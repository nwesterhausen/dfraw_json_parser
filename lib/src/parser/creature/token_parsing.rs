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
fn parse_complex_token(token: &CreatureTag, args: &str) -> Option<CreatureTag> {
    let mut values = args.split(':').collect::<Vec<&str>>();
    match token {
        CreatureTag::ApplyCreatureVariation { .. } => {
            // Check if there are at least 2 arguments
            if values.len() < 2 {
                tracing::warn!("parse_complex_token: not enough arguments for CreatureTag::ApplyCreatureVariation {}/2", args.len());
                return None;
            }
            let id = values.remove(0).to_string();
            let args = values
                .iter()
                .map(std::string::ToString::to_string)
                .collect();
            Some(CreatureTag::ApplyCreatureVariation { id, args })
        }
        CreatureTag::ClusterNumber { .. } => {
            // Check if there are at least 2 arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::ClusterNumber {}/2",
                    args.len()
                );
                return None;
            }
            let Ok(min) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::ClusterNumber failed to parse min value: {args}");
                return None;
            };
            let Ok(max) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::ClusterNumber failed to parse max value: {args}");
                return None;
            };
            Some(CreatureTag::ClusterNumber { min, max })
        }
        CreatureTag::Color { .. } => {
            // Check if there are at least 3 arguments
            if values.len() < 3 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::Color {}/3",
                    args.len()
                );
                return None;
            }
            let Ok(foreground) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::Color failed to parse foreground value: {args}");
                return None;
            };
            let Ok(background) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::Color failed to parse background value: {args}");
                return None;
            };
            let Ok(brightness) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::Color failed to parse brightness value: {args}");
                return None;
            };
            Some(CreatureTag::Color {
                foreground,
                background,
                brightness,
            })
        }
        CreatureTag::GlowColor { .. } => {
            // Check if there are at least 3 arguments
            if values.len() < 3 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::GlowColor {}/3",
                    args.len()
                );
                return None;
            }
            let Ok(foreground) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::GlowColor failed to parse foreground value: {args}");
                return None;
            };
            let Ok(background) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::GlowColor failed to parse background value: {args}");
                return None;
            };
            let Ok(brightness) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::GlowColor failed to parse brightness value: {args}");
                return None;
            };
            Some(CreatureTag::GlowColor {
                foreground,
                background,
                brightness,
            })
        }
        CreatureTag::GeneralBabyName { .. } => {
            // Check if there are at least 2 arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::GeneralBabyName {}/2",
                    args.len()
                );
                return None;
            }
            let singular = values.remove(0).to_string();
            let plural = values.join(":").to_string();
            Some(CreatureTag::GeneralBabyName { singular, plural })
        }
        CreatureTag::GeneralChildName { .. } => {
            // Check if there are at least 2 arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::GeneralChildName {}/2",
                    args.len()
                );
                return None;
            }
            let singular = values.remove(0).to_string();
            let plural = values.join(":").to_string();
            Some(CreatureTag::GeneralChildName { singular, plural })
        }
        CreatureTag::HarvestProduct { .. } => {
            // Check if there are at least 3 arguments
            if values.len() < 3 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::HarvestProduct {}/3",
                    args.len()
                );
                return None;
            }
            let Ok(number) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::HarvestProduct failed to parse number value: {args}");
                return None;
            };
            let Ok(time) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::HarvestProduct failed to parse time value: {args}");
                return None;
            };
            let item_tokens: Vec<String> = values
                .iter()
                .map(std::string::ToString::to_string)
                .collect();
            Some(CreatureTag::HarvestProduct {
                number,
                time,
                item_tokens,
            })
        }
        CreatureTag::Name { .. } => {
            // Check if there are at least 3 arguments
            if values.len() < 3 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::Name {}/3",
                    args.len()
                );
                return None;
            }
            let name = values.remove(0).to_string();
            let plural_name = values.remove(0).to_string();
            let adjective = values.join(":").to_string();
            Some(CreatureTag::Name {
                name,
                plural_name,
                adjective,
            })
        }
        CreatureTag::PopulationNumber { .. } => {
            // Check if there are at least 2 arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::PopulationNumber {}/2",
                    args.len()
                );
                return None;
            }
            let Ok(min) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::PopulationNumber failed to parse min value: {args}");
                return None;
            };
            let Ok(max) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::PopulationNumber failed to parse max value: {args}");
                return None;
            };
            Some(CreatureTag::PopulationNumber { min, max })
        }
        CreatureTag::ProfessionName { .. } => {
            // Check if there are at least 3 arguments
            if values.len() < 3 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::ProfessionName {}/3",
                    args.len()
                );
                return None;
            }
            let id = values.remove(0).to_string();
            let name = values.remove(0).to_string();
            let plural_name = values.join(":").to_string();
            Some(CreatureTag::ProfessionName {
                id,
                name,
                plural_name,
            })
        }
        CreatureTag::TriggerableGroup { .. } => {
            // Check if there are at least 2 arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::TriggerableGroup {}/2",
                    args.len()
                );
                return None;
            }
            let Ok(min) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::TriggerableGroup failed to parse min value: {args}");
                return None;
            };
            let Ok(max) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::TriggerableGroup failed to parse max value: {args}");
                return None;
            };
            Some(CreatureTag::TriggerableGroup { min, max })
        }
        CreatureTag::UndergroundDepth { .. } => {
            // Check if there are at least 2 arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::UndergroundDepth {}/2",
                    args.len()
                );
                return None;
            }
            let Ok(min) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::UndergroundDepth failed to parse min value: {args}");
                return None;
            };
            let Ok(max) = values.remove(0).parse::<u32>() else {
                tracing::warn!("parse_complex_token: CreatureTag::UndergroundDepth failed to parse max value: {args}");
                return None;
            };
            Some(CreatureTag::UndergroundDepth { min, max })
        }
        CreatureTag::UseCaste { .. } => {
            // Check if there are at least 2 arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::UseCaste {}/2",
                    args.len()
                );
                return None;
            }
            let caste = values.remove(0).to_string();
            let original_caste = values.join(":").to_string();
            Some(CreatureTag::UseCaste {
                caste,
                original_caste,
            })
        }
        CreatureTag::UseMaterial { .. } => {
            // Check if there are at least 2 arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::UseMaterial {}/2",
                    args.len()
                );
                return None;
            }
            let material = values.remove(0).to_string();
            let original_material = values.join(":").to_string();
            Some(CreatureTag::UseMaterial {
                material,
                original_material,
            })
        }
        CreatureTag::UseMaterialTemplate { .. } => {
            // Check if there are at least 2 arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::UseMaterialTemplate {}/2",
                    args.len()
                );
                return None;
            }
            let material = values.remove(0).to_string();
            let template = values.join(":").to_string();
            Some(CreatureTag::UseMaterialTemplate { material, template })
        }
        CreatureTag::UseTissue { .. } => {
            // Check if there are at least 2 arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::UseTissue {}/2",
                    args.len()
                );
                return None;
            }
            let tissue = values.remove(0).to_string();
            let original_tissue = values.join(":").to_string();
            Some(CreatureTag::UseTissue {
                tissue,
                original_tissue,
            })
        }
        CreatureTag::UseTissueTemplate { .. } => {
            // Check if there are at least 2 arguments
            if values.len() < 2 {
                tracing::warn!(
                    "parse_complex_token: not enough arguments for CreatureTag::UseTissueTemplate {}/2",
                    args.len()
                );
                return None;
            }
            let tissue = values.remove(0).to_string();
            let template = values.join(":").to_string();
            Some(CreatureTag::UseTissueTemplate { tissue, template })
        }
        _ => {
            tracing::warn!("parse_complex_token: non-complex token: {token}");
            None
        }
    }
}
