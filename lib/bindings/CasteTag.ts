// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

/**
 * Tokens that can be found in a creature's caste definitions.
 */
export type CasteTag =
  | "AdoptsOwner"
  | "AlcoholDependent"
  | "AllActive"
  | {
      AltTile: {
        /**
         * The tile to use
         */
        tile: string;
      };
    }
  | "AmbushPredator"
  | "Amphibious"
  | {
      ApplyCreatureVariation: {
        /**
         * Creature variation ID to apply
         */
        id: string;
        /**
         * (Optional) any number of arguments to pass to the creature variation
         */
        args: Array<string>;
      };
    }
  | "ApplyCurrentCreatureVariation"
  | "Aquatic"
  | "ArenaRestricted"
  | "AtPeaceWithWildlife"
  | {
      Attack: {
        /**
         * The name of the attack
         */
        name: string;
        /**
         * The body part used for the attack
         */
        body_part: string;
      };
    }
  | {
      AttackTrigger: {
        /**
         * Population trigger
         */
        population: number;
        /**
         * Exported wealth trigger
         */
        exported_wealth: number;
        /**
         * Created wealth trigger
         */
        created_wealth: number;
      };
    }
  | {
      Baby: {
        /**
         * The age at which the creature is considered a child
         */
        age: number;
      };
    }
  | {
      BabyName: {
        /**
         * Singular name for the baby
         */
        singular: string;
        /**
         * Plural name for the baby
         */
        plural: string;
      };
    }
  | {
      BeachFrequency: {
        /**
         * Frequency of beaching
         */
        frequency: number;
      };
    }
  | "Benign"
  | {
      Blood: {
        /**
         * Blood material
         */
        material: string;
        /**
         * Blood token
         */
        state: string;
      };
    }
  | "BloodSucker"
  | {
      Body: {
        /**
         * Body parts arguments
         */
        body_parts: Array<string>;
      };
    }
  | {
      BodyAppearanceModifier: {
        /**
         * Body part to modify
         */
        attribute: string;
        /**
         * Range of values, spread from lowest to median to highest
         */
        values: [number, number, number, number, number, number, number];
      };
    }
  | {
      BodyDetailPlan: {
        /**
         * Body detail plan to load
         */
        body_plan: string;
        /**
         * Body detail plan arguments
         */
        arguments: Array<string>;
      };
    }
  | {
      BodySize: {
        /**
         * Year at which the size is set
         */
        year: number;
        /**
         * Days at which the size is set
         */
        days: number;
        /**
         * Size in cubic centimeters
         */
        size: number;
      };
    }
  | {
      BodyGloss: {
        /**
         * The gloss to use on the body part
         */
        gloss: string;
      };
    }
  | "BoneCarn"
  | {
      BodyPartAddType: {
        /**
         * The body part type to add
         */
        body_part_type: string;
      };
    }
  | {
      BodyPartAppearanceModifier: {
        /**
         * The quality that can appear
         */
        quality: string;
        /**
         * The spread of the quality, from lowest to median to highest
         */
        spread: [number, number, number, number, number, number, number];
      };
    }
  | {
      BodyPartRemoveType: {
        /**
         * The body part type to remove
         */
        body_part_type: string;
      };
    }
  | {
      BuildingDestroyer: {
        /**
         * Whether the creature focuses on doors, hatches, furniture, etc. (`1`) or anything not made with the b + C commands (`2`)
         */
        door_and_furniture_focused: boolean;
      };
    }
  | {
      CanDoInteraction: {
        /**
         * Interaction to allow
         */
        interaction: string;
      };
    }
  | "CanLearn"
  | "CanSpeak"
  | "CannotClimb"
  | "CannotJump"
  | "CannotUndead"
  | "CanOpenDoors"
  | "Carnivore"
  | "CaveAdaptation"
  | {
      ChangeBodySizePercent: {
        /**
         * The percentage to change the body size by
         */
        percent: number;
      };
    }
  | {
      Child: {
        /**
         * The age at which the creature is considered an adult
         */
        age: number;
      };
    }
  | {
      ChildName: {
        /**
         * Singular name for the child
         */
        singular: string;
        /**
         * Plural name for the child
         */
        plural: string;
      };
    }
  | {
      ClutchSize: {
        /**
         * Minimum number of eggs laid in one sitting
         */
        min: number;
        /**
         * Maximum number of eggs laid in one sitting
         */
        max: number;
      };
    }
  | {
      Color: {
        /**
         * The foreground color
         */
        foreground: number;
        /**
         * The background color
         */
        background: number;
        /**
         * The brightness of the color
         */
        brightness: number;
      };
    }
  | "CommonDomestic"
  | "ConvertedSpouse"
  | "CookableLive"
  | "Crazed"
  | {
      CreatureClass: {
        /**
         * The creature class
         */
        class: string;
      };
    }
  | "Crepuscular"
  | "CuriousBeastEater"
  | "CuriousBeastGuzzler"
  | "CuriousBeastItem"
  | {
      CreatureVariationAddTag: {
        /**
         * The tag to add
         */
        tag: string;
      };
    }
  | {
      CreatureVariationRemoveTag: {
        /**
         * The tag to remove
         */
        tag: string;
      };
    }
  | "Demon"
  | {
      Description: {
        /**
         * The description to use
         */
        description: string;
      };
    }
  | "DieWhenVerminBite"
  | {
      Difficulty: {
        /**
         * The difficulty of the creature
         */
        difficulty: number;
      };
    }
  | "Diurnal"
  | "DiveHuntsVermin"
  | {
      ExtraButcherObjectItem: {
        /**
         * The item to add
         */
        item: string;
        /**
         * The material of the item
         */
        material: string;
      };
    }
  | {
      ExtraButcherObjectShape: {
        /**
         * The shape to add
         */
        shape: string;
      };
    }
  | {
      EggMaterial: {
        /**
         * The material to use
         */
        material: string;
        /**
         * The state of the material
         */
        state: string;
      };
    }
  | {
      EggSize: {
        /**
         * The size of the egg
         */
        size: number;
      };
    }
  | "Equips"
  | {
      ExtraButcherObject: {
        /**
         * Details about the extra butcher object
         */
        object_type: string;
        /**
         * Arguments for the extra butcher object
         */
        arguments: Array<string>;
      };
    }
  | {
      Extract: {
        /**
         * The extract material
         */
        material: string;
      };
    }
  | "Extravision"
  | "FeatureAttackGroup"
  | "FeatureBeast"
  | "Female"
  | "FireImmune"
  | "FireImmuneSuper"
  | "FishItem"
  | {
      FixedTemp: {
        /**
         * The temperature of the creature
         */
        temperature: number;
      };
    }
  | "FleeQuick"
  | "Flier"
  | {
      Gait: {
        /**
         * The value of the token
         */
        gait: string;
      };
    }
  | {
      GeneralMaterialForceMultiplier: {
        /**
         * The material to apply the multiplier to
         */
        value_a: number;
        /**
         * The multiplier to apply
         */
        value_b: number;
      };
    }
  | "GetsInfectionsFromRot"
  | "GetsWoundInfections"
  | {
      GlowColor: {
        /**
         * The foreground color
         */
        foreground: number;
        /**
         * The background color
         */
        background: number;
        /**
         * The brightness of the color
         */
        brightness: number;
      };
    }
  | {
      GlowTile: {
        /**
         * The tile to use
         */
        tile: string;
      };
    }
  | {
      Gnawer: {
        /**
         * The verb to use
         */
        verb: string;
      };
    }
  | {
      GobbleVerminClass: {
        /**
         * The vermin class to eat
         */
        vermin_class: string;
      };
    }
  | {
      GobbleVerminCreature: {
        /**
         * The vermin creature to eat
         */
        vermin_creature: string;
        /**
         * The vermin caste to eat
         */
        vermin_caste: string;
      };
    }
  | {
      GrassTrample: {
        /**
         * The trample value
         */
        trample: number;
      };
    }
  | {
      GravitateBodySize: {
        /**
         * The target body size of the creature when it is an adult (fully grown)
         */
        target: number;
      };
    }
  | {
      Grazer: {
        /**
         * The grazer value
         */
        grazer: number;
      };
    }
  | {
      Habit: {
        /**
         * The habit to add
         */
        habit: string;
      };
    }
  | {
      HabitNumber: {
        /**
         * The number of habits to add. A value of `TEST_ALL` will add all habits and will cause number to be 0.
         */
        number: number;
      };
    }
  | "HasNerves"
  | "HasShell"
  | {
      Homeotherm: {
        /**
         * The temperature of the creature, as number or `NONE` which is the default
         */
        temperature: number | null;
      };
    }
  | "HuntsVermin"
  | "Immobile"
  | "ImmobileLand"
  | "Immolate"
  | "Intelligent"
  | {
      InteractionDetail: {
        /**
         * Arbitrary arguments for the interaction
         */
        args: Array<string>;
      };
    }
  | {
      ItemCorpse: {
        /**
         * The item token to use
         */
        item: string;
        /**
         * The material token to use
         */
        material: string;
      };
    }
  | {
      ItemCorpseQuality: {
        /**
         * The quality of the item
         */
        quality: number;
      };
    }
  | {
      Lair: {
        /**
         * The lair type
         */
        lair: string;
        /**
         * The probability of the lair
         */
        probability: number;
      };
    }
  | {
      LairCharacteristic: {
        /**
         * The characteristic to add
         */
        characteristic: string;
      };
    }
  | "LairHunter"
  | {
      LairHunterSpeech: {
        /**
         * The file containing what the creature says
         */
        speech_file: string;
      };
    }
  | "LargePredator"
  | "LaysEggs"
  | {
      LaysUnusualEggs: {
        /**
         * The item to lay
         */
        item: string;
        /**
         * The material of the item
         */
        material: string;
      };
    }
  | {
      Ligaments: {
        /**
         * The material to use
         */
        material: string;
        /**
         * The healing rate
         */
        healing_rate: number;
      };
    }
  | "LightGen"
  | "LikesFighting"
  | "Lisp"
  | {
      LitterSize: {
        /**
         * The minimum number of offspring
         */
        min: number;
        /**
         * The maximum number of offspring
         */
        max: number;
      };
    }
  | "LockPicker"
  | {
      LowLightVision: {
        /**
         * The vision value
         */
        vision: number;
      };
    }
  | "Magical"
  | "MagmaVision"
  | "Male"
  | "MannerismLaugh"
  | "MannerismSmile"
  | "MannerismWalk"
  | "MannerismSit"
  | "MannerismBreath"
  | "MannerismPosture"
  | "MannerismStretch"
  | "MannerismEyelids"
  | { MannerismFingers: { finger: string; fingers: string } }
  | { MannerismNose: { nose: string } }
  | { MannerismEar: { ear: string } }
  | { MannerismHead: { head: string } }
  | { MannerismEyes: { eyes: string } }
  | { MannerismMouth: { mouth: string } }
  | { MannerismHair: { hair: string } }
  | { MannerismKnuckles: { knuckles: string } }
  | { MannerismLips: { lips: string } }
  | { MannerismCheek: { cheek: string } }
  | { MannerismNails: { nails: string } }
  | { MannerismFeet: { feet: string } }
  | { MannerismArms: { arms: string } }
  | { MannerismHands: { hands: string } }
  | { MannerismTongue: { tongue: string } }
  | { MannerismLeg: { leg: string } }
  | "Matutinal"
  | {
      MaxAge: {
        /**
         * The minimum age of the creature
         */
        min: number;
        /**
         * The maximum age of the creature
         */
        max: number;
      };
    }
  | "Meanderer"
  | "Megabeast"
  | {
      MentalAttributeCapPercentage: {
        /**
         * The attribute to modify
         */
        attribute: string;
        /**
         * The percentage to modify the attribute by
         */
        percentage: number;
      };
    }
  | {
      MentalAttributeRange: {
        /**
         * The attribute to modify
         */
        attribute: string;
        /**
         * The ranges from lowest to highest with 7 steps
         */
        ranges: [number, number, number, number, number, number, number];
      };
    }
  | {
      MentalAttributeRate: {
        /**
         * The attribute to modify
         */
        attribute: string;
        /**
         * The cost to improve the attribute
         */
        improvement_cost: number;
        /**
         * The decay rate of the attribute when it is unused
         */
        decay_rate_unused: number;
        /**
         * The decay rate of the attribute when it is rusty
         */
        decay_rate_rusty: number;
        /**
         * The decay rate of the attribute when it is demoted
         */
        decay_rate_demotion: number;
      };
    }
  | {
      Milkable: {
        /**
         * The material of the milk
         */
        material: string;
        /**
         * The frequency the creature can be milked
         */
        frequency: number;
      };
    }
  | "Mischievous"
  | {
      ModValue: {
        /**
         * The value to modify
         */
        value: string;
      };
    }
  | "Mount"
  | "MountExotic"
  | "MultipartFullVision"
  | "MultipleLitterRare"
  | {
      Name: {
        /**
         * The singular name of the caste
         */
        singular: string;
        /**
         * The plural name of the caste
         */
        plural: string;
        /**
         * The adjective form of the caste
         */
        adjective: string;
      };
    }
  | "Natural"
  | {
      NaturalSkill: {
        /**
         * The skill token to add
         */
        skill: string;
        /**
         * The level of the skill
         */
        level: number;
      };
    }
  | "NightCreatureBogeyman"
  | "NightCreatureExperimenter"
  | "NightCreatureHunter"
  | "NightCreatureNightmare"
  | "NoConnectionsForMovement"
  | "NoDizziness"
  | "NoDrink"
  | "NoEat"
  | "NoFall"
  | "NoFevers"
  | "NoGender"
  | "NoPhysicalAttributeGain"
  | "NoPhysicalAttributeRust"
  | "NoSleep"
  | "NoSpring"
  | "NoSummer"
  | "NoThoughtCenterForMovement"
  | "NoUnitTypeColor"
  | "NoVegetationDisturbance"
  | "NoWinter"
  | "NoBones"
  | "NoBreathe"
  | "Nocturnal"
  | "NoEmotion"
  | "NoExert"
  | "NoFear"
  | "NoMeat"
  | "NoNausea"
  | "NoPain"
  | "NoSkin"
  | "NoSkull"
  | "NoSmellyRot"
  | "NoStuckIns"
  | "NoStun"
  | "NotButcherable"
  | "NotLiving"
  | "NoThought"
  | {
      OdorLevel: {
        /**
         * The odor level, defaults to 50
         */
        odor_level: number;
      };
    }
  | {
      OdorString: {
        /**
         * The odor string to use
         */
        odor_string: string;
      };
    }
  | "OpposedToLife"
  | {
      Orientation: {
        /**
         * The caste to set orientation to
         */
        caste: string;
        /**
         * The chance of being disinterested in `caste`
         */
        disinterested_chance: number;
        /**
         * The chance of being casually interested in `caste`
         */
        casual_chance: number;
        /**
         * The chance of being strongly interested in `caste`
         */
        strong_chance: number;
      };
    }
  | "OutsiderControllable"
  | "PackAnimal"
  | "ParalyzeImmune"
  | "PatternFlier"
  | "Pearl"
  | {
      PenetratePower: {
        /**
         * The penetration power
         */
        penetrate_power: number;
      };
    }
  | {
      Personality: {
        /**
         * The trait to modify
         */
        personality_trait: string;
        /**
         * The lowest chance of having the trait
         */
        low: number;
        /**
         * The median chance of having the trait
         */
        median: number;
        /**
         * The highest chance of having the trait
         */
        high: number;
      };
    }
  | "Pet"
  | "PetExotic"
  | {
      PetValue: {
        /**
         * The pet value
         */
        pet_value: number;
      };
    }
  | {
      PetValueDivisor: {
        /**
         * The divisor
         */
        divisor: number;
      };
    }
  | {
      PhysicalAttributeCapPercentage: {
        /**
         * The attribute to modify
         */
        attribute: string;
        /**
         * The percentage to modify the attribute by
         */
        percentage: number;
      };
    }
  | {
      PhysicalAttributeRange: {
        /**
         * The attribute to modify
         */
        attribute: string;
        /**
         * The ranges from lowest to highest with 7 steps
         */
        ranges: [number, number, number, number, number, number, number];
      };
    }
  | {
      PhysicalAttributeRate: {
        /**
         * The attribute to modify
         */
        attribute: string;
        /**
         * The cost to improve the attribute
         */
        improvement_cost: number;
        /**
         * The decay rate of the attribute when it is unused
         */
        decay_rate_unused: number;
        /**
         * The decay rate of the attribute when it is rusty
         */
        decay_rate_rusty: number;
        /**
         * The decay rate of the attribute when it is demoted
         */
        decay_rate_demotion: number;
      };
    }
  | {
      PlusBodyPartGroup: {
        /**
         * The body part selector to use
         */
        body_part_selector: string;
        /**
         * The body part group to add
         */
        body_part_group: string;
      };
    }
  | {
      PopulationRatio: {
        /**
         * The population ratio
         */
        pop_ratio: number;
      };
    }
  | "Power"
  | {
      ProfessionName: {
        /**
         * The profession name / unit type token ID
         */
        profession: string;
        /**
         * The singular name of the profession
         */
        singular: string;
        /**
         * The plural name of the profession
         */
        plural: string;
      };
    }
  | {
      ProneToRage: {
        /**
         * The rage chance
         */
        rage_chance: number;
      };
    }
  | {
      Pus: {
        /**
         * The material of the pus
         */
        material: string;
        /**
         * The material state of the pus
         */
        material_state: string;
      };
    }
  | {
      RelativeSize: {
        /**
         * The body part selector to use
         */
        body_part_selector: string;
        /**
         * The body part to modify
         */
        body_part: string;
        /**
         * The relative size of the body part (by percentage?)
         */
        relative_size: number;
      };
    }
  | {
      Remains: {
        /**
         * The singular name of the remains
         */
        singular: string;
        /**
         * The plural name of the remains
         */
        plural: string;
      };
    }
  | {
      RemainsColor: {
        /**
         * The color of the remains
         */
        remains_color: string;
      };
    }
  | "RemainsOnVerminBiteDeath"
  | "RemainsUndetermined"
  | {
      RetractIntoBodyPart: {
        /**
         * The body part selector to use
         */
        body_part_selector: string;
        /**
         * The body part to retract
         */
        body_part: string;
        /**
         * Description using "you" and "your"
         */
        second_person: string;
        /**
         * Description using "it" and "its"
         */
        third_person: string;
        /**
         * Description using "you" and "your" when the creature is no longer retracted
         */
        second_person_cancel: string;
        /**
         * Description using "it" and "its" when the creature is no longer retracted
         */
        third_person_cancel: string;
      };
    }
  | "ReturnsVerminKillsToOwner"
  | {
      RootAround: {
        /**
         * The body part selector to use
         */
        body_part_selector: string;
        /**
         * The body part to use
         */
        body_part: string;
        /**
         * Verb to use in second person tense ("you")
         */
        second_person_verb: string;
        /**
         * Verb to use in third person tense ("it")
         */
        third_person_verb: string;
      };
    }
  | {
      Secretion: {
        /**
         * The material of the secretion
         */
        material_token: string;
        /**
         * The material state of the secretion
         */
        material_state: string;
        /**
         * The body part selector to use
         */
        body_part_selector: string;
        /**
         * The body part to use
         */
        body_part: string;
        /**
         * The tissue layer to use
         */
        tissue_layer: string;
        /**
         * The trigger to use (CONTINUOUS, EXERTION, EXTREME_EMOTION)
         */
        trigger: string;
      };
    }
  | "SemiMegabeast"
  | {
      SenseCreatureClass: {
        /**
         * The creature class to sense
         */
        creature_class: string;
        /**
         * The tile to use
         */
        tile: string;
        /**
         * The foreground color to use
         */
        foreground: number;
        /**
         * The background color to use
         */
        background: number;
        /**
         * The brightness to use
         */
        brightness: number;
      };
    }
  | {
      SetBodyPartGroup: {
        /**
         * The body part selector to use (BY_TYPE, BY_CATEGORY, BY_TOKEN)
         */
        body_part_selector: string;
        /**
         * The body part to use (via category, type or token)
         */
        body_part: string;
      };
    }
  | {
      SkillLearnRate: {
        /**
         * The skill to modify
         */
        skill: string;
        /**
         * The rate to modify the skill by
         */
        rate: number;
      };
    }
  | {
      SkillLearnRates: {
        /**
         * The rate to modify the skill by
         */
        rate: number;
      };
    }
  | {
      SkillRate: {
        /**
         * The skill to modify
         */
        skill: string;
        /**
         * The improvement rate to modify the skill by
         */
        improvement_rate: number;
        /**
         * The decay rate of the skill when it is unused
         */
        decay_rate_unused: number;
        /**
         * The decay rate of the skill when it is rusty
         */
        decay_rate_rusty: number;
        /**
         * The decay rate of the skill when it is demoted
         */
        decay_rate_demotion: number;
      };
    }
  | {
      SkillRates: {
        /**
         * The improvement rate to modify the skill by
         */
        improvement_rate: number;
        /**
         * The decay rate of the skill when it is unused
         */
        decay_rate_unused: number;
        /**
         * The decay rate of the skill when it is rusty
         */
        decay_rate_rusty: number;
        /**
         * The decay rate of the skill when it is demoted
         */
        decay_rate_demotion: number;
      };
    }
  | {
      SkillRustRate: {
        /**
         * The skill to modify
         */
        skill: string;
        /**
         * The decay rate of the skill when it is unused
         */
        decay_rate_unused: number;
        /**
         * The decay rate of the skill when it is rusty
         */
        decay_rate_rusty: number;
        /**
         * The decay rate of the skill when it is demoted
         */
        decay_rate_demotion: number;
      };
    }
  | {
      SkillRustRates: {
        /**
         * The decay rate of the skill when it is unused
         */
        decay_rate_unused: number;
        /**
         * The decay rate of the skill when it is rusty
         */
        decay_rate_rusty: number;
        /**
         * The decay rate of the skill when it is demoted
         */
        decay_rate_demotion: number;
      };
    }
  | {
      SlainSpeech: {
        /**
         * The speech set to use
         */
        speech_file: string;
      };
    }
  | "SlowLearner"
  | "SmallRemains"
  | {
      SoldierTile: {
        /**
         * The tile to use
         */
        tile: string;
      };
    }
  | {
      SoldierAltTile: {
        /**
         * The tile to use
         */
        tile: string;
      };
    }
  | {
      Sound: {
        /**
         * The sound type to use (ALERT or PEACEFUL_INTERMITTENT)
         */
        sound_type: string;
        /**
         * The range of the sound (in tiles)
         */
        sound_range: number;
        /**
         * A delay before the sound is produced again (in ticks)
         */
        sound_interval: number;
        /**
         * Whether the creature needs to breathe to make the sound
         */
        requires_breathing: boolean;
        /**
         * The first-person description of the sound
         */
        first_person: string;
        /**
         * The third-person description of the sound
         */
        third_person: string;
        /**
         * The out-of-sight description of the sound
         */
        out_of_sight: string;
      };
    }
  | {
      SpecificFood: {
        /**
         * Whether the required item is a plant (true) or creature (false)
         */
        is_plant: boolean;
        /**
         * The identifier of the required plant or creature
         */
        identifier: string;
      };
    }
  | "SpouseConversionTarget"
  | "SpouseConverter"
  | "SpreadEvilSpheresIfRuler"
  | "StanceClimber"
  | "StandardGrazer"
  | "StrangeMoods"
  | "Supernatural"
  | "SwimsInnate"
  | "SwimsLearned"
  | {
      SyndromeDilutionFactor: {
        /**
         * The syndrome to modify
         */
        syndrome: string;
        /**
         * The percentage to modify the syndrome by
         */
        percentage: number;
      };
    }
  | {
      Tendons: {
        /**
         * The material of the tendons
         */
        material: string;
        /**
         * The rate at which the tendons heal (lower is faster)
         */
        healing_rate: number;
      };
    }
  | "ThickWeb"
  | {
      Tile: {
        /**
         * The tile to use
         */
        tile: string;
      };
    }
  | {
      TissueLayer: {
        /**
         * The body part selector to use (BY_TYPE, BY_CATEGORY, BY_TOKEN)
         */
        body_part_selector: string;
        /**
         * The body part to use (via category, type or token)
         */
        body_part: string;
        /**
         * The name of the tissue to use
         */
        tissue: string;
        /**
         * The location to use (FRONT, RIGHT, LEFT, TOP, BOTTOM) or with an additional argument, (AROUND, CLEANS) with a body part and a percentage
         */
        location: string;
      };
    }
  | {
      TissueLayerUnder: {
        /**
         * The body part selector to use (BY_TYPE, BY_CATEGORY, BY_TOKEN)
         */
        body_part_selector: string;
        /**
         * The body part to use (via category, type or token)
         */
        body_part: string;
        /**
         * The name of the tissue to use
         */
        tissue: string;
      };
    }
  | "Titan"
  | {
      TradeCapacity: {
        /**
         * The capacity of the creature
         */
        capacity: number;
      };
    }
  | "Trainable"
  | "TrainableHunting"
  | "TrainableWar"
  | "Trances"
  | "TrapAvoid"
  | "UnderSwim"
  | "UniqueDemon"
  | "Vegetation"
  | {
      VerminBite: {
        /**
         * The chance to inject the material
         */
        chance: number;
        /**
         * The verb to use (e.g. "bitten, stung")
         */
        verb: string;
        /**
         * The material to inject
         */
        material: string;
        /**
         * The material state to inject
         */
        material_state: string;
      };
    }
  | "VerminHateable"
  | "VerminMicro"
  | "VerminNoFish"
  | "VerminNoRoam"
  | "VerminNoTrap"
  | "VerminHunter"
  | "Vespertine"
  | {
      ViewRange: {
        /**
         * The view range of the creature, default is 20
         */
        view_range: number;
      };
    }
  | {
      VisionArc: {
        /**
         * The binocular vision arc of the creature, default is 60
         */
        binocular: number;
        /**
         * The non-binocular vision arc of the creature, default is 120
         */
        non_binocular: number;
      };
    }
  | "WagonPuller"
  | {
      Webber: {
        /**
         * The material of the webs
         */
        material: string;
      };
    }
  | "WebImmune"
  | "Unknown"
  | "NightCreature"
  | "NotFireImmune"
  | "HasBlood"
  | "Grasp"
  | "RaceGait"
  | "CannotBreatheWater"
  | "NaturalAnimal"
  | "CuriousBeast"
  | "CannotBreatheAir";
