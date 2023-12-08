use super::tokens::CasteTag;

pub static CASTE_TOKENS: phf::Map<&'static str, CasteTag> = phf::phf_map! {
    "DIURNAL" => CasteTag::ActiveDiurnal,
    "CREPUSCULAR" => CasteTag::ActiveCrepuscular,
    "MATUTINAL" => CasteTag::ActiveMatutinal,
    "VESPERTINE" =>  CasteTag::ActiveVespertine,
    "NOCTURNAL" =>  CasteTag::ActiveNocturnal,
    "AMBUSHPREDATOR" => CasteTag::AmbushPredator,
    "AMBUSH_PREDATOR" => CasteTag::AmbushPredator,
    "AMPHIBIOUS" => CasteTag::Amphibious,
    "CURIOUSBEAST_EATER" => CasteTag::CuriousEater,
    "CURIOUSBEAST_GUZZLER" => CasteTag::CuriousGuzzler,
    "CURIOUSBEAST_ITEM" => CasteTag::CuriousItem,
    "NO_SPRING" => CasteTag::NoSpring,
    "NO_SUMMER" => CasteTag::NoSummer,
    "NO_AUTUMN" => CasteTag::NoFall,
    "NO_WINTER" => CasteTag::NoWinter,
    "TRAINABLE_HUNTING" => CasteTag::TrainableHunting,
    "TRAINABLE_WAR" => CasteTag::TrainableWar,
    "TRAINABLE" => CasteTag::Trainable,
    "ADOPTS_OWNER" => CasteTag::AdoptsOwner,
    "BENIGN" => CasteTag::Benign,
    "AQUATIC" => CasteTag::Aquatic,
    "ARENA_RESTRICTED" => CasteTag::ArenaRestricted,
    "AT_PEACE_WITH_WILDLIFE" => CasteTag::AtPeaceWithWildlife,
    "BONECARN" => CasteTag::BoneCarn,
    "CAN_LEARN" => CasteTag::CanLearn,
    "CAN_SPEAK" => CasteTag::CanSpeak,
    "CARNIVORE" => CasteTag::Carnivore,
    "COMMON_DOMESTIC" => CasteTag::CommonDomestic,
    "COOKABLE_LIVE" => CasteTag::CookableLive,
    "DEMON" => CasteTag::Demon,
    "DIE_WHEN_VERMIN_BITE" => CasteTag::DieWhenVerminBite,
    "EQUIPS" => CasteTag::Equips,
    "EXTRAVISION" => CasteTag::Extravision,
    "FEATURE_BEAST" => CasteTag::FeatureBeast,
    "FEMALE" => CasteTag::Female,
    "FIREIMMUNE" => CasteTag::FireImmune,
    "FIREIMMUNE_SUPER" => CasteTag::FireImmuneSuper,
    "FISHITEM" => CasteTag::FishItem,
    "FLIER" => CasteTag::Flier,
    "GNAWER" => CasteTag::Gnawer,
    "HAS_NERVES" => CasteTag::HasNerves,
    "HUNTS_VERMIN" => CasteTag::HuntsVermin,
    "IMMOBILE" => CasteTag::Immobile,
    "IMMOBILE_LAND" => CasteTag::ImmobileLand,
    "IMMOLATE" => CasteTag::Immolate,
    "INTELLIGENT" => CasteTag::Intelligent,
    "LIGHT_GEN" => CasteTag::LightGen,
    "LOCKPICKER" => CasteTag::LockPicker,
    "MAGMA_VISION" => CasteTag::MagmaVision,
    "MALE" => CasteTag::Male,
    "MEANDERER" => CasteTag::Meanderer,
    "MEGABEAST" => CasteTag::Megabeast,
    "MISCHIEVIOUS"  => CasteTag::Mischievous,
    "MISCHIEVOUS" => CasteTag::Mischievous,
    "MOUNT" => CasteTag::Mount,
    "MOUNT_EXOTIC" => CasteTag::MountExotic,
    "MULTIPART_FULL_VISION" => CasteTag::MultipartFullVision,
    "MULTIPLE_LITTER_RARE" => CasteTag::MultipleLitterRare,
    "NATURAL" => CasteTag::Natural,
    "NO_CONNECTIONS_FOR_MOVEMENT" => CasteTag::NoConnectionsForMovement,
    "NO_DIZZINESS" => CasteTag::NoDizziness,
    "NO_DRINK" => CasteTag::NoDrink,
    "NO_EAT" => CasteTag::NoEat,
    "NO_FEVERS" => CasteTag::NoFevers,
    "NO_GENDER" => CasteTag::NoGender,
    "NO_SLEEP" => CasteTag::NoSleep,
    "NOBONES" => CasteTag::NoBones,
    "NOBREATHE" => CasteTag::NoBreathe,
    "NOEMOTION" => CasteTag::NoEmotion,
    "NOEXERT" => CasteTag::NoExert,
    "NOFEAR" => CasteTag::NoFear,
    "NOMEAT" => CasteTag::NoMeat,
    "NONAUSEA" => CasteTag::NoNausea,
    "NOPAIN" => CasteTag::NoPain,
    "NOSKIN" => CasteTag::NoSkin,
    "NOSKULL" => CasteTag::NoSkull,
    "NOSMELLYROT" => CasteTag::NoSmellyRot,
    "NOSTUCKINS" => CasteTag::NoStuckIns,
    "NOSTUN" => CasteTag::NoStun,
    "NOT_BUTCHERABLE" => CasteTag::NotButcherable,
    "NOT_LIVING" => CasteTag::NotLiving,
    "NOTHOUGHT" => CasteTag::NoThought,
    "OPPOSED_TO_LIFE" => CasteTag::OpposedToLife,
    "OUTSIDER_CONTROLLABLE" => CasteTag::OutsiderControllable,
    "PACK_ANIMAL" => CasteTag::PackAnimal,
    "PARALYZEIMMUNE" => CasteTag::ParalyzeImmune,
    "PET" => CasteTag::Pet,
    "PET_EXOTIC" => CasteTag::PetExotic,
    "POWER" => CasteTag::Power,
    "SEMIMEGABEAST" => CasteTag::SemiMegabeast,
    "SLOW_LEARNER" => CasteTag::SlowLearner,
    "SMALL_REMAINS" => CasteTag::SmallRemains,
    "STANDARD_GRAZER" => CasteTag::StandardGrazer,
    "SUPERNATURAL" => CasteTag::Supernatural,
    "SWIMS_INNATE" => CasteTag::SwimsInnate,
    "SWIMS_LEARNED" => CasteTag::SwimsLearned,
    "THICKWEB" => CasteTag::ThickWeb,
    "TITAN" => CasteTag::Titan,
    "TRANCES" => CasteTag::Trances,
    "TRAPAVOID" => CasteTag::TrapAvoid,
    "UNIQUE_DEMON" => CasteTag::UniqueDemon,
    "VEGETATION" => CasteTag::Vegetation,
    "VERMIN_HATEABLE" => CasteTag::VerminHateable,
    "VERMIN_MICRO" => CasteTag::VerminMicro,
    "VERMIN_NOFISH" => CasteTag::VerminNoFish,
    "VERMIN_NOROAM" => CasteTag::VerminNoRoam,
    "VERMIN_NOTRAP" => CasteTag::VerminNoTrap,
    "WAGON_PULLER" => CasteTag::WagonPuller,
    "WEBIMMUNE" => CasteTag::WebImmune,
    "BODY_SIZE" => CasteTag::BodySize,
    "MILKABLE" => CasteTag::Milkable,
    "EGG_SIZE" => CasteTag::EggSize,
    "BABY" => CasteTag::Baby,
    "CHILD" => CasteTag::Child,
    "DIFFICULTY" => CasteTag::Difficulty,
    "GRASSTRAMPLE" => CasteTag::GrassTrample,
    "GRAZER" => CasteTag::Grazer,
    "LOW_LIGHT_VISION" => CasteTag::LowLightVision,
    "PETVALUE" => CasteTag::PetValue,
    "POP_RATIO" => CasteTag::PopRatio,
    "CLUTCH_SIZE" => CasteTag::ClutchSize,
    "LITTERSIZE" => CasteTag::LitterSize,
    "DESCRIPTION" => CasteTag::Description,
    "MAXAGE" => CasteTag::MaxAge,
    "ALL_ACTIVE" => CasteTag::AllActive,
    "CREATURE_CLASS" => CasteTag::CreatureClass,
    "BABY_NAME" => CasteTag::BabyName,
    "CHILD_NAME" => CasteTag::ChildName,
    "CASTE_NAME" => CasteTag::CasteName,
    "CASTE_TILE" => CasteTag::CasteTile,
    "CASTE_ALTTILE" => CasteTag::CasteAltTile,
    "CASTE_COLOR" => CasteTag::CasteColor,
    "CASTE_GLOWTILE" => CasteTag::CasteGlowTile,
    "CASTE_GLOWCOLOR" => CasteTag::CasteGlowColor,
    "CHANGE_BODY_SIZE_PERC" => CasteTag::ChangeBodySizePercent,
    "NOT_FIREIMMUNE" => CasteTag::NotFireImmune,
    "NIGHT_CREATURE" => CasteTag::NightCreature,
    "NIGHT_CREATURE_BOGEYMAN" => CasteTag::NightCreatureBogeyman,
    "NIGHT_CREATURE_NIGHTMARE" => CasteTag::NightCreatureNightmare,
    "NIGHT_CREATURE_HUNTER" => CasteTag::NightCreatureHunter,
    "LARGE_PREDATOR" => CasteTag::LargePredator,
    "HAS_BLOOD" => CasteTag::HasBlood,
    "GRASP" => CasteTag::Grasp,
    "RACE_GAIT" => CasteTag::RaceGait,
    "NATURAL_ANIMAL" => CasteTag::NaturalAnimal,
    "CANNOT_BREATHE_WATER" => CasteTag::CannotBreatheWater,
    "CANNOT_BREATHE_AIR" => CasteTag::CannotBreatheAir,
    "CURIOUS_BEAST" => CasteTag::CuriousBeast,
    "UTTERANCES" => CasteTag::Utterances,
    "GAIT" => CasteTag::Gait,
};
