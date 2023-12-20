use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum CreatureTag {
    /// Enables the creature to be kept in artificial hives by beekeepers.
    ///
    /// Appears as `ARTIFICIAL_HIVEABLE`
    ArtificialHiveable,
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
    ///
    Good,
    ///
    Savage,
    ///
    Generated,
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
    LargeRoaming,
    ///
    LocalPopsControllable,
    ///
    LocalPopsProduceHeroes,
    ///
    LooseClusters,
    ///
    Mundane,
    ///
    Biome,
    ///
    PrefString,
    ///
    Name,
    ///
    GeneralBabyName,
    ///
    GeneralChildName,
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
    ///
    UndergroundDepth,
    ///
    PopulationNumber,
    ///
    CopyTagsFrom,
    ///
    ApplyCreatureVariation,
    ///
    CreatureTile,
    /// If set, the creature will blink between its [Tile] and its [AltTile].
    ///
    /// Arguments:
    ///
    /// - the 'character' or tile number
    ///
    /// Appears as `ALTTILE:123`
    AltTile,
    ///
    GlowColor,
    ///
    GlowTile,
    #[default]
    Unknown,
    // Tokens found in the legends xml exports but not in the raws
    MatesToBreed,
    TwoGenders,
    AllCastesAlive,
    SmallRace,
    OccursAsEntityRace,
    Equipment,
    /// Defines a caste
    Caste {
        /// The name of the caste
        name: String,
    },
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
