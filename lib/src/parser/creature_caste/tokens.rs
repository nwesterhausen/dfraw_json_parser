use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
/// Tokens that can be found in a creature's caste definitions.
pub enum CasteTag {
    /// Prevents tamed creature from being made available for adoption, instead allowing it to automatically adopt whoever it wants.
    /// The basic requirements for adoption are intact, and the creature will only adopt individuals who have a preference for their species.
    /// Used by cats in the vanilla game. When viewing a tame creature with this token, the message "This animal isn't interested in your
    /// wishes" will appear instead of "This adorable animal can't work" or "This animal is waiting to be trained".
    ///
    /// Appears as `ADOPTS_OWNER`
    AdoptsOwner,
    /// Makes the creature need alcohol to get through the working day; it will choose to drink booze instead of water if possible.
    /// Going sober for too long reduces speed.
    ///
    /// Appears as `ALCOHOL_DEPENDENT`
    AlcoholDependent,
    /// Sets the creature to be active during the day, night, and twilight in Adventurer Mode. Seems to be a separate value from `[DIURNAL]/[NOCTURNAL]/[CREPUSCULAR]`,
    /// rather than implying them.
    AllActive,
    /// Caste-specific version of [Creature::AltTile]. Requires [Tile].
    ///
    /// Appears as `CASTE_ALTTILE:SomeTile`
    AltTile {
        /// The tile to use
        tile: String,
    },
    /// Found on [LargePredator]s who ambush their prey. Instead of charging relentlessly at prey, the predator will wait till the prey is
    /// within a few squares before charging. May or may not work on other creatures.
    ///
    /// Appears as `AMBUSHPREDATOR`
    AmbushPredator,
    /// Allows a creature to breathe both in and out of water (unlike [Aquatic]) - does not prevent drowning in magma.
    ///
    /// Appears as `AMPHIBIOUS`
    Amphibious,
    /// Applies the specified creature variation with the given arguments to the creature. See [ApplyCreatureVariation] for more information.
    ///
    /// Appears as `APPLY_CREATURE_VARIATION:SOME_VARIATION` or `APPLY_CREATURE_VARIATION:SOME_VARIATION:ARG1:ARG2:ARG3`
    ApplyCreatureVariation {
        /// Creature variation ID to apply
        id: String,
        /// (Optional) any number of arguments to pass to the creature variation
        args: Vec<String>,
    },
    /// Applies the effects of all pending `[CV_ADD_TAG]` and `[CV_REMOVE_TAG]` tokens that have been defined in the current creature (so far).
    ///
    /// Appears as `APPLY_CURRENT_CREATURE_VARIATION`
    ApplyCurrentCreatureVariation,
    /// Enables the creature to breathe in water, but causes it to air-drown on dry land.
    ///
    /// Appears as `AQUATIC`
    Aquatic,
    /// Causes the creature to be excluded from the object testing arena's creature spawning list. Typically applied to spoileriffic creatures.
    ///
    /// Appears as `ARENA_RESTRICTED`
    ArenaRestricted,
    /// Prevents the creature from attacking or frightening creatures with the [Natural] tag.
    ///
    /// Appears as `AT_PEACE_WITH_WILDLIFE`
    AtPeaceWithWildlife,
    /// Defines the attack name, and the body part used.
    ///
    /// Appears as `ATTACK:NAME:BODYPART:BY_CATEGORY:HORN` or similar
    Attack {
        /// The name of the attack
        name: String,
        /// The body part used for the attack
        body_part: String,
    },
    /// Specifies when a megabeast or semi-megabeast will attack the fortress. The attacks will start occurring when at least one of the requirements is met.
    /// Setting a value to 0 disables the trigger.
    ///
    /// Appears as `ATTACK_TRIGGER:0:1:2`
    AttackTrigger {
        /// Population trigger
        population: u32,
        /// Exported wealth trigger
        exported_wealth: u32,
        /// Created wealth trigger
        created_wealth: u32,
    },
    /// Age at which creature is considered a child, the default is zero. One can think of this as the duration of the baby stage.
    ///
    /// Appears as `BABY:12`
    Baby {
        /// The age at which the creature is considered a child
        age: u32,
    },
    /// Defines a new name for a creature in baby state at the caste level. For non-caste-specific baby names, see [CreatureToken::GeneralBabyName].
    ///
    /// Appears as `BABYNAME:SomeName:SomeNames`
    BabyName {
        /// Singular name for the baby
        singular: String,
        /// Plural name for the baby
        plural: String,
    },
    /// Creature may be subject to beaching, becoming stranded on shores, where they will eventually air-drown. The number indicates the frequency of the occurrence.
    /// Presumably requires the creature to be [Aquatic]. Used by orcas, sperm whales and sea nettle jellyfish in the vanilla game.
    ///
    /// Appears as `BEACH_FREQUENCY:100`
    BeachFrequency {
        /// Frequency of beaching
        frequency: u32,
    },
    /// The creature is non-aggressive by default, and will never automatically be engaged by companions or soldiers, running away from any creatures
    /// that are not friendly to it, and will only defend itself if it becomes enraged. Can be thought of as the counterpoint of the [LargePredator] tag.
    /// When tamed, animals with this tag will be useless for fortress defense.
    ///
    /// Appears as `BENIGN`
    Benign,
    /// Select a biome the creature may appear in.
    ///
    /// Appears as `BIOME:SomeBiomeId`
    Biome {
        /// Biome identifier
        id: String,
    },
    /// Specifies what the creature's blood is made of.
    ///
    /// Appears as `BLOOD:SomeMaterial:SomeToken`
    Blood {
        /// Blood material
        material: String,
        /// Blood token
        state: String,
    },
    /// Causes vampire-like behavior; the creature will suck the blood of unconscious victims when its thirst for blood grows sufficiently large. When controlling the
    /// creature in adventure mode, this can be done at will. Seems to be required to make the creature denouncable (in-world) as a creature of the night.
    ///
    /// Appears as `BLOODSUCKER`
    BloodSucker,
    /// Draws body parts from `OBJECT:BODY` files (such as `body_default.txt`)
    ///
    /// Appears as `BODY:BODY_WITH_HEAD_FLAG:HEART:GUTS:BRAIN:MOUTH`
    Body {
        /// Body parts arguments
        body_parts: Vec<String>,
    },
    /// These body modifiers give individual creatures different characteristics. In the case of HEIGHT, BROADNESS and LENGTH, the modifier is also a percentage change to
    /// the BODY_SIZE of the individual creature. The seven numbers afterward give a distribution of ranges. Each interval has an equal chance of occurring.
    ///
    /// Example: `BODY_APPEARANCE_MODIFIER:HEIGHT:90:95:98:100:102:105:110`
    ///
    /// * `HEIGHT`: marks the height to be changed
    /// * `90:95:98:100:102:105:110`: sets the range from the shortest (90% of the average height) to the tallest (110% of the average height) creature variation.
    BodyAppearanceModifier {
        /// Body part to modify
        attribute: String,
        /// Range of values, spread from lowest to median to highest
        values: [i32; 7],
    },
    /// Loads a plan from listed OBJECT:BODY_DETAIL_PLAN files, such as b_detail_plan_default.txt. Mass applies USE_MATERIAL_TEMPLATE, mass alters RELSIZE, alters
    /// body part positions, and will allow tissue layers to be defined. Tissue layers are defined in order of skin to bone here.
    ///
    /// Example: `BODY_DETAIL_PLAN:VERTEBRATE_TISSUE_LAYERS:SKIN:FAT:MUSCLE:BONE:CARTILAGE`
    ///
    /// This creates the detailed body of a fox, the skin, fat, muscle, bones and cartilage out of the vertebrate tissues. A maggot would only need:
    ///
    /// `BODY_DETAIL_PLAN:EXOSKELETON_TISSUE_LAYERS:SKIN:FAT:MUSCLE`
    BodyDetailPlan {
        /// Body detail plan to load
        body_plan: String,
        /// Body detail plan arguments
        arguments: Vec<String>,
    },
    /// Sets size at a given age. Size is in cubic centimeters, and for normal body materials, is roughly equal to the creature's average weight in grams.
    ///
    /// Appears as `BODY_SIZE:0:0:1000`
    BodySize {
        /// Year at which the size is set
        year: u32,
        /// Days at which the size is set
        days: u32,
        /// Size in cubic centimeters
        size: u32,
    },
    /// Substitutes body part text with replacement text. Draws gloss information from OBJECT:BODY files (such as body_default.txt)
    ///
    /// Appears as `BODYGLOSS:SomeGloss`
    BodyGloss {
        /// The gloss to use on the body part
        gloss: String,
    },
    /// Creature eats bones. Implies [Carnivore]. Adventurers with this token are currently unable to eat bones.
    ///
    /// Appears as `BONECARN`
    BoneCarn,
    /// Adds a type to a body part - used with [SetBodyPartGroup]. In vanilla DF, this is used for adding the type [Geldable] to the lower body of certain creatures.
    ///
    /// Appears as `BP_ADD_TYPE:SomeBodyPartType`
    BodyPartAddType {
        /// The body part type to add
        body_part_type: String,
    },
    /// Sets up the breadth of possibilities for appearance qualities for a selected BP group. EG.
    ///
    /// * Eyes (CLOSE_SET, DEEP_SET, ROUND_VS_NARROW, LARGE_IRIS)
    /// * Lips (THICKNESS)
    /// * Nose (BROADNESS, LENGTH, UPTURNED, CONVEX)
    /// * Ear (SPLAYED_OUT, HANGING_LOBES, BROADNESS, HEIGHT)
    /// * Tooth (GAPS)
    /// * Skull (HIGH_CHEEKBONES, BROAD_CHIN, JUTTING CHIN, SQUARE_CHIN)
    /// * Neck (DEEP_VOICE, RASPY_VOICE)
    /// * Head (BROADNESS, HEIGHT)
    ///
    /// Appears as `BP_APPEARANCE_MODIFIER:SomeQuality:0:0:0:0:0:0:0`
    BodyPartAppearanceModifier {
        /// The quality that can appear
        quality: String,
        /// The spread of the quality, from lowest to median to highest
        spread: [i32; 7],
    },
    /// Removes a type from a body part. Used with [SetBodyPartGroup].
    ///
    /// Appears as `BP_REMOVE_TYPE:SomeBodyPartType`
    BodyPartRemoveType {
        /// The body part type to remove
        body_part_type: String,
    },
    /// Allows a creature to destroy furniture and buildings. Value `1` targets mostly doors, hatches, furniture and the like. Value `2` targets
    /// anything not made with the b + C commands.
    ///
    /// Appears as `BUILDINGDESTROYER:1`
    BuildingDestroyer {
        /// Whether the creature focuses on doors, hatches, furniture, etc. (`1`) or anything not made with the b + C commands (`2`)
        door_and_furniture_focused: bool,
    },
    /// The creature can perform an interaction.
    ///
    /// Appears as `CAN_DO_INTERACTION:SomeInteraction`
    CanDoInteraction {
        /// Interaction to allow
        interaction: String,
    },
    /// The creature gains skills and can have professions. If a member of a civilization (even a pet) has this token, it'll need to eat, drink and sleep.
    /// Note that this token makes the creature unable to be eaten by an adventurer, so it is not recommended for uncivilized monsters. Adventurers lacking
    /// this token can allocate but not increase attributes and skills. Skills allocated will disappear on start.
    ///
    /// Appears as `CAN_LEARN`
    CanLearn,
    /// Can talk. Note that this is not necessary for a creature to gain social skills.
    ///
    /// Appears as `CAN_SPEAK`
    CanSpeak,
    /// Creature cannot climb, even if it has free grasp parts.
    ///
    /// Appears as `CANNOT_CLIMB`
    CannotClimb,
    /// Creature cannot jump.
    ///
    /// Appears as `CANNOT_JUMP`
    CannotJump,
    /// Acts like [NotLiving], except that [OpposedToLife] creatures will attack them.
    ///
    /// Appears as `CANNOT_UNDEAD`
    CannotUndead,
    /// Allows the creature to open doors that are set to be unpassable for pets. In adventure mode, creatures lacking this token will be unable to pass through door
    /// tiles except whilst these are occupied by other creatures. Not currently useful in Fortress mode as doors can no longer be set unpassable for pets.
    ///
    /// Appears as `CANOPENDOORS`
    CanOpenDoors,
    /// Creature only eats meat. If the creature goes on rampages in worldgen, it will often devour the people/animals it kills.
    /// Does not seem to affect the diet of the adventurer in adventure mode.
    ///
    /// Appears as `CARNIVORE`
    Carnivore,
    /// Gives the creature a bonus in caves. Also causes cave adaptation.
    ///
    /// Appears as `CAVE_ADAPT`
    CaveAdaptation,
    /// Multiplies body size by a factor of (integer)%. 50 halves size, 200 doubles.
    ///
    /// Appears as `CHANGE_BODY_SIZE_PERC:100`
    ChangeBodySizePercent {
        /// The percentage to change the body size by
        percent: u32,
    },
    /// Age at which creature is considered an adult - one can think of this as the duration of the child stage. Allows the creature's offspring to be
    /// rendered fully tame if trained during their childhood.
    ///
    /// Appears as `CHILD:12`
    Child {
        /// The age at which the creature is considered an adult
        age: u32,
    },
    /// Defines a name for the creature in child state at the caste level. For non-caste-specific child names, see [CreatureToken::GeneralChildName].
    ///
    /// Appears as `CHILDNAME:SomeName:SomeNames`
    ChildName {
        /// Singular name for the child
        singular: String,
        /// Plural name for the child
        plural: String,
    },
    /// Number of eggs laid in one sitting.
    ///
    /// Appears as `CLUTCH_SIZE:1:1`
    ClutchSize {
        /// Minimum number of eggs laid in one sitting
        min: u32,
        /// Maximum number of eggs laid in one sitting
        max: u32,
    },
    /// Caste-specific color
    ///
    /// Arguments:
    ///
    /// * `foreground`: The foreground color
    /// * `background`: The background color
    /// * `brightness`: The brightness of the color
    ///
    /// Appears as `CASTE_COLOR:0:0:0`
    Color {
        /// The foreground color
        foreground: u32,
        /// The background color
        background: u32,
        /// The brightness of the color
        brightness: u32,
    },
    /// When combined with any of [Pet], [PackAnimal], [WagonPuller] and/or [Mount], the creature is guaranteed to be domesticated by any civilization with
    /// [EntityToken::CommonDomesticPet], [EntityToken::CommonDomesticPackAnimal], [EntityToken::CommonDomesticWagonPuller] and/or [EntityToken::CommonDomesticMount]
    /// respectively. Such civilizations will always have access to the creature, even in the absence of wild populations. This token is invalid on [CreatureToken::Fanciful] creatures.
    ///
    /// Appears as `COMMON_DOMESTIC`
    CommonDomestic,
    /// Creatures of this caste's species with the [SpouseConverter] and [NightCreatureHunter] tokens will kidnap [SpouseConversionTarget]s of an appropriate
    /// sex and convert them into castes with CONVERTED_SPOUSE.
    ///
    /// Appears as `CONVERTED_SPOUSE`
    ConvertedSpouse,
    /// Set this to allow the creature to be cooked in meals without first being butchered/cleaned. Used by some water-dwelling vermin such as mussels, nautiluses and oysters.
    ///
    /// Appears as `COOKABLE_LIVE`
    CookableLive,
    /// Creature is 'berserk' and will attack all other creatures, except members of its own species that also have the CRAZED tag. It will show Berserk in the unit list.
    /// Berserk creatures go on rampages during worldgen much more frequently than non-berserk ones.
    ///
    /// Appears as `CRAZED`
    Crazed,
    /// An arbitrary creature classification. Can be set to anything, but the only vanilla uses are `GENERAL_POISON` (used in syndromes), `EDIBLE_GROUND_BUG`
    /// (used as targets for [GobbleVerminClass]), `MAMMAL`, and `POISONOUS` (both used for kobold pet eligibility). A single creature can have multiple classes.
    ///
    /// Appears as `CREATURE_CLASS:SomeClass`
    CreatureClass,
    /// Sets the creature to be active at twilight in adventurer mode.
    ///
    /// Appears as `CREPUSCULAR`
    Crepuscular,
    /// Allows a creature to steal and eat edible items from a site. It will attempt to grab a food item and immediately make its way to the map's edge,
    /// where it will disappear with it. If the creature goes on rampages during worldgen, it will often steal food instead of attacking. Trained and tame instances
    /// of the creature will no longer display this behavior.
    ///
    /// Appears as `CURIOUSBEAST_EATER`
    CuriousBeastEater,
    /// Allows a creature to (very quickly) drink your alcohol. Or spill the barrel to the ground. Also affects undead versions of the creature. Unlike food or item thieves,
    /// drink thieves will consume your alcohol on the spot rather than run away with one piece of it. Trained and tame instances of the creature will no longer display this behavior.
    ///
    /// Appears as `CURIOUSBEAST_GUZZLER`
    CuriousBeastGuzzler,
    /// Allows a creature to steal things (apparently, of the highest value it can find). It will attempt to grab an item of value and immediately make its way to the map's edge,
    /// where it will disappear with it. If a creature with any of the CURIOUSBEAST tokens carries anything off the map, even if it is a caravan's pack animal, it will be reported
    /// as stealing everything it carries. If the creature goes on rampages in worldgen, it will often steal items instead of attacking - kea birds are infamous for this.
    /// Trained and tame instances of the creature will no longer display this behavior. Also, makes the creature unable to drop hauled items until it enters combat.
    ///
    /// Appears as `CURIOUSBEAST_ITEM`
    CuriousBeastItem,
    /// Adds a tag. Used in conjunction with creature variation templates.
    ///
    /// Appears as `CV_ADD_TAG:SomeTag`
    CreatureVariationAddTag {
        /// The tag to add
        tag: String,
    },
    /// Removes a tag. Used in conjunction with creature variation templates.
    ///
    /// Appears as `CV_REMOVE_TAG:SomeTag`
    CreatureVariationRemoveTag {
        /// The tag to remove
        tag: String,
    },
    /// Found on generated demons. Marks the caste to be used in the initial wave after breaking into the underworld, and by the demon civilizations created during world-gen breachings
    ///
    /// Appears as `DEMON`
    Demon,
    /// A brief description of the creature type, as displayed when viewing the creature's description/thoughts & preferences screen.
    Description {
        /// The description to use
        description: String,
    },
    /// Causes the creature to die upon attacking. Used by honey bees to simulate them dying after using their stingers.
    ///
    /// Appears as `DIE_WHEN_VERMIN_BITE`
    DieWhenVerminBite,
    /// Increases experience gain during adventure mode. Creatures with a difficulty of 11 or higher are not assigned for quests in adventure mode.
    ///
    /// Appears as `DIFFICULTY:10`
    Difficulty {
        /// The difficulty of the creature
        difficulty: u32,
    },
    /// Sets the creature to be active during the day in Adventurer Mode.
    ///
    /// Appears as `DIURNAL`
    Diurnal,
    /// The creature hunts vermin by diving from the air. On tame creatures, it has the same effect as [HuntsVermin]. Found on peregrine falcons.
    ///
    /// Appears as `DIVE_HUNTS_VERMIN`
    DiveHuntsVermin,
    /// Defines the item that the creature drops upon being butchered. Used with [ExtraButcherObject].
    ///
    /// Appears as `EBO_ITEM:SomeItem:SomeMaterial`
    ExtraButcherObjectItem {
        /// The item to add
        item: String,
        /// The material of the item
        material: String,
    },
    /// The shape of the creature's extra butchering drop. Used with [ExtraButcherObject].
    ///
    /// Appears as `EBO_SHAPE:SomeShape`
    ExtraButcherObjectShape {
        /// The shape to add
        shape: String,
    },
    /// Defines the material composition of eggs laid by the creature. Egg-laying creatures in the default game define this 3 times, using LOCAL_CREATURE_MAT:EGGSHELL,
    /// LOCAL_CREATURE_MAT:EGG_WHITE, and then LOCAL_CREATURE_MAT:EGG_YOLK. Eggs will be made out of eggshell. Edibility is determined by tags on whites or yolk,
    /// but they otherwise do not exist.
    ///
    /// Appears as `EGG_MATERIAL:SomeMaterial:SomeState`
    EggMaterial {
        /// The material to use
        material: String,
        /// The state of the material
        state: String,
    },
    /// Determines the size of laid eggs. Doesn't affect hatching or cooking, but bigger eggs will be heavier, and may take longer to be hauled depending on the hauler's strength.
    ///
    /// Appears as `EGG_SIZE:100`
    EggSize {
        /// The size of the egg
        size: u32,
    },
    /// Allows the creature to wear or wield items.
    ///
    /// Appears as `EQUIPS`
    Equips,
    /// The creature drops an additional object when butchered, as defined by [ExtraButcherObjectItem] and [ExtraButcherObjectShape].
    /// Used for gizzard stones in default creatures. For some materials, needs to be defined after caste definitions with SELECT_CASTE:ALL
    ///
    /// Appears as `EXTRA_BUTCHER_OBJECT`
    ExtraButcherObject {
        /// Details about the extra butcher object
        object_type: String,
        /// Arguments for the extra butcher object
        arguments: Vec<String>,
    },
    /// Defines a creature extract which can be obtained via small animal dissection.
    ///
    /// Appears as `EXTRACT:SomeExtract`
    Extract {
        /// The extract material
        material: String,
    },
    /// The creature can see regardless of whether it has working eyes and has full 360 degree vision, making it impossible to strike the creature from a blind spot
    /// in combat. Invisible creatures will also be seen, namely intelligent undead using a "vanish" power.
    ///
    /// Appears as `EXTRAVISION`
    Extravision,
    /// Found on subterranean animal-man tribals. Currently defunct. In previous versions, it caused these creatures to crawl out of chasms and underground rivers.
    ///
    /// Appears as `FEATURE_ATTACK_GROUP`
    FeatureAttackGroup,
    /// Found on forgotten beasts. Presumably makes it act as such, initiating underground attacks on fortresses, or leads to the pop-up message upon encountering one.
    /// Hides the creature from displaying in a world_sites_and_pops file. Does not create historical figures like generated forgotten beasts do.
    ///
    /// Requires specifying a [Biome] in which the creature will live, and both surface and subterranean biomes are allowed. Does not stack with [LargeRoaming] and if
    /// both are used the creature will not spawn. Appears to be incompatible with [Demon] even if used in separate castes.
    ///
    /// Appears as `FEATURE_BEAST`
    FeatureBeast,
    /// Makes the creature biologically female, enabling her to bear young. Usually specified inside a caste.
    ///
    /// Appears as `FEMALE`
    Female,
    /// Makes the creature immune to FIREBALL and FIREJET attacks, and allows it to path through high temperature zones, like lava or fires. Does not, by itself,
    /// make the creature immune to the damaging effects of burning in fire, and does not prevent general heat damage or melting on its own (this would require adjustments
    /// to be made to the creature's body materials - see the dragon raws for an example).
    ///
    /// Appears as `FIREIMMUNE`
    FireImmune,
    /// Like [FireImmune], but also renders the creature immune to DRAGONFIRE attacks.
    ///
    /// Appears as `FIREIMMUNE_SUPER`
    FireImmuneSuper,
    /// The creature's corpse is a single FISH_RAW food item that needs to be cleaned (into a FISH item) at a fishery to become edible. Before being cleaned the item is
    /// referred to as "raw". The food item is categorized under "fish" on the food and stocks screens, and when uncleaned it is sorted under "raw fish" in the stocks
    /// (but does not show up on the food screen).
    ///
    /// Without this or [CookableLive], fished vermin will turn into food the same way as non-vermin creatures, resulting in multiple units of food (meat, brain, lungs,
    /// eyes, spleen etc.) from a single fished vermin. These units of food are categorized as meat by the game.
    ///
    /// Appears as `FISHITEM`
    FishItem,
    /// The creature's body is constantly at this temperature, heating up or cooling the surrounding area. Alters the temperature of the creature's inventory and all
    /// adjacent tiles, with all the effects that this implies - may trigger wildfires at high enough values. Also makes the creature immune to extreme heat or cold, as
    /// long as the temperature set is not harmful to the materials that the creature is made from. Corpses and body parts of creatures with a fixed temperature maintain
    /// their temperature even after death.
    ///
    /// Note that temperatures of 12000 and higher may cause pathfinding issues in fortress mode.
    ///
    /// Appears as `FIXED_TEMP:10000`
    FixedTemp {
        /// The temperature of the creature
        temperature: i32,
    },
    /// If engaged in combat, the creature will flee at the first sign of resistance. Used by kobolds in the vanilla game.
    ///
    /// Appears as `FLEEQUICK`
    FleeQuick,
    /// Allows a creature to fly, independent of it having wings or not. Fortress Mode pathfinding only partially incorporates flying - flying creatures need a land path
    /// to exist between them and an area in order to access it, but as long as one such path exists, they do not need to use it, instead being able to fly over intervening
    /// obstacles. Winged creatures with this token can lose their ability to fly if their wings are crippled or severed. Winged creatures without this token will be unable
    /// to fly. (A 'wing' in this context refers to any body part with its own FLIER token).
    ///
    /// Appears as `FLIER`
    Flier,
    /// Caste-specific glow color.
    ///
    /// Arguments:
    ///
    /// * `foreground`: The foreground color
    /// * `background`: The background color
    /// * `brightness`: The brightness of the color
    ///
    /// Appears as `CASTE_GLOWCOLOR:0:0:0`
    GlowColor {
        /// The foreground color
        foreground: u32,
        /// The background color
        background: u32,
        /// The brightness of the color
        brightness: u32,
    },
    /// Caste-specific glow tile.
    ///
    /// Appears as `CASTE_GLOWTILE:SomeTile`
    GlowTile {
        /// The tile to use
        tile: String,
    },
    ///
    Gnawer,
    ///
    HasNerves,
    ///
    HuntsVermin,
    /// Specifies interaction details following a [CanDoInteraction] token.
    ///
    /// Appears as `CDI:SomeArgs:etc`
    InteractionDetail {
        /// Arbitrary arguments for the interaction
        args: Vec<String>,
    },
    ///
    Immobile,
    ///
    ImmobileLand,
    ///
    Immolate,
    ///
    Intelligent,
    ///
    LaysEggs,
    ///
    LightGen,
    ///
    LockPicker,
    ///
    MagmaVision,
    ///
    Male,
    ///
    Meanderer,
    ///
    Megabeast,
    ///
    Mischievous,
    ///
    Mount,
    ///
    MountExotic,
    ///
    MultipartFullVision,
    ///
    MultipleLitterRare,
    /// Name of the caste
    ///
    /// Arguments:
    ///
    /// * `singular`: The singular name of the caste
    /// * `plural`: The plural name of the caste
    /// * `adjective`: The adjective form of the caste
    ///
    /// Appears as `CASTE_NAME:SomeName:SomeNames:SomeAdjective`
    Name {
        /// The singular name of the caste
        singular: String,
        /// The plural name of the caste
        plural: String,
        /// The adjective form of the caste
        adjective: String,
    },
    ///
    Natural,
    ///
    NoConnectionsForMovement,
    ///
    NoDizziness,
    ///
    NoDrink,
    ///
    NoEat,
    ///
    NoFevers,
    ///
    NoGender,
    ///
    NoSleep,
    ///
    NoBones,
    ///
    NoBreathe,
    ///
    NoEmotion,
    ///
    NoExert,
    ///
    NoFear,
    ///
    NoMeat,
    ///
    NoNausea,
    ///
    NoPain,
    ///
    NoSkin,
    ///
    NoSkull,
    ///
    NoSmellyRot,
    ///
    NoStuckIns,
    ///
    NoStun,
    ///
    NotButcherable,
    ///
    NotLiving,
    ///
    NoThought,
    ///
    OpposedToLife,
    ///
    OutsiderControllable,
    ///
    PackAnimal,
    ///
    ParalyzeImmune,
    ///
    Pet,
    ///
    PetExotic,
    ///
    Power,
    /// Caste-specific profession name.
    ///
    /// Arguments:
    ///
    /// * `profession`: The profession name / unit type token ID
    /// * `singular`: The singular name of the profession
    /// * `plural`: The plural name of the profession
    ///
    /// Appears as `CASTE_PROFESSION_NAME:SomeProfession:SomeName:SomeNames`
    ProfessionName {
        /// The profession name / unit type token ID
        profession: String,
        /// The singular name of the profession
        singular: String,
        /// The plural name of the profession
        plural: String,
    },
    ///
    SemiMegabeast,
    ///
    SlowLearner,
    ///
    SmallRemains,
    /// Acts as [GRAZER] but set to 20000*G*(max size)^(-3/4)
    StandardGrazer,
    /// Caste-specific solider tile.
    ///
    /// Appears as `CASTE_SOLDIER_TILE:SomeTile`
    SoldierTile {
        /// The tile to use
        tile: String,
    },
    /// Caste-specific solider alt tile.
    ///
    /// Appears as `CASTE_SOLDIER_ALTTILE:SomeTile`
    SoldierAltTile {
        /// The tile to use
        tile: String,
    },
    ///
    Supernatural,
    ///
    SwimsInnate,
    ///
    SwimsLearned,
    ///
    ThickWeb,
    /// Caste-specific tile.
    ///
    /// Appears as `CASTE_TILE:SomeTile`
    Tile {
        /// The tile to use
        tile: String,
    },
    ///
    Titan,
    ///
    Trances,
    ///
    TrapAvoid,
    ///
    Trainable,
    ///
    TrainableHunting,
    ///
    TrainableWar,
    ///
    NoSpring,
    ///
    NoSummer,
    ///
    NoFall,
    ///
    NoWinter,
    ///
    UniqueDemon,
    ///
    Vegetation,
    ///
    VerminHateable,
    ///
    VerminMicro,
    ///
    VerminNoFish,
    ///
    VerminNoRoam,
    ///
    VerminNoTrap,
    ///
    WagonPuller,
    ///
    WebImmune,
    ///
    Milkable,
    ///
    GrassTrample,
    ///
    Grazer,
    ///
    LowLightVision,
    ///
    PetValue,
    ///
    PopRatio,
    ///
    LitterSize,
    ///
    MaxAge,
    #[default]
    Unknown,
    // Tokens found in the XML exports..
    NightCreature,
    NightCreatureHunter,
    NightCreatureBogeyman,
    NightCreatureNightmare,
    LargePredator,
    NotFireImmune,
    HasBlood,
    Grasp,
    RaceGait,
    CannotBreatheWater,
    NaturalAnimal,
    CuriousBeast,
    CannotBreatheAir,
    Utterances,
    Gait,
}
