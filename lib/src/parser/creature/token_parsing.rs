use super::{Token as CreatureTag, TOKEN_MAP};
use crate::parser::metadata::{RawObjectToken, TokenComplexity};

#[typetag::serde]
impl RawObjectToken for CreatureTag {
    fn get_complexity(&self) -> TokenComplexity {
        // Implement the logic for getting the complexity of the token.
        // Return the appropriate `TokenComplexity` variant.
        match self {
          CreatureTag::ApplyCurrentCreatureVariation |
          CreatureTag::ArtificialHiveable |
          CreatureTag::DoesNotExist |
          CreatureTag::EquipmentWagon |
          CreatureTag::Evil |
          CreatureTag::Fanciful |
          CreatureTag::Generated |
          CreatureTag::Good |
          CreatureTag::GoToEnd |
          CreatureTag::GoToStart |
          CreatureTag::LargeRoaming |
          CreatureTag::LocalPopsControllable |
          CreatureTag::LocalPopsProduceHeroes |
          CreatureTag::LooseClusters |
          CreatureTag::Mundane |
          CreatureTag::Savage |
          CreatureTag::Ubiquitous |
          CreatureTag::Utterances |
          CreatureTag::VerminEater |
          CreatureTag::VerminFish |
          CreatureTag::VerminGrounder |
          CreatureTag::VerminRotter |
          CreatureTag::VerminSoil |
          CreatureTag::VerminSoilColony |
          // Other tags
          CreatureTag::Unknown |
          CreatureTag::AllCastesAlive |
          CreatureTag::Equipment |
          CreatureTag::MatesToBreed |
          CreatureTag::OccursAsEntityRace |
          CreatureTag::SmallRace |
          CreatureTag::TwoGenders
          => TokenComplexity::None,
          CreatureTag::AltTile{..} |
          CreatureTag::Biome {..} |
          CreatureTag::Caste {..} |
          CreatureTag::ChangeFrequencyPercent {..} |
          CreatureTag::CopyTagsFrom {..} |
          CreatureTag::CreatureSoldierTile {..} |
          CreatureTag::CreatureTile {..} |
          CreatureTag::Frequency { .. } |
          CreatureTag::GlowTile {..} |
          CreatureTag::GoToTag {..} |
          CreatureTag::PlusMaterial {..} |
          CreatureTag::PrefString {..} |
          CreatureTag::RemoveMaterial {..} |
          CreatureTag::RemoveTissue {..} |
          CreatureTag::SelectAdditionalCaste {..} |
          CreatureTag::SelectCaste {..} |
          CreatureTag::SelectMaterial {..} |
          CreatureTag::SelectTissue {..} |
          CreatureTag::SlainSpeech {..} |
          CreatureTag::SmellTrigger {..} |
          CreatureTag::SoldierAltTile {..} |
          CreatureTag::SourceHfid {..} |
          CreatureTag::Sphere {..} |
          CreatureTag::Tissue {..}
          => TokenComplexity::Simple,
          CreatureTag::ApplyCreatureVariation{..} |
          CreatureTag::ClusterNumber {..} |
          CreatureTag::Color {..} |
          CreatureTag::GlowColor {..} |
          CreatureTag::GeneralBabyName {..} |
          CreatureTag::GeneralChildName {..} |
          CreatureTag::HarvestProduct {..} |
          CreatureTag::Name {..} |
          CreatureTag::PopulationNumber {..} |
          CreatureTag::ProfessionName {..} |
CreatureTag::TriggerableGroup { .. } |
          CreatureTag::UndergroundDepth {..} |
          CreatureTag::UseCaste {..} |
          CreatureTag::UseMaterial {..} |
          CreatureTag::UseMaterialTemplate {..} |
          CreatureTag::UseTissue {..} |
          CreatureTag::UseTissueTemplate {..}
          => TokenComplexity::Complex,
        }
    }

    fn parse_token(key: &str, _value: &str) -> Option<Self> {
        // Implement the logic for parsing the token from the key and value.
        // Create a new `CreatureTag` instance and return it, or return `None` if the token could not be parsed.

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
