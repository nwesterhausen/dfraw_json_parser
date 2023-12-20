use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum CreatureTag {
    /// If set, the creature will blink between its [Tile] and its [AltTile].
    ///
    /// Arguments:
    ///
    /// - the 'character' or tile number
    ///
    /// Appears as `ALTTILE:123`
    AltTile {
        /// The character or tile number
        character: u32,
    },
    /// Enables the creature to be kept in artificial hives by beekeepers.
    ///
    /// Appears as `ARTIFICIAL_HIVEABLE`
    ArtificialHiveable,
    /// Select a biome the creature may appear in.
    ///
    /// Appears as `BIOME:SomeBiomeId`
    Biome {
        /// Biome identifier
        id: String,
    },
    /// Defines a caste
    Caste {
        /// The name of the caste
        name: String,
    },
    /// Multiplies frequency by a factor of (integer)%.
    ///
    /// Appears as `CHANGE_FREQUENCY_PERC:100`
    ChangeFrequencyPercent {
        /// The percentage to change the frequency by
        percent: u32,
    },
    /// The minimum/maximum numbers of how many creatures per spawned cluster. Vermin fish with this token in
    /// combination with temperate ocean and river biome tokens will perform seasonal migrations.
    ///
    /// Defaults to 1:1 if not specified.
    ///
    /// Appears as `CLUSTER_NUMBER:1:1`
    ClusterNumber {
        /// The minimum number of creatures per spawned cluster
        min: u32,
        /// The maximum number of creatures per spawned cluster
        max: u32,
    },
    /// The color of the creature's tile.
    ///
    /// Arguments:
    ///
    /// * `foreground`: The foreground color
    /// * `background`: The background color
    /// * `brightness`: The brightness of the color
    ///
    /// Appears as `COLOR:0:0:0`
    Color,
    /// Adding this token to a creature prevents it from appearing in generated worlds (unless it's marked as always present for a particular
    /// civilization). For example, adding it to dogs will lead to worlds being generated without dogs in them. Also removes the creature from the
    /// object testing arena's spawn list. If combined with [Fanciful], artistic depictions of the creature will occur regardless. Used by centaurs,
    /// chimeras and griffons in the vanilla game.
    ///
    /// Appears as `DOES_NOT_EXIST`
    DoesNotExist,
    /// Makes the creature appear as a large 3x3 wagon responsible for carrying trade goods, pulled by two [WAGON_PULLER] creatures and driven by a merchant.
    ///
    /// Appears as `EQUIPMENT_WAGON`
    EquipmentWagon,
    /// The creature is considered evil and will only show up in evil biomes. Civilizations with [EntityToken::UseEvilAnimals] can domesticate them
    /// regardless of exotic status. Has no effect on cavern creatures except to restrict taming. A civilization with evil creatures can colonize evil areas.
    ///
    /// Appears as `EVIL`
    Evil,
    /// The creature is a thing of legend and known to all civilizations. Its materials cannot be requested or preferred. The tag also adds some art value modifiers.
    /// Used by a number of creatures. Conflicts with [CasteToken::CommonDomestic].
    Fanciful,
    /// Determines the chances of a creature appearing within its environment, with higher values resulting in more frequent appearance. Also affects the chance of a
    /// creature being brought in a caravan for trading. The game effectively considers all creatures that can possibly appear and uses the FREQUENCY value as a weight
    ///
    /// For example, if there are three creatures with frequencies 10/25/50, the creature with [FREQUENCY:50] will appear approximately 58.8% of the time.
    ///
    /// Defaults to 50 if not specified. Not to be confused with [PopulationRatio].
    ///
    /// Appears as `FREQUENCY:50`
    Frequency {
        /// The frequency of the creature, a number between 0 and 100 (inclusive)
        frequency: u32,
    },
    /// Name of the creatures baby form. Applies to all castes but can be overridden by [CasteToken::BabyName].
    ///
    /// Appears as `GENERAL_BABY_NAME:BabyName:BabyNames`
    GeneralBabyName,
    /// Name of the creatures child form. Applies to all castes but can be overridden by [CasteToken::ChildName].
    ///
    /// Appears as `GENERAL_CHILD_NAME:ChildName:ChildNames`
    GeneralChildName,
    /// Found on procedurally generated creatures like forgotten beasts, titans, demons, angels, and night creatures. Cannot be specified in user-defined raws.
    ///
    /// Appears as `GENERATED`
    Generated,
    /// The color of the creature's glow tile.
    ///
    /// Arguments:
    ///
    /// * `foreground`: The foreground color
    /// * `background`: The background color
    /// * `brightness`: The brightness of the color
    ///
    /// Appears as `GLOWCOLOR:0:0:0`
    GlowColor {
        /// The foreground color
        foreground: u32,
        /// The background color
        background: u32,
        /// The brightness of the color
        brightness: u32,
    },
    /// The creature's tile when it is glowing.
    ///
    /// Arguments:
    ///
    /// * `character`: The character or tile number
    ///
    /// Appears as `GLOWTILE:123`
    GlowTile {
        /// The character or tile number
        character: u32,
    },
    /// Creature is considered good and will only show up in good biomes - unicorns, for example. Civilizations with [EntityToken::UseGoodAnimals] can
    /// domesticate them regardless of exotic status. Has no effect on cavern creatures except to restrict taming. A civilization that has good
    /// creatures can colonize good areas in world-gen.
    ///
    /// Appears as `GOOD`
    Good,
    /// When using tags from an existing creature, inserts new tags at the end of the creature.
    ///
    /// Appears as `GO_TO_END`
    GoToEnd,
    /// When using tags from an existing creature, inserts new tags at the beginning of the creature.
    ///
    /// Appears as `GO_TO_START`
    GoToStart,
    /// When using tags from an existing creature, inserts new tags after the specified tag.
    ///
    /// Arguments:
    ///
    /// * `tag`: The tag to insert after
    ///
    /// Appears as `GO_TO_TAG:TAG`
    GoToTag {
        /// The tag to insert after
        tag: String,
    },
    /// What product is harvested from beekeeping.
    ///
    /// Arguments:
    ///
    /// * `number`: The number of products harvested
    /// * `time`: The time it takes before the next harvest
    /// * `item tokens`: The item tokens that are harvested (some arbitrary list of items)
    ///
    /// Appears as `HARVEST_PRODUCT:1:1:ITEM_TOKENS`
    HarvestProduct {
        /// The number of products harvested
        number: u32,
        /// The time it takes before the next harvest
        time: u32,
        /// The item tokens that are harvested (some arbitrary list of items)
        item_tokens: Vec<String>,
    },
    /// This is the core requisite tag allowing the creature to spawn as a wild animal in the appropriate biomes. Requires specifying a [Biome] in which the creature will spawn.
    /// Does not require specifying a frequency, population number, or cluster number.
    ///
    /// This tag stacks with [CasteToken::Megabeast], [CasteToken::SemiMegabeast], or [CasteToken::NightCreatureHunter]; if used with one of these tags, the creature will spawn
    /// as both a boss and as a wild animal. This tag does not stack with [CasteToken::FeatureBeast] and if both are used the creature will not spawn. This tag is unaffected by
    /// [CasteToken::Demon].
    ///
    /// Appears as `LARGE_ROAMING`
    LargeRoaming,
    /// Allows you to play as a wild animal of this species in adventurer mode. Prevents trading of (tame) instances of this creature in caravans.
    ///
    /// Appears as `LOCAL_POPS_CONTROLLABLE`
    LocalPopsControllable,
    /// Wild animals of this species may occasionally join a civilization. Prevents trading of (tame) instances of this creature in caravans.
    ///
    /// Appears as `LOCAL_POPS_PRODUCE_HEROES`
    LocalPopsProduceHeroes,
    /// The creatures will scatter if they have this tag, or form tight packs if they don't.
    ///
    /// Appears as `LOOSE_CLUSTERS`
    LooseClusters,
    /// Marks if the creature is an actual real-life creature. Only used for age-names at present.
    Mundane,
    /// The generic name for any creature of this type - will be used when distinctions between caste are unimportant. For names for specific castes, use [CASTE_NAME] instead.
    /// If left undefined, the creature will be labeled as "nothing" by the game.
    ///
    /// Appears as `NAME:Name:Names:NameAdj`
    Name {
        /// The name of the creature
        name: String,
        /// The plural name of the creature
        plural_name: String,
        /// The adjective form of the creature's name
        adjective: String,
    },
    /// Adds a material to selected materials. Used immediately after [SELECT_MATERIAL].
    ///
    /// Appears as `PLUS_MATERIAL:Material`
    PlusMaterial {
        /// The material to add
        material: String,
    },
    /// The minimum/maximum numbers of how many of these creatures are present in each world map tile of the appropriate region. Defaults to 1:1 if not specified.
    ///
    /// Appears as `POPULATION_NUMBER:1:1`
    PopulationNumber {
        /// The minimum number of creatures per spawned cluster
        min: u32,
        /// The maximum number of creatures per spawned cluster
        max: u32,
    },
    /// Sets what other creatures prefer about this creature.
    ///
    /// "Urist likes dwarves for their beards."
    ///
    /// Multiple entries will be chosen from at random. Creatures lacking a PREFSTRING token will never appear under another's preferences.
    ///
    /// Appears as `PREFSTRING:PrefString`
    PrefString {
        /// The preference string
        pref_string: String,
    },
    /// The generic name for members of this profession, at the creature level. In order to give members of specific castes different names for professions,
    /// use [CASTE_PROFESSION_NAME] instead.
    ///
    /// Appears as `PROFESSION_NAME:ProfessionId:ProfessionName:ProfessionNames`
    ProfessionName {
        /// The profession id
        id: String,
        /// The name of the profession
        name: String,
        /// The plural name of the profession
        plural_name: String,
    },
    /// Removes a material from the creature.
    ///
    /// Appears as `REMOVE_MATERIAL:Material`
    RemoveMaterial {
        /// The material to remove
        material: String,
    },
    /// Removes a tissue from the creature.
    ///
    /// Appears as `REMOVE_TISSUE:Tissue`
    RemoveTissue {
        /// The tissue to remove
        tissue: String,
    },
    /// The creature will only show up in "savage" biomes. Has no effect on cavern creatures. Cannot be combined with [GOOD] or [EVIL].
    ///
    /// Appears as `SAVAGE`
    Savage,
    /// Adds an additional previously defined caste to the selection. Used after [SELECT_CASTE].
    ///
    /// Appears as `SELECT_ADDITIONAL_CASTE:Caste`
    SelectAdditionalCaste {
        /// The caste to add
        caste: String,
    },
    /// Selects a previously defined caste
    ///
    /// Appears as `SELECT_CASTE:Caste`
    SelectCaste {
        /// The caste to select
        caste: String,
    },
    /// Selects a locally defined material. Can be ALL.
    ///
    /// Appears as `SELECT_MATERIAL:Material`
    SelectMaterial {
        /// The material to select
        material: String,
    },
    /// Selects a tissue for editing.
    ///
    /// Appears as `SELECT_TISSUE:Tissue`
    SelectTissue {
        /// The tissue to select
        tissue: String,
    },
    /// Boasting speeches relating to killing this creature. Examples include text_dwarf.txt and text_elf.txt in data\vanilla\vanilla_creatures\objects.
    ///
    /// Appears as `SLAIN_CASTE:SomeSpeechSet`
    SlainSpeech {
        /// The speech set to use
        slain_speech: String,
    },
    /// Determines how keen a creature's sense of smell is - lower is better. At 10000, a creature cannot smell at all.
    ///
    /// Appears as `SMELL_TRIGGER:10000`
    SmellTrigger {
        /// The smell trigger
        smell_trigger: u32,
    },
    /// If this creature is active in its civilization's military, it will blink between its default tile and this one.
    ///
    /// Appears as `SOLDIER_ALTTILE:SomeTile`
    SoldierAltTile {
        /// The tile to use
        tile: String,
    },
    /// Found on generated angels. This is the historical figure ID of the deity with which the angel is associated. Since HFIDs are not predictable before worldgen,
    /// this isn't terribly usable in mods.
    ///
    /// Appears as `SOURCE_HFID:123`
    SourceHfid {
        /// The historical figure ID
        hfid: u32,
    },
    /// Sets what religious spheres the creature is aligned to, for purposes of being worshipped via the [POWER] token. Also affects the layout of hidden fun stuff,
    /// and the creature's name.
    ///
    /// Appears as `SPHERE:SomeSphere`
    Sphere {
        /// The sphere to use
        sphere: String,
    },
    ///
    Ubiquitous,
    ///
    VerminSoil,
    ///
    VerminSoilColony,
    ///
    VerminRotter,
    ///
    VerminGrounder,
    ///
    VerminFish,
    ///
    VerminEater,
    ///
    UndergroundDepth,
    ///
    CopyTagsFrom,
    ///
    ApplyCreatureVariation,
    ///
    CreatureTile,
    #[default]
    Unknown,
    // Tokens found in the legends xml exports but not in the raws
    MatesToBreed,
    TwoGenders,
    AllCastesAlive,
    SmallRace,
    OccursAsEntityRace,
    Equipment,
}

