//! String token to parsed tag map for position tokens.

use crate::tags::PositionTag;

/// Mapping of position tokens to strings
pub static POSITION_TOKENS: phf::Map<&'static str, PositionTag> = phf::phf_map! {
  "ACCOUNT_EXEMPT" => PositionTag::AccountExempt,
  "ALLOWED_CLASS" => PositionTag::AllowedClass,
  "ALLOWED_CREATURE" => PositionTag::AllowedCreature,
  "APPOINTED_BY" => PositionTag::AppointedBy,
  "BRAG_ON_KILL" => PositionTag::BragOnKill,
  "CHAT_WORTHY" => PositionTag::ChatWorthy,
  "COLOR" => PositionTag::Color,
  "COMMANDER" => PositionTag::Commander,
  "CONQUERED_SITE" => PositionTag::ConqueredSite,
  "DEMAND_MAX" => PositionTag::DemandMax,
  "DETERMINES_COIN_DESIGN" => PositionTag::DeterminesCoinDesign,
  "DO_NOT_CULL" => PositionTag::DoNotCull,
  "DUTY_BOUND" => PositionTag::DutyBound,
  "ELECTED" => PositionTag::Elected,
  "EXECUTION_SKILL" => PositionTag::ExecutionSkill,
  "EXPORTED_IN_LEGENDS" => PositionTag::ExportedInLegends,
  "FLASHES" => PositionTag::Flashes,
  "GENDER" => PositionTag::Gender,
  "KILL_QUEST" => PositionTag::KillQuest,
  "LAND_HOLDER" => PositionTag::LandHolder,
  "LAND_NAME" => PositionTag::LandName,
  "MANDATE_MAX" => PositionTag::MandateMax,
  "MENIAL_WORK_EXEMPTION" => PositionTag::MenialWorkExemption,
  "MENIAL_WORK_EXEMPTION_SPOUSE" => PositionTag::MenialWorkExemptionSpouse,
  "MILITARY_SCREEN_ONLY" => PositionTag::MilitaryScreenOnly,
  "NAME" => PositionTag::Name,
  "NAME_MALE" => PositionTag::NameMale,
  "NAME_FEMALE" => PositionTag::NameFemale,
  "DESCRIPTION" => PositionTag::Description,
  "NUMBER" => PositionTag::Number,
  "PRECEDENCE" => PositionTag::Precedence,
  "PUNISHMENT_EXEMPTION" => PositionTag::PunishmentExemption,
  "QUEST_GIVER" => PositionTag::QuestGiver,
  "REJECTED_CLASS" => PositionTag::RejectedClass,
  "REJECTED_CREATURE" => PositionTag::RejectedCreature,
  "REPLACED_BY" => PositionTag::ReplacedBy,
  "REQUIRED_BEDROOM" => PositionTag::RequiredBedroom,
  "REQUIRED_BOXES" => PositionTag::RequiredBoxes,
  "REQUIRED_CABINETS" => PositionTag::RequiredCabinets,
  "REQUIRED_DINING" => PositionTag::RequiredDining,
  "REQUIRED_OFFICE" => PositionTag::RequiredOffice,
  "REQUIRED_RACKS" => PositionTag::RequiredRacks,
  "REQUIRED_STANDS" => PositionTag::RequiredStands,
  "REQUIRED_TOMB" => PositionTag::RequiredTomb,
  "REQUIRES_MARKET" => PositionTag::RequiresMarket,
  "REQUIRES_POPULATION" => PositionTag::RequiresPopulation,
  "RESPONSIBILITY" => PositionTag::Responsibility,
  "RULES_FROM_LOCATION" => PositionTag::RulesFromLocation,
  "SITE" => PositionTag::Site,
  "SLEEP_PRETENSION" => PositionTag::SleepPretension,
  "SPECIAL_BURIAL" => PositionTag::SpecialBurial,
  "SPOUSE" => PositionTag::Spouse,
  "SPOUSE_FEMALE" => PositionTag::SpouseFemale,
  "SPOUSE_MALE" => PositionTag::SpouseMale,
  "SQUAD" => PositionTag::Squad,
  "SUCCESSION" => PositionTag::Succession,
};
