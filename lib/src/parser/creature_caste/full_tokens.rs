pub enum FullToken {
    /// Prevents tamed creature from being made available for adoption, instead allowing it to automatically adopt whoever it wants.
    /// The basic requirements for adoption are intact, and the creature will only adopt individuals who have a preference for their
    /// species. Used by cats in the vanilla game. When viewing a tame creature with this token, the message "This animal isn't
    /// interested in your wishes" will appear instead of "This adorable animal can't work" or "This animal is waiting to be trained".
    AdoptsOwner,
    /// Makes the creature need alcohol to get through the working day; it will choose to drink booze instead of water if possible.
    /// Going sober for too long reduces speed.
    AlcoholDependent,
    /// Sets the creature to be active during the day, night, and twilight in Adventurer Mode. Seems to be a separate value from
    /// `[DIURNAL]/[NOCTURNAL]/[CREPUSCULAR]``, rather than implying them.
    AllActive,
    /// Found on [LARGE_PREDATOR]s who ambush their prey. Instead of charging relentlessly at prey, the predator will wait till the
    /// prey is within a few squares before charging. May or may not work on other creatures.
    AmbushPredator,
    /// Allows a creature to breathe both in and out of water (unlike `[AQUATIC]`) - does not prevent drowning in magma.
    Amphibious,
    /// Applies the specified creature variation. See Creature_variation_token#Arguments_and_conditional_tokens for how the subsequent
    /// arguments may be used.
    ApplyCreatureVariation {
        /// The creature variation to apply.
        variation: String,
        /// The arguments to pass to the creature variation.
        arguments: Vec<String>,
    },
    /// Applies the effects of all pending `[CV_ADD_TAG]` and `[CV_REMOVE_TAG]` tokens that have been defined in the current creature.
    ApplyCurrentCreatureVariation,
    /// Enables the creature to breathe in water, but causes it to air-drown on dry land.
    Aquatic,
    /// Causes the creature to be excluded from the object testing arena's creature spawning list. Typically applied to spoileriffic creatures.
    ArenaRestricted,
    /// Prevents the creature from attacking or frightening creatures with the `[NATURAL]` tag.
    AtPeaceWithWildlife,
    /// Defines the attack name and the body part used
    Attack {
        /// The name of the attack
        name: String,
        /// The body part used for the attack
        /// e.g `BODYPART:BY_CATEGORY:HORN` = the horn is used to attack (presuming the creature has one)
        body_part_selector: String,
    },
    /// Specifies when a megabeast or semi-megabeast will attack the fortress. The attacks will start occurring when at least one of the
    /// requirements is met. Setting a value to 0 disables the trigger.
    AttackTrigger {
        /// The number of dwarves in the fortress
        population: u32,
        /// The amount of traded wealth
        exported_wealth: u32,
        /// The amount of wealth created in total
        created_wealth: u32,
    },
    /// Age at which creature is considered a child, the default is zero. One can think of this as the duration of the baby stage.
    Baby {
        /// The age at which the creature is considered a child
        age: u32,
    },
    /// Defines a new name for a creature in baby state at the caste level. For non-caste-specific baby names, see `[GENERAL_BABY_NAME]`
    BabyName {
        /// The name of the baby
        singular: String,
        /// The plural of the baby name
        plural: String,
    },
    /// Creature may be subject to beaching, becoming stranded on shores, where they will eventually air-drown. The number indicates the frequency
    /// of the occurrence. Presumably requires the creature to be `[AQUATIC]`. Used by orcas, sperm whales and sea nettle jellyfish in the vanilla game.
    BeachFrequency {
        /// The frequency of the beaching
        frequency: u32,
    },
    /// The creature is non-aggressive by default, and will never automatically be engaged by companions or soldiers, running away from any creatures
    /// that are not friendly to it, and will only defend itself if it becomes enraged. Can be thought of as the counterpoint of the `[LARGE_PREDATOR]` tag.
    /// When tamed, animals with this tag will be useless for fortress defense.
    Benign,
    /// Specifies what the creature's blood is made of.
    Blood {
        /// The material token of the creature's blood
        material_token: String,
        /// The material state of the creature's blood
        material_state: String,
    },
    /// Causes vampire-like behavior; the creature will suck the blood of unconscious victims when its thirst for blood grows sufficiently large.
    /// When controlling the creature in adventure mode, this can be done at will. Seems to be required to make the creature denouncable (in-world)
    /// as a creature of the night.
    BloodSucker,
    /// Draws body parts from `OBJECT:BODY` files (such as `body_default.txt`)
    /// e.g. `[BODY:BODY_WITH_HEAD_FLAG:HEART:GUTS:BRAIN:MOUTH]`
    /// This is the body from a purring maggot. It creates a body with head, a heart, some guts, a brain, and a mouth. That's all a maggot needs.
    Body {
        /// The body part to draw from the `OBJECT:BODY` file
        body_parts: Vec<String>,
    },
    /// These body modifiers give individual creatures different characteristics. In the case of HEIGHT, BROADNESS and LENGTH, the modifier is
    /// also a percentage change to the BODY_SIZE of the individual creature. The seven numbers afterward give a distribution of ranges. Each interval
    /// has an equal chance of occurring.
    BodyAppearanceModifier {
        attribute: String,
        lowest: i32,
        lower: i32,
        low: i32,
        median: i32,
        high: i32,
        higher: i32,
        highest: i32,
    },
    /// Loads a plan from listed `OBJECT:BODY_DETAIL_PLAN` files, such as `b_detail_plan_default.txt`. Mass applies `USE_MATERIAL_TEMPLATE`,
    /// mass alters `RELSIZE`, alters body part positions, and will allow tissue layers to be defined. Tissue layers are defined in order of skin to bone here.
    BodyDetailPlan {
        plan_name: String,
        arguments: Vec<String>,
    },
    /// Sets size at a given age. Size is in cubic centimeters, and for normal body materials, is roughly equal to the creature's average weight in grams.
    BodySize {
        /// The years to be added with the days to get the age of the creature
        years: u32,
        /// The days to be added with the years to get the age of the creature
        days: u32,
        /// The size of the creature at the given age
        size: u32,
    },
    /// Substitutes body part text with replacement text. Draws gloss information from `OBJECT:BODY` files (such as `body_default.txt`)
    BodyGloss {
        /// The gloss of the body
        gloss: String,
    },
    /// Creature eats bones. Implies `[CARNIVORE]`. Adventurers with this token are currently unable to eat bones.
    BoneCarnivore,
    /// Adds a type to a body part - used with `[SET_BP_GROUP]`. In vanilla DF, this is used for adding the type
    /// `'GELDABLE'` to the lower body of certain creatures.
    BodyPartAddType { part_type: String },
}
