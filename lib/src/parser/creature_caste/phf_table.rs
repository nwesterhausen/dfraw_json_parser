use super::tokens::CasteTag;

pub static CASTE_TOKENS: phf::Map<&'static str, CasteTag> = phf::phf_map! {
    "ADOPTS_OWNER" => CasteTag::AdoptsOwner,
    "ALCOHOL_DEPENDENT" => CasteTag::AlcoholDependent,
    "ALL_ACTIVE" => CasteTag::AllActive,
    "CASTE_ALTTILE" => CasteTag::AltTile {
        tile: String::new(),
    },
    "AMBUSHPREDATOR" => CasteTag::AmbushPredator,
    "AMPHIBIOUS" => CasteTag::Amphibious,
    "APPLY_CREATURE_VARIATION" => CasteTag::ApplyCreatureVariation {
        id: String::new(),
        args: Vec::new(),
    },
    "APPLY_CURRENT_CREATURE_VARIATION" => CasteTag::ApplyCurrentCreatureVariation,
    "AQUATIC" => CasteTag::Aquatic,
    "ARENA_RESTRICTED" => CasteTag::ArenaRestricted,
    "AT_PEACE_WITH_WILDLIFE" => CasteTag::AtPeaceWithWildlife,
    "ATTACK" => CasteTag::Attack { name: String::new(), body_part: String::new() },
    "ATTACK_TRIGGER" => CasteTag::AttackTrigger { population: 0, exported_wealth: 0, created_wealth: 0 },
    "BABY" => CasteTag::Baby { age: 0 },
    "BABYNAME" => CasteTag::BabyName { singular: String::new(), plural: String::new() },
    "BEACH_FREQUENCY" => CasteTag::BeachFrequency { frequency: 0 },
    "BENIGN" => CasteTag::Benign,
    "BLOOD" => CasteTag::Blood { material: String::new(), state: String::new() },
    "BLOODSUCKER" => CasteTag::BloodSucker,
    "BODY" => CasteTag::Body { body_parts: Vec::new() },
    "BODY_APPEARANCE_MODIFIER" => CasteTag::BodyAppearanceModifier { attribute: String::new(), values: [0,0,0,0,0,0,0] },
    "BODY_DETAIL_PLAN" => CasteTag::BodyDetailPlan { body_plan: String::new(), arguments: Vec::new() },
    "BODY_SIZE" => CasteTag::BodySize { year: 0, days: 0, size: 0 },
    "BODYGLOSS" => CasteTag::BodyGloss { gloss: String::new() },
    "BONECARN" => CasteTag::BoneCarn,
    "BP_ADD_TYPE" => CasteTag::BodyPartAddType { body_part_type: String::new() },
    "BP_APPEARANCE_MODIFIER" => CasteTag::BodyPartAppearanceModifier { quality: String::new(), spread: [0,0,0,0,0,0,0] },
    "BP_REMOVE_TYPE" => CasteTag::BodyPartRemoveType { body_part_type: String::new() },
    "BUILDINGDESTROYER" => CasteTag::BuildingDestroyer {door_and_furniture_focused: false},
    "CAN_DO_INTERACTION" => CasteTag::CanDoInteraction { interaction: String::new() },
    "CAN_LEARN" => CasteTag::CanLearn,
    "CAN_SPEAK" => CasteTag::CanSpeak,
    "CANNOT_CLIMB" => CasteTag::CannotClimb,
    "CANNOT_JUMP" => CasteTag::CannotJump,
    "CANNOT_UNDEAD" => CasteTag::CannotUndead,
    "CANOPENDOORS" => CasteTag::CanOpenDoors,
    "CARNIVORE" => CasteTag::Carnivore,
    "CAVE_ADAPT" => CasteTag::CaveAdaptation,
    "CHANGE_BODY_SIZE_PERC" => CasteTag::ChangeBodySizePercent { percent: 0 },
    "CHILD" => CasteTag::Child { age: 0 },
    "CHILDNAME" => CasteTag::ChildName { singular: String::new(), plural: String::new() },
    "CLUTCH_SIZE" => CasteTag::ClutchSize { min: 0, max: 0 },
    "CASTE_COLOR" => CasteTag::Color { foreground: 0, background: 0, brightness: 0 },
    "COMMON_DOMESTIC" => CasteTag::CommonDomestic,
    "CONVERTED_SPOUSE" => CasteTag::ConvertedSpouse,
    "COOKABLE_LIVE" => CasteTag::CookableLive,
    "CRAZED" => CasteTag::Crazed,
    "CREATURE_CLASS" => CasteTag::CreatureClass { class: String::new() },
    "CREPUSCULAR" => CasteTag::Crepuscular,
    "CURIOUSBEAST_EATER" => CasteTag::CuriousBeastEater,
    "CURIOUSBEAST_GUZZLER" => CasteTag::CuriousBeastGuzzler,
    "CURIOUSBEAST_ITEM" => CasteTag::CuriousBeastItem,
    "CV_ADD_TAG" => CasteTag::CreatureVariationAddTag { tag: String::new() },
    "CV_REMOVE_TAG" => CasteTag::CreatureVariationRemoveTag { tag: String::new() },
    "DEMON" => CasteTag::Demon,
    "DESCRIPTION" => CasteTag::Description { description: String::new() },
    "DIE_WHEN_VERMIN_BITE" => CasteTag::DieWhenVerminBite,
    "DIFFICULTY" => CasteTag::Difficulty { difficulty: 0 },
    "DIURNAL" => CasteTag::Diurnal,
    "DIVE_HUNTS_VERMIN" => CasteTag::DiveHuntsVermin,
    "EBO_ITEM" => CasteTag::ExtraButcherObjectItem { item: String::new(), material: String::new() },
    "EBO_SHAPE" => CasteTag::ExtraButcherObjectShape { shape: String::new() },
    "EGG_MATERIAL" => CasteTag::EggMaterial { material: String::new(), state: String::new() },
    "EGG_SIZE" => CasteTag::EggSize { size: 0 },
    "EQUIPS" => CasteTag::Equips,
    "EXTRA_BUTCHER_OBJECT" => CasteTag::ExtraButcherObject { object_type: String::new(), arguments: Vec::new() },
    "EXTRACT" => CasteTag::Extract { material: String::new() },
    "EXTRAVISION" => CasteTag::Extravision,
    "FEATURE_ATTACK_GROUP" => CasteTag::FeatureAttackGroup,
    "FEATURE_BEAST" => CasteTag::FeatureBeast,
    "FEMALE" => CasteTag::Female,
    "FIREIMMUNE" => CasteTag::FireImmune,
    "FIREIMMUNE_SUPER" => CasteTag::FireImmuneSuper,
    "FISHITEM" => CasteTag::FishItem,
    "FIXED_TEMP" => CasteTag::FixedTemp { temperature: 0 },
    "FLEEQUICK" => CasteTag::FleeQuick,
    "FLIER" => CasteTag::Flier,
    "GAIT" => CasteTag::Gait { gait: String::new() },
    "GENERAL_MATERIAL_FORCE_MULTIPLIER" => CasteTag::GeneralMaterialForceMultiplier { value_a: 0, value_b: 0 },
    "GETS_INFECTIONS_FROM_ROT" => CasteTag::GetsInfectionsFromRot,
    "GETS_WOUND_INFECTIONS" => CasteTag::GetsWoundInfections,
    "CASTE_GLOWCOLOR" => CasteTag::GlowColor { foreground: 0, background: 0, brightness: 0 },
    "CASTE_GLOWTILE" => CasteTag::GlowTile { tile: String::new() },
    "GNAWER" => CasteTag::Gnawer { verb: String::new() },
    "GOBBLE_VERMIN_CLASS" => CasteTag::GobbleVerminClass { vermin_class: String::new() },
    "GOBBLE_VERMIN_CREATURE" => CasteTag::GobbleVerminCreature { vermin_creature: String::new(), vermin_caste: String::new() },
    "GRASS_TRAMPLE" => CasteTag::GrassTrample { trample: 0 },
    "GRAVITATE_BODY_SIZE" => CasteTag::GravitateBodySize { target: 0 },
    "GRAZER" => CasteTag::Grazer { grazer: 0 },
    "HABIT" => CasteTag::Habit { habit: String::new() },
    "HABIT_NUM" => CasteTag::HabitNumber { number: 0 },
    "HAS_NERVES" => CasteTag::HasNerves,
    "HASSHELL" => CasteTag::HasShell,
    "HOMEOTHERM" => CasteTag::Homeotherm { temperature: None },
    "HUNTS_VERMIN" => CasteTag::HuntsVermin,
    "IMMOBILE" => CasteTag::Immobile,
    "IMMOBILE_LAND" => CasteTag::ImmobileLand,
    "IMMOLATE" => CasteTag::Immolate,
    "INTELLIGENT" => CasteTag::Intelligent,
    "CDI" => CasteTag::InteractionDetail { args: Vec::new() },
    "ITEMCORPSE" => CasteTag::ItemCorpse { item: String::new(), material: String::new() },
    "ITEMCORPSE_QUALITY" => CasteTag::ItemCorpseQuality { quality: 0 },
    "LAIR" => CasteTag::Lair { lair: String::new(), probability: 0 },
    "LAIR_CHARACTERISTIC" => CasteTag::LairCharacteristic { characteristic: String::new() },
    "LAIR_HUNTER" => CasteTag::LairHunter,
    "LAIR_HUNTER_SPEECH" => CasteTag::LairHunterSpeech { speech_file: String::new() },
    "LARGE_PREDATOR" => CasteTag::LargePredator,
    "LAYS_EGGS" => CasteTag::LaysEggs,
    "LAYS_UNUSUAL_EGGS" => CasteTag::LaysUnusualEggs { item: String::new(), material: String::new() },
    "LIGAMENTS" => CasteTag::Ligaments { material: String::new(), healing_rate: 0 },
    "LIGHT_GEN" => CasteTag::LightGen,
    "LIKES_FIGHTING" => CasteTag::LikesFighting,
    "LISP_SPEECH" => CasteTag::Lisp,
    "LITTER_SIZE" => CasteTag::LitterSize { min: 0, max: 0 },
    "LOCKPICKER" => CasteTag::LockPicker,
    "LOW_LIGHT_VISION" => CasteTag::LowLightVision { vision: 0 },
    "MAGICAL" => CasteTag::Magical,
    "MAGMA_VISION" => CasteTag::MagmaVision,
    "MALE" => CasteTag::Male,
    "MANNERISM_LAUGH" => CasteTag::MannerismLaugh,
    "MANNERISM_SMILE" => CasteTag::MannerismSmile,
    "MANNERISM_WALK" => CasteTag::MannerismWalk,
    "MANNERISM_SIT" => CasteTag::MannerismSit,
    "MANNERISM_BREATH" => CasteTag::MannerismBreath,
    "MANNERISM_POSTURE" => CasteTag::MannerismPosture,
    "MANNERISM_STRETCH" => CasteTag::MannerismStretch,
    "MANNERISM_EYELIDS" => CasteTag::MannerismEyelids,
    "MANNERISM_FINGERS" => CasteTag::MannerismFingers { finger: String::new(), fingers: String::new() },
    "MANNERISM_NOSE" => CasteTag::MannerismNose { nose: String::new() },
    "MANNERISM_EAR" => CasteTag::MannerismEar { ear: String::new() },
    "MANNERISM_HEAD" => CasteTag::MannerismHead { head: String::new() },
    "MANNERISM_EYES" => CasteTag::MannerismEyes { eyes: String::new() },
    "MANNERISM_MOUTH" => CasteTag::MannerismMouth { mouth: String::new() },
    "MANNERISM_KNUCKLES" => CasteTag::MannerismKnuckles { knuckles: String::new() },
    "MANNERISM_LIPS" => CasteTag::MannerismLips { lips: String::new() },
    "MANNERISM_CHEEK" => CasteTag::MannerismCheek { cheek: String::new() },
    "MANNERISM_NAILS" => CasteTag::MannerismNails { nails: String::new() },
    "MANNERISM_FEET" => CasteTag::MannerismFeet { feet: String::new() },
    "MANNERISM_ARMS" => CasteTag::MannerismArms { arms: String::new() },
    "MANNERISM_HANDS" => CasteTag::MannerismHands { hands: String::new() },
    "MANNERISM_TONGUE" => CasteTag::MannerismTongue { tongue: String::new() },
    "MANNERISM_LEG" => CasteTag::MannerismLeg { leg: String::new() },
    "MATUTINAL" => CasteTag::Matutinal,
    "MAXAGE" => CasteTag::MaxAge { min: 0, max: 0 },
    "MEANDERER" => CasteTag::Meanderer,
    "MEGABEAST" => CasteTag::Megabeast,
    "MENT_ATT_CAP_PERC" => CasteTag::MentalAttributeCapPercentage { attribute: String::new(), percentage: 0 },
    "MENT_ATT_RANGE" => CasteTag::MentalAttributeRange { attribute: String::new(), ranges: [0,0,0,0,0,0,0] },
    "MENT_ATT_RATE" => CasteTag::MentalAttributeRate { attribute: String::new(), improvement_cost: 0, decay_rate_unused: 0, decay_rate_rusty: 0, decay_rate_demotion: 0 },
    "MILKABLE" => CasteTag::Milkable { material: String::new(), frequency: 0 },
    "MISCHIEVOUS" => CasteTag::Mischievous,
    "MISCHIEVIOUS" => CasteTag::Mischievous,
    "MODVALUE" => CasteTag::ModValue { value: String::new() },
    "MOUNT" => CasteTag::Mount,
    "MOUNT_EXOTIC" => CasteTag::MountExotic,
    "MULTIPART_FULL_VISION" => CasteTag::MultipartFullVision,
    "MULTIPLE_LITTER_RARE" => CasteTag::MultipleLitterRare,
    "CASTE_NAME" => CasteTag::Name { singular: String::new(), plural: String::new(), adjective: String::new() },
    "NATURAL" => CasteTag::Natural,
    "NATURAL_ANIMAL" => CasteTag::Natural,
    "NATURAL_SKILL" => CasteTag::NaturalSkill { skill: String::new(), level: 0 },
    "NIGHT_CREATURE_BOGEYMAN" => CasteTag::NightCreatureBogeyman,
    "NIGHT_CREATURE_EXPERIMENTER" => CasteTag::NightCreatureExperimenter,
    "NIGHT_CREATURE_HUNTER" => CasteTag::NightCreatureHunter,
    "NIGHT_CREATURE_NIGHTMARE" => CasteTag::NightCreatureNightmare,
    "NO_CONNECTIONS_FOR_MOVEMENT" => CasteTag::NoConnectionsForMovement,
    "NO_DIZZINESS" => CasteTag::NoDizziness,
    "NO_DRINK" => CasteTag::NoDrink,
    "NO_EAT" => CasteTag::NoEat,
    "NO_FALL" => CasteTag::NoFall,
    "NO_FEVERS" => CasteTag::NoFevers,
    "NO_GENDER" => CasteTag::NoGender,
    "NO_PHYS_ATT_GAIN" => CasteTag::NoPhysicalAttributeGain,
    "NO_PHYS_ATT_RUST" => CasteTag::NoPhysicalAttributeRust,
    "NO_SLEEP" => CasteTag::NoSleep,
    "NO_SPRING" => CasteTag::NoSpring,
    "NO_SUMMER" => CasteTag::NoSummer,
    "NO_THOUGHT_CENTER_FOR_MOVEMENT" => CasteTag::NoThoughtCenterForMovement,
    "NO_UNIT_TYPE_COLOR" => CasteTag::NoUnitTypeColor,
    "NO_VEGETATION_PERTURB" => CasteTag::NoVegetationDisturbance,
    "NO_WINTER" => CasteTag::NoWinter,
    "NOBONES" => CasteTag::NoBones,
    "NOBREATHE" => CasteTag::NoBreathe,
    "NOCTURNAL" => CasteTag::Nocturnal,
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
    "ODOR_LEVEL" => CasteTag::OdorLevel { odor_level: 0 },
    "ODOR_STRING" => CasteTag::OdorString { odor_string: String::new() },
    "OPPOSED_TO_LIFE" => CasteTag::OpposedToLife,
    "ORIENTATION" => CasteTag::Orientation { caste: String::new(), disinterested_chance: 0, casual_chance: 0, strong_chance: 0 },
    "OUTSIDER_CONTROLLABLE" => CasteTag::OutsiderControllable,
    "PACK_ANIMAL" => CasteTag::PackAnimal,
    "PARALYZEIMMUNE" => CasteTag::ParalyzeImmune,
    "PATTERNFLIER" => CasteTag::PatternFlier,
    "PEARL" => CasteTag::Pearl,
    "PENETRATEPOWER" => CasteTag::PenetratePower { penetrate_power: 0 },
    "PERSONALITY" => CasteTag::Personality { personality_trait: String::new(), low: 0, median: 0, high: 0 },
    "PET" => CasteTag::Pet,
    "PET_EXOTIC" => CasteTag::PetExotic,
    "PETVALUE" => CasteTag::PetValue { pet_value: 0 },
    "PETVALUE_DIVISOR" => CasteTag::PetValueDivisor { divisor: 0 },
    "PHYS_ATT_CAP_PERC" => CasteTag::PhysicalAttributeCapPercentage { attribute: String::new(), percentage: 0 },
    "PHYS_ATT_RANGE" => CasteTag::PhysicalAttributeRange { attribute: String::new(), ranges: [0,0,0,0,0,0,0] },
    "PHYS_ATT_RATE" => CasteTag::PhysicalAttributeRate { attribute: String::new(), improvement_cost: 0, decay_rate_unused: 0, decay_rate_rusty: 0, decay_rate_demotion: 0 },
    "PLUS_BP_GROUP" => CasteTag::PlusBodyPartGroup { body_part_selector: String::new(), body_part_group: String::new() },
    "POP_RATIO" => CasteTag::PopulationRatio { pop_ratio: 0 },
    "POWER" => CasteTag::Power,
    "CASTE_PROFESSION_NAME" => CasteTag::ProfessionName { profession: String::new(), singular: String::new(), plural: String::new() },
    "PRONE_TO_RAGE" => CasteTag::ProneToRage { rage_chance: 0 },
    "PUS" => CasteTag::Pus { material: String::new(), material_state: String::new() },
    "RELATIVE_SIZE" => CasteTag::RelativeSize { body_part_selector: String::new(), body_part: String::new(), relative_size: 0 },
    "REMAINS" => CasteTag::Remains { singular: String::new(), plural: String::new() },
    "REMAINS_COLOR" => CasteTag::RemainsColor { remains_color: String::new() },
    "REMAINS_ON_VERMIN_BITE_DEATH" => CasteTag::RemainsOnVerminBiteDeath,
    "REMAINS_UNDETERMINED" => CasteTag::RemainsUndetermined,
    "RETRACT_INTO_BP" => CasteTag::RetractIntoBodyPart { body_part_selector: String::new(), body_part: String::new(), second_person: String::new(), third_person: String::new(), second_person_cancel: String::new(), third_person_cancel: String::new() },
    "RETURNS_VERMIN_KILLS_TO_OWNER" => CasteTag::ReturnsVerminKillsToOwner,
    "ROOT_AROUND" => CasteTag::RootAround { body_part_selector: String::new(), body_part: String::new(), second_person_verb: String::new(), third_person_verb: String::new() },
    "SECRETION" => CasteTag::Secretion { material_token: String::new(), material_state: String::new(), body_part_selector: String::new(), body_part: String::new(), tissue_layer: String::new(), trigger: String::new() },
    "SEMIMEGABEAST" => CasteTag::SemiMegabeast,
    "SENSE_CREATURE_CLASS" => CasteTag::SenseCreatureClass { creature_class: String::new(), tile: String::new(), foreground: 0, background: 0, brightness: 0 },
    "SET_BP_GROUP" => CasteTag::SetBodyPartGroup { body_part_selector: String::new(), body_part: String::new() },
    "SKILL_LEARN_RATE" => CasteTag::SkillLearnRate { skill: String::new(), rate: 0 },
    "SKILL_LEARN_RATES" => CasteTag::SkillLearnRates { rate: 0 },
    "SKILL_RATE" => CasteTag::SkillRate { skill: String::new(), improvement_rate: 0, decay_rate_unused: 0, decay_rate_rusty: 0, decay_rate_demotion: 0 },
    "SKILL_RATES" => CasteTag::SkillRates { improvement_rate: 0, decay_rate_unused: 0, decay_rate_rusty: 0, decay_rate_demotion: 0 },
    "SKILL_RUST_RATE" => CasteTag::SkillRustRate { skill: String::new(), decay_rate_unused: 0, decay_rate_rusty: 0, decay_rate_demotion: 0 },
    "SKILL_RUST_RATES" => CasteTag::SkillRustRates { decay_rate_unused: 0, decay_rate_rusty: 0, decay_rate_demotion: 0 },
    "SLAIN_CASTE_SPEECH" => CasteTag::SlainSpeech { speech_file: String::new() },
    "SLOW_LEARNER" => CasteTag::SlowLearner,
    "SMALL_REMAINS" => CasteTag::SmallRemains,
    "CASTE_SOLDIER_TILE" => CasteTag::SoldierTile { tile: String::new() },
    "CASTE_SOLDIER_ALTTILE" => CasteTag::SoldierAltTile { tile: String::new() },
    "SOUND" => CasteTag::Sound { sound_type: String::new(), sound_range: 0, sound_interval: 0, requires_breathing: false, first_person: String::new(), third_person: String::new(), out_of_sight: String::new() },
    "SPECIFIC_FOOD" => CasteTag::SpecificFood { is_plant: false, identifier: String::new() },
    "SPOUSE_CONVERSION_TARGET" => CasteTag::SpouseConversionTarget,
    "SPOUSE_CONVERTER" => CasteTag::SpouseConverter,
    "SPREAD_EVIL_SPHERES_IF_RULER" => CasteTag::SpreadEvilSpheresIfRuler,
    "STANCE_CLIMBER" => CasteTag::StanceClimber,
    "STANDARD_GRAZER" => CasteTag::StandardGrazer,
    "STRANGE_MOODS" => CasteTag::StrangeMoods,
    "SUPERNATURAL" => CasteTag::Supernatural,
    "SWIMS_INNATE" => CasteTag::SwimsInnate,
    "SWIMS_LEARNED" => CasteTag::SwimsLearned,
    "SYNDROME_DILUTION_FACTURE" => CasteTag::SyndromeDilutionFactor { syndrome: String::new(), percentage: 0 },
    "TENDONS" => CasteTag::Tendons { material: String::new(), healing_rate: 0 },
    "THICKWEB" => CasteTag::ThickWeb,
    "CASTE_TILE" => CasteTag::Tile { tile: String::new() },
    "TISSUE_LAYER" => CasteTag::TissueLayer { body_part_selector: String::new(), body_part: String::new(), tissue: String::new(), location: String::new() },
    "TISSUE_LAYER_OVER" => CasteTag::TissueLayer { body_part_selector: String::new(), body_part: String::new(), tissue: String::new(), location: String::new() },
    "TISSUE_LAYER_UNDER" => CasteTag::TissueLayerUnder { body_part_selector: String::new(), body_part: String::new(), tissue: String::new() },
    "TITAN" => CasteTag::Titan,
    "TRADE_CAPACITY" => CasteTag::TradeCapacity { capacity: 0 },
    "TRAINABLE" => CasteTag::Trainable,
    "TRAINABLE_HUNTING" => CasteTag::TrainableHunting,
    "TRAINABLE_WAR" => CasteTag::TrainableWar,
    "TRANCES" => CasteTag::Trances,
    "TRAPAVOID" => CasteTag::TrapAvoid,
    "UNDERSWIM" => CasteTag::UnderSwim,
    "UNIQUE_DEMON" => CasteTag::UniqueDemon,
    "VEGETATION" => CasteTag::Vegetation,
    "VERMIN_BITE" => CasteTag::VerminBite { chance: 0, verb: String::new(), material: String::new(), material_state: String::new() },
    "VERMIN_HATEABLE" => CasteTag::VerminHateable,
    "VERMIN_MICRO" => CasteTag::VerminMicro,
    "VERMIN_NOFISH" => CasteTag::VerminNoFish,
    "VERMIN_NOROAM" => CasteTag::VerminNoRoam,
    "VERMIN_NOTRAP" => CasteTag::VerminNoTrap,
    "VERMINHUNTER" => CasteTag::VerminHunter,
    "VESPERTINE" => CasteTag::Vespertine,
    "VIEWRANGE" => CasteTag::ViewRange { view_range: 0 },
    "VISION_ARC" => CasteTag::VisionArc { binocular: 0, non_binocular: 0 },
    "WAGON_PULLER" => CasteTag::WagonPuller,
    "WEBBER" => CasteTag::Webber { material: String::new() },
    "WEBIMMUNE" => CasteTag::WebImmune,
};
