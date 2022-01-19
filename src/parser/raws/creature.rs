use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use slug::slugify;

use super::names::{Name, SingPlurName};

// Creature Raw

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DFCreature {
    identifier: String,
    parent_raw: String,
    #[serde(rename = "objectId")]
    object_id: String,
    // Boolean Flags

    // Enables the creature to be kept in artificial hives by beekeepers.
    pub artificial_hiveable: bool,

    // Adding this token to a creature prevents it from appearing in generated worlds
    pub does_not_exist: bool,

    // The creature is considered evil and will only show up in evil biomes.
    pub evil: bool,

    // The creature is a thing of legend and known to all civilizations.
    pub fanciful: bool,

    // Found on procedurally generated creatures like forgotten beasts, titans, demons, angels, and night creatures.
    pub generated: bool,

    // Creature is considered good and will only show up in good biomes
    pub good: bool,

    // Creature can spawn as a wild animal in the appropriate biomes.
    pub large_roaming: bool,

    // Allows you to play as a wild animal of this species in adventurer mode.
    pub local_pops_controllable: bool,

    // Wild animals of this species may occasionally join a civilization.
    pub local_pops_produce_heroes: bool,

    // The creatures will scatter if they have this tag, or form tight packs if they don't.
    pub loose_clusters: bool,

    // Marks if the creature is an actual real-life creature. Only used for age-names at present.
    pub mundane: bool,

    // The creature will only show up in "savage" biomes.
    pub savage: bool,

    // Creature will occur in every region with the correct biome.
    pub ubiquitous: bool,

    // The vermin creature will attempt to eat exposed food.
    pub vermin_eater: bool,

    // The vermin appears in water and will attempt to swim around.
    pub vermin_fish: bool,

    // The creature appears in "general" surface ground locations.
    pub vermin_grounder: bool,

    // The vermin are attracted to rotting stuff and loose food left in the open and cause unhappy thoughts to dwarves who encounter them
    pub vermin_rotter: bool,

    // The creature randomly appears near dirt or mud
    pub vermin_soil: bool,

    // The vermin will appear in a single tile cluster of many vermin, such as a colony of ants.
    pub vermin_soil_colony: bool,

    // integers
    pub frequency: u16, //Defaults to 50 if not specified

    // [min, max] ranges
    pub cluster_number: [u16; 2],    //Defaults to 1:1 if not specified.
    pub population_number: [u16; 2], //default 1:1

    // strings
    pub general_baby_name: SingPlurName,
    pub general_child_name: SingPlurName,
    pub name: Name,

    // Listicle
    pub biomes: Vec<String>,
    pub pref_string: Vec<String>,

    // sub definitions
    pub castes: Vec<DFCreatureCaste>,

    // copy_from
    pub copy_tags_from: Vec<String>, // vec of creature identifiers
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DFCreatureCaste {
    // Identification
    name: String,
    // Boolean Flags

    // Prevents tamed creature from being made available for adoption,
    // instead allowing it to automatically adopt whoever it wants.
    pub adopts_owner: bool,

    // Found on [LARGE_PREDATOR]s who ambush their prey. Instead of charging relentlessly at prey,
    // the predator will wait till the prey is within a few squares before charging.
    pub ambush_predator: bool,

    // Allows a creature to breathe both in and out of water
    // (unlike [AQUATIC]) - does not prevent drowning in magma.
    pub amphibious: bool,

    // Enables the creature to breathe in water, but causes it to air-drown on dry land.
    pub aquatic: bool,

    // Causes the creature to be excluded from the object testing arena's creature spawning list.
    pub arena_restricted: bool,

    // 	Prevents the creature from attacking or frighten creatures with the [NATURAL] tag.
    pub at_peace_with_wildlife: bool,

    // The creature is non-aggressive by default, and will never automatically be engaged by companions
    // or soldiers, running away from any creatures that are not friendly to it, and will only defend
    // itself if it becomes enraged
    pub benign: bool,

    // Creature eats bones. Implies [CARNIVORE].
    pub bone_carnivore: bool,

    // Creature only eats meat. If the creature goes on rampages in worldgen, it will often devour the people/animals it kills.
    pub carnivore: bool,

    // When combined with any of [PET], [PACK_ANIMAL], [WAGON_PULLER] and/or [MOUNT], the creature is guaranteed
    // to be domesticated by any civilization with [COMMON_DOMESTIC_PET], [COMMON_DOMESTIC_PACK], [COMMON_DOMESTIC_PULL]
    // and/or [COMMON_DOMESTIC_MOUNT] respectively. Such civilizations will always have access to the creature,
    // even in the absence of wild populations.
    pub common_domestic: bool,

    // The creature can be cooked in meals without first being butchered/cleaned
    pub cookable_live: bool,

    // Found on generated demons. Marks the caste to be used in the initial wave after breaking into the underworld,
    // and by the demon civilizations created during world-gen breaches
    pub demon: bool,

    // Causes the creature to die upon attacking. Used by honey bees to simulate them dying after using their stingers.
    pub die_when_vermin_bite: bool,

    // Allows the creature to wear or wield items.
    pub equips: bool,

    // The creature can see regardless of whether it has working eyes and has full 360 degree vision,
    // making it impossible to strike the creature from a blind spot in combat.
    pub extravision: bool,

    // Found on forgotten beasts
    pub feature_beast: bool,

    // Makes the creature biologically female, enabling her to bear young.
    pub female: bool,

    // Makes the creature immune to FIREBALL and FIREJET attacks, and allows it to path through high temperature zones, like lava or fires
    pub fire_immune: bool,

    // Like [FIREIMMUNE], but also renders the creature immune to DRAGONFIRE attacks.
    pub fire_immune_super: bool,

    // The creature's corpse is a single FISH_RAW food item that needs to be cleaned (into a FISH item) at a fishery to become edible
    pub fish_item: bool,

    // Allows a creature to fly, independent of it having wings or not.
    pub flier: bool,

    // The creature can and will gnaw its way out of animal traps and cages using the specified verb,
    // depending on the material from which it is made (normally wood).
    pub gnawer: bool,

    //	The creature has nerves in its muscles. Cutting the muscle tissue can sever motor and sensory nerves.
    pub has_nerves: bool,

    // 	Creature hunts and kills nearby vermin.
    pub hunts_vermin: bool,

    // 	The creature cannot move.
    pub immobile: bool,

    // The creature is immobile while on land. Only works on [AQUATIC] creatures which can't breathe on land.
    pub immobile_land: bool,

    // The creature radiates fire. It will ignite, and potentially completely destroy, items the creature is standing on.
    pub immolate: bool,

    // Alias for [CAN_SPEAK] + [CAN_LEARN]
    pub intelligent: bool,

    // Will attack things that are smaller than it (like dwarves). Only one group of "large predators"
    // (possibly two groups on "savage" maps) will appear on any given map
    pub large_predator: bool,

    // 	Creature lays eggs instead of giving birth to live young.
    pub lays_eggs: bool,

    // The creature will generate light, such as in adventurer mode at night.
    pub light_gen: bool,

    // 	Lets a creature open doors that are set to forbidden in fortress mode.
    pub lock_picker: bool,

    // The creature is able to see while submerged in magma.
    pub magma_vision: bool,

    // 	Makes the creature biologically male.
    pub male: bool,

    // Makes the creature slowly stroll around, unless it's in combat or performing a job.
    pub meanderer: bool,

    // A 'boss' creature. A small number of the creatures are created during worldgen, their histories and descendants (if any)
    // will be tracked in worldgen (as opposed to simply 'spawning'), and they will occasionally go on rampages
    pub megabeast: bool,

    // The creature spawns stealthed and will attempt to path into the fortress, pulling any levers it comes across.
    pub mischievous: bool,

    // Creature may be used as a mount.
    pub mount: bool,

    // Creature may be used as a mount, but civilizations cannot domesticate it in worldgen without certain exceptions.
    pub mount_exotic: bool,

    // 	Allows the creature to have all-around vision as long as it has multiple heads that can see.
    pub multipart_full_vision: bool,

    // Makes the species usually produce a single offspring per birth, occasionally producing twins or triplets with a 1/500 chance.
    pub multiple_litter_rare: bool,

    // Animal is considered to be natural
    pub natural: bool,

    // Creature doesn't require connected body parts to move
    pub no_connections_for_movement: bool,

    // Creature cannot become dizzy
    pub no_dizziness: bool,

    // Creature does not need to drink
    pub no_drink: bool,

    // Creature does not need to eat
    pub no_eat: bool,

    // Creature cannot suffer fevers
    pub no_fevers: bool,

    // Creature is biologically sexless and unable to breed
    pub no_gender: bool,

    // Creature does not need to sleep
    pub no_sleep: bool,

    // Creature has no bones
    pub no_bones: bool,

    // Creature does not need to breath
    pub no_breathe: bool,

    // Creature has no emotions
    pub no_emotion: bool,

    // Creature can't become tired or over-exerted
    pub no_exert: bool,

    // Creature doesn't feel fear and will never flee from battle
    pub no_fear: bool,

    // Creature will not drop meat when butchered
    pub no_meat: bool,

    // Creature isn't nauseated by gut hits and cannot vomit
    pub no_nausea: bool,

    // Creature doesn't feel pain
    pub no_pain: bool,

    // Creature will not drop a hide when butchered
    pub no_skin: bool,

    // Creature will not drop a skull when butchered or when rot/decay of severed head
    pub no_skull: bool,

    // Creature doesn't produce miasma when rotting
    pub no_smelly_rot: bool,

    // Weapons can't get stuck in the creature
    pub no_stuck_ins: bool,

    // Creature cannot be stunned or knocked unconscious
    pub no_stun: bool,

    // Creature cannot be butchered
    pub not_butcherable: bool,

    // Cannot be raised from the dead, implies creature is not a normal living being
    pub not_living: bool,

    // Creature doesn't require a [THOUGHT] bodypart to survive
    pub no_thought: bool,

    // Is hostile to all creatures except undead and other non-living ones
    pub opposed_to_life: bool,

    // Lets you play as an outsider of this species in adventure mode.
    pub outsider_controllable: bool,

    // Allows the creature to be used as a pack animal.
    pub pack_animal: bool,

    // immune to all paralyzing special attacks.
    pub paralyzeimmune: bool,

    // Allows the creature to be tamed in Fortress mode.
    pub pet: bool,

    // Allows the creature to be tamed in Fortress mode.
    pub pet_exotic: bool,

    // Allows the being to represent itself as a deity
    pub power: bool,

    // Essentially the same as [MEGABEAST], but more of them are created during worldgen
    pub semi_megabeast: bool,

    // Shorthand for [CAN_LEARN] + [SKILL_LEARN_RATES:50]
    pub slow_learner: bool,

    // Creature leaves "remains" instead of a corpse. Used by vermin.
    pub small_remains: bool,

    //Acts as [GRAZER] but set to 20000*G*(max size)^(-3/4)
    pub standard_grazer: bool,

    // Gives the creature knowledge of any secrets with [SUPERNATURAL_LEARNING_POSSIBLE] that match its spheres.
    pub supernatural: bool,

    // 	The creature naturally knows how to swim perfectly and does not use the swimmer skill
    pub swims_innate: bool,

    // The creature swims only as well as their present swimming skill allows them to.
    pub swims_learned: bool,

    // 	The creature's webs can catch larger creatures.
    pub thick_web: bool,

    // 	Found on titans. Cannot be specified in user-defined raws.
    pub titan: bool,

    // Allows the creature to go into martial trances.
    pub trances: bool,

    // The creature will never trigger traps it steps on.
    pub trap_avoid: bool,

    // Found on generated demons; causes the game to create a single named instance of the demon
    pub unique_demon: bool,

    // Like [AT_PEACE_WITH_WILDLIFE], but also makes the creature more valued in artwork by civilisations with the PLANT sphere
    pub vegetation: bool,

    // Some dwarves will hate the creature and get unhappy thoughts when around it.
    pub vermin_hateable: bool,

    // This makes the creature move in a swarm of creatures of the same race as it (e.g. swarm of flies, swarm of ants).
    pub vermin_micro: bool,

    // 	The creature cannot be caught by fishing.
    pub vermin_no_fish: bool,

    //	The creature will not be observed randomly roaming about the map.
    pub vermin_no_roam: bool,

    // The creature cannot be caught in baited animal traps
    pub vermin_no_trap: bool,

    // 	Allows the creature to pull caravan wagons.
    pub wagon_puller: bool,

    // The creature will not get caught in thick webs.
    pub web_immune: bool,

    // [min, max] ranges
    pub clutch_size: [u16; 2], // Number of eggs laid in one sitting.
    pub litter_size: [u16; 2], // Number of live children birthed at once
    pub max_age: [u16; 2],     // creatures can lack this and are immortal

    // Combo flags (custom)
    pub active_time: u8, // MATUTINAL/DIURNAL/NOCTURNAL/CREPUSCULAR/VESPERTINE via binary math
    pub curious_beast: u8, // EATER/GUZZLER/ITEM via binary math
    pub no_season: u8,   // NO_SPRING/NO_SUMMER/NO_AUTUMN/NO_WINTER
    pub trainable: u8,   // trainable_HUNTING/trainable_WAR/BOTH(aka trainable)

    // Integer tokens

    // Age at which creature is considered a child, the default is zero.
    pub baby: u32,

    // Age at which creature is considered an adult
    pub child: u32,

    // Increases experience gain during adventure mode
    pub difficulty: u32,

    // Determines the size of laid eggs.
    pub egg_size: u32,

    // The value determines how rapidly grass is trampled when a creature steps on it
    pub grass_trample: u8,

    // The higher the number, the less frequently it needs to eat in order to live.
    pub grazer: u32,

    // Determines how well a creature can see in the dark - higher is better. Dwarves have 10000, which amounts to perfect nightvision.
    pub low_light_vision: u32,

    // How valuable a tamed animal is. Actual cost in points in the embarking screen is 1+(PETVALUE/2) for an untrained animal,
    // 1+PETVALUE for a war/hunting one.
    pub pet_value: u16,

    // Weighted population of caste; Lower is rarer.
    pub pop_ratio: u16,

    // String Tokens
    pub baby_name: SingPlurName,
    pub caste_name: Name,
    pub child_name: SingPlurName,
    pub description: String,

    // listicles
    pub creature_class: Vec<String>, // An arbitrary creature classification. Can be set to anything

    // Custom tokens
    pub body_size: Vec<DFBodySize>,
    pub milkable: DFMilkable,
}

// active time:
//      diurnal & nocturnal & crepuscular & matutinal & vespertine = 31
pub static ACTIVE_DIURNAL: u8 = 1;
pub static ACTIVE_NOCTURNAL: u8 = 2;
pub static ACTIVE_CREPUSCULAR: u8 = 4;
pub static ACTIVE_MATUTINAL: u8 = 8;
pub static ACTIVE_VESPERTINE: u8 = 16;

// curious beast:
//      eater & guzzler & item = 7
pub static CURIOUS_EATER: u8 = 1;
pub static CURIOUS_GUZZLER: u8 = 2;
pub static CURIOUS_ITEM: u8 = 4;

// "no" season (creature does not appear):
//      NO_SPRING & NO_SUMMER & NO_AUTUMN & NO_WINTER = 15
pub static NO_SPRING: u8 = 1;
pub static NO_SUMMER: u8 = 2;
pub static NO_FALL: u8 = 4;
pub static NO_WINTER: u8 = 8;

// trainable:
//      war & hunting = 3
pub static TRAINABLE_HUNTING: u8 = 1;
pub static TRAINABLE_WAR: u8 = 2;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DFBodySize {
    years: u32,
    days: u32,
    size_cm3: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DFMilkable {
    material: String,
    frequency: u32,
}

impl DFCreature {
    pub fn new(raw: &str, id: &str) -> Self {
        Self {
            identifier: String::from(id),
            parent_raw: String::from(raw),
            object_id: format!("{}-{}-{}", raw, "CREATURE", slugify(id)),
            // Boolean Flags
            artificial_hiveable: false,
            does_not_exist: false,
            evil: false,
            fanciful: false,
            generated: false,
            good: false,
            large_roaming: false,
            local_pops_controllable: false,
            local_pops_produce_heroes: false,
            loose_clusters: false,
            mundane: false,
            savage: false,
            ubiquitous: false,
            vermin_eater: false,
            vermin_fish: false,
            vermin_grounder: false,
            vermin_rotter: false,
            vermin_soil: false,
            vermin_soil_colony: false,

            // integers
            frequency: 50, //Defaults to 50 if not specified

            // [min, max] ranges
            cluster_number: [1, 1],    //Defaults to 1:1 if not specified.
            population_number: [1, 1], //default 1:1

            // strings
            general_baby_name: SingPlurName::new(String::new()),
            general_child_name: SingPlurName::new(String::new()),
            name: Name::new(String::new()),

            // Listicle
            biomes: Vec::new(),
            pref_string: Vec::new(),

            // sub definitions
            castes: Vec::new(),

            // copy_from
            copy_tags_from: Vec::new(), // vec of creature identifiers
        }
    }
    pub fn get_identifier(&self) -> String {
        String::from(&self.identifier)
    }
    pub fn get_parent_raw(&self) -> String {
        String::from(&self.parent_raw)
    }
    pub fn get_object_id(&self) -> String {
        String::from(&self.object_id)
    }
    pub fn get_all_names(&self) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        names.append(&mut self.name.to_string_vec());
        names.append(&mut self.general_baby_name.to_string_vec());
        names.append(&mut self.general_child_name.to_string_vec());
        for self_caste in &self.castes {
            names.append(&mut self_caste.baby_name.to_string_vec());
            names.append(&mut self_caste.child_name.to_string_vec());
        }
        names.retain(|s| s != "");
        names.sort_unstable();
        names.dedup();
        names
    }
    pub fn get_description(&self) -> String {
        let mut descriptions: Vec<String> = Vec::new();
        for self_caste in &self.castes {
            descriptions.push(String::from(&self_caste.description));
        }
        descriptions.join(" ")
    }
    pub fn get_max_ages(&self) -> HashMap<String, [u16; 2]> {
        let mut max_ages: HashMap<String, [u16; 2]> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.max_age[0] != self_caste.max_age[1] && self_caste.max_age[1] != 0 {
                max_ages.insert(String::from(&self_caste.name), self_caste.max_age);
            }
        }
        max_ages
    }
    pub fn get_clutch_sizes(&self) -> HashMap<String, [u16; 2]> {
        let mut clutch_sizes: HashMap<String, [u16; 2]> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.clutch_size[0] != self_caste.clutch_size[1]
                && self_caste.clutch_size[1] != 0
            {
                clutch_sizes.insert(String::from(&self_caste.name), self_caste.clutch_size);
            }
        }
        clutch_sizes
    }
    pub fn lays_eggs(&self) -> bool {
        for self_caste in &self.castes {
            if self_caste.lays_eggs {
                return true;
            }
        }
        false
    }
}

