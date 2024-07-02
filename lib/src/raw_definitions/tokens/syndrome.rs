use crate::tags::SyndromeTag;

/// Mapping of syndrome tokens to strings
pub static SYNDROME_TOKENS: phf::Map<&'static str, SyndromeTag> = phf::phf_map! {
    "SYN_NAME" => SyndromeTag::Name,
    "SYN_IDENTIFIER" => SyndromeTag::Identifier,
    "SYN_INJECTED" => SyndromeTag::Injected,
    "SYN_CONTACT" => SyndromeTag::Contact,
    "SYN_INHALED" => SyndromeTag::Inhaled,
    "SYN_INGESTED" => SyndromeTag::Ingested,
    "SYN_AFFECTED_CLASS" => SyndromeTag::AffectedClass,
    "SYN_IMMUNE_CLASS" => SyndromeTag::ImmuneClass,
    "SYN_AFFECTED_CREATURE" => SyndromeTag::AffectedCreature,
    "SYN_IMMUNE_CREATURE" => SyndromeTag::ImmuneCreature,
    "SYN_CONCENTRATION_ADDED" => SyndromeTag::ConcentrationAdded,
    "SYN_NO_HOSPITAL" => SyndromeTag::NoHospital,
    "SYN_CLASS" => SyndromeTag::Class,
};
