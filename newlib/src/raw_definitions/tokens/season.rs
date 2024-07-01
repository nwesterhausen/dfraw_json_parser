use crate::tags::SeasonTag;

/// Mapping of position tokens to strings
pub static SEASON_TOKENS: phf::Map<&'static str, SeasonTag> = phf::phf_map! {
  "SPRING" => SeasonTag::Spring,
  "SUMMER" => SeasonTag::Summer,
  "AUTUMN" => SeasonTag::Autumn,
  "WINTER" => SeasonTag::Winter,
};