impl DFCreatureCaste {
    pub fn new(name: &str) -> Self {
        Self {
            // Identification
            name: String::from(name),
            // Boolean Flags
            adopts_owner: false,
            ambush_predator: false,
            amphibious: false,
            aquatic: false,
            arena_restricted: false,
            at_peace_with_wildlife: false,
            benign: false,
            bone_carnivore: false,
            carnivore: false,
            common_domestic: false,
            cookable_live: false,
            demon: false,
            die_when_vermin_bite: false,
            equips: false,
            extravision: false,
            feature_beast: false,
            female: false,
            fire_immune: false,
            fire_immune_super: false,
            fish_item: false,
            flier: false,
            gnawer: false,
            has_nerves: false,
            hunts_vermin: false,
            immobile: false,
            immobile_land: false,
            immolate: false,
            intelligent: false,
            large_predator: false,
            lays_eggs: false,
            light_gen: false,
            lock_picker: false,
            magma_vision: false,
            male: false,
            meanderer: false,
            megabeast: false,
            mischievous: false,
            mount: false,
            mount_exotic: false,
            multipart_full_vision: false,
            multiple_litter_rare: false,
            natural: false,
            no_connections_for_movement: false,
            no_dizziness: false,
            no_drink: false,
            no_eat: false,
            no_fevers: false,
            no_gender: false,
            no_sleep: false,
            no_bones: false,
            no_breathe: false,
            no_emotion: false,
            no_exert: false,
            no_fear: false,
            no_meat: false,
            no_nausea: false,
            no_pain: false,
            no_skin: false,
            no_skull: false,
            no_smelly_rot: false,
            no_stuck_ins: false,
            no_stun: false,
            not_butcherable: false,
            not_living: false,
            no_thought: false,
            opposed_to_life: false,
            outsider_controllable: false,
            pack_animal: false,
            paralyzeimmune: false,
            pet: false,
            pet_exotic: false,
            power: false,
            semi_megabeast: false,
            slow_learner: false,
            small_remains: false,
            standard_grazer: false, //Acts as [GRAZER] but set to 20000*G*(max size)^(-3/4)
            supernatural: false,
            swims_innate: false,
            swims_learned: false,
            thick_web: false,
            titan: false,
            trances: false,
            trap_avoid: false,
            unique_demon: false,
            vegetation: false,
            vermin_hateable: false,
            vermin_micro: false,
            vermin_no_fish: false,
            vermin_no_roam: false,
            vermin_no_trap: false,
            wagon_puller: false,
            web_immune: false,

            // [min, max] ranges
            clutch_size: [0, 0],
            litter_size: [0, 0],
            max_age: [0, 0],

            // Combo flags (custom)
            active_time: 0, // MATUTINAL/DIURNAL/NOCTURNAL/CREPUSCULAR/VESPERTINE via binary math
            curious_beast: 0, // EATER/GUZZLER/ITEM via binary math
            no_season: 0,   // NO_SPRING/NO_SUMMER/NO_AUTUMN/NO_WINTER
            trainable: 0,   // trainable_HUNTING/trainable_WAR/BOTH(aka trainable)

            // Integer tokens
            baby: 0,
            child: 0,
            difficulty: 0,
            egg_size: 0,
            grass_trample: 0,
            grazer: 0,
            low_light_vision: 0,
            pet_value: 0,
            pop_ratio: 0,

            // String Tokens
            baby_name: SingPlurName::new(String::new()),
            caste_name: Name::new(String::new()),
            child_name: SingPlurName::new(String::new()),
            description: String::new(),

            // listicles
            creature_class: Vec::new(),

            // Custom tokens
            body_size: Vec::new(),
            milkable: DFMilkable::new("", 0),
        }
    }
}

impl DFMilkable {
    pub fn new(material: &str, frequency: u32) -> Self {
        Self {
            material: String::from(material),
            frequency: frequency,
        }
    }
}

impl DFBodySize {
    pub fn new(years: u32, days: u32, size_cm3: u32) -> Self {
        Self {
            years: years,
            days: days,
            size_cm3: size_cm3,
        }
    }
}