impl std::fmt::Display for CreatureTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CreatureTag::ArtificialHiveable => write!(f, "ArtificialHiveable"),
            CreatureTag::DoesNotExist => write!(f, "DoesNotExist"),
            CreatureTag::Evil => write!(f, "Evil"),
            CreatureTag::Fanciful => write!(f, "Fanciful"),
            CreatureTag::Good => write!(f, "Good"),
            CreatureTag::Savage => write!(f, "Savage"),
            CreatureTag::Generated => write!(f, "Generated"),
            CreatureTag::Ubiquitous => write!(f, "Ubiquitous"),
            CreatureTag::VerminSoil => write!(f, "VerminSoil"),
            CreatureTag::VerminSoilColony => write!(f, "VerminSoilColony"),
            CreatureTag::VerminRotter => write!(f, "VerminRotter"),
            CreatureTag::VerminGrounder => write!(f, "VerminGrounder"),
            CreatureTag::VerminFish => write!(f, "VerminFish"),
            CreatureTag::VerminEater => write!(f, "VerminEater"),
            CreatureTag::LargeRoaming => write!(f, "LargeRoaming"),
            CreatureTag::LocalPopsControllable => write!(f, "LocalPopsControllable"),
            CreatureTag::LocalPopsProduceHeroes => write!(f, "LocalPopsProduceHeroes"),
            CreatureTag::LooseClusters => write!(f, "LooseClusters"),
            CreatureTag::Mundane => write!(f, "Mundane"),
            CreatureTag::Biome => write!(f, "Biome"),
            CreatureTag::PrefString => write!(f, "PrefString"),
            CreatureTag::Name => write!(f, "Name"),
            CreatureTag::GeneralBabyName => write!(f, "GeneralBabyName"),
            CreatureTag::GeneralChildName => write!(f, "GeneralChildName"),
            CreatureTag::Frequency => write!(f, "Frequency"),
            CreatureTag::UndergroundDepth => write!(f, "UndergroundDepth"),
            CreatureTag::PopulationNumber => write!(f, "PopulationNumber"),
            CreatureTag::CopyTagsFrom => write!(f, "CopyTagsFrom"),
            CreatureTag::ApplyCreatureVariation => write!(f, "ApplyCreatureVariation"),
            CreatureTag::CreatureTile => write!(f, "CreatureTile"),
            CreatureTag::AltTile => write!(f, "AltTile"),
            CreatureTag::Color => write!(f, "Color"),
            CreatureTag::GlowColor => write!(f, "GlowColor"),
            CreatureTag::GlowTile => write!(f, "GlowTile"),
            CreatureTag::ChangeFrequencyPercent => write!(f, "ChangeFrequencyPercent"),
            CreatureTag::ClusterNumber => write!(f, "ClusterNumber"),
            CreatureTag::Unknown => write!(f, "Unknown"),
            CreatureTag::MatesToBreed => write!(f, "MatesToBreed"),
            CreatureTag::TwoGenders => write!(f, "TwoGenders"),
            CreatureTag::AllCastesAlive => write!(f, "AllCastesAlive"),
            CreatureTag::SmallRace => write!(f, "SmallRace"),
            CreatureTag::OccursAsEntityRace => write!(f, "OccursAsEntityRace"),
            CreatureTag::Equipment => write!(f, "Equipment"),
            CreatureTag::EquipmentWagon => write!(f, "EquipmentWagon"),
            CreatureTag::Caste { name } => write!(f, "Caste: {}", name),
        }
    }
}
