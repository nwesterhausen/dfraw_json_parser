import {DFRaw} from './common';

type Milkable = {
	// reference [SELECT CASTE:<string>] ... ... [MILKABLE::<fluid>:<amt>]
	material: string,
	amount: number
}

export class EggDescription {
	// reference [SELECT CASTE:<string>] .. .. [LAYS_EGGS] .. [EGG_SIZE:num] .. [CLUTCH_SIZE:min:max]
	eggSize: number = 0;
	clutchMin: number = -1;
	clutchMax: number = -1;
}

type BodySize = {
	// reference [BODY_SIZE:years:days:size]
	years: number,
	days: number,
	size: number
}

type NaturalSkill = {
	// reference [NATURAL_SKILL:skill:value]
	skill: string,
	value: number
}

export class CreatureCaste {
	id: string;
	litter = "";
	milk: Milkable | null = null;
	egg: EggDescription | null = null;
	names: string[] = [];

	constructor(castename: string) {
		this.id = castename;
	}
}

export class CreatureRaw extends DFRaw {
	name = "";
	description = "";
	nameList: string[] = []; // singular, plural, adjective
	ageGrown = 0; // the [CHILD] tag, age done being a child
	oldAgeMin = -1; // [MAXAGE:min:max]
	oldAgeMax = -1;
	prefStrings: string[] = []; // multiple [PREFSTRING] tags per creature possible
	petValue = -1; // [PETVALUE]
	tile = ""; // [CREATURE_TILE]
	class = ""; // [CREATURE_CLASS]
	bodyTemperature = -1; // [HOMEOTHERM]
	biome: string[] = []; // [BIOME:<string>],
	clusterMin = -1; // [CLUSTER_SIZE:min:max]
	clusterMax = -1;
	populationMin = -1; // [POPULATION_NUMBER:min:max]
	populationMax = -1;
	proneToRagePerc = -1; // [PRONE_TO_RAGE:percentage] 0-1 as a percentage in raws
	size: BodySize[] = [];
	naturalSkills: NaturalSkill[] = [];
	castes: CreatureCaste[] = [];
	difficulty = 1; // [DIFFICULTY:num] default modifier for difficulty is 1
	attributeTags: string[] = []; // Includes any of the following (as themselves)
		// [LARGE_ROAMING], [COMMON_DOMESTIC], [BENIGN], [MEANDERER], [PET_EXOTIC]
		// [NATURAL], [CARNIVORE], [AMPHIBIOUS], [LARGE_PREDATOR], [VERMIN_GROUNDER]
		// [SMALL_REMAINS], [NOT_BUTCHERABLE], [UBIQUITIOUS], [VERMIN_NOTRAP]
		// [VERMIN_SOIL_COLONY], [DIURNAL], [NOCTURNAL], [NO_SLEEP], [HAS_NERVES]
		// [NOBONES], [FLIER], [FEATURE_ATTACK_GROUP], [CAN_LEARN], [CAN_SPEAK]
		// [EQUIPS], [CANOPENDOORS], [ALL_ACTIVE], [SWIMS_INNATE], [CREPUSCULAR]
		// [UNDERSWIM], [PARALYZEIMMUNE], [VERMIN_HATEABLE], [NOPAIN], [NOSTUN], [NOFEAR]
		// [EXTRAVISION], [NO_WINTER], [PET], [TRAINABLE_HUNTING], [TRAINABLE_WAR]
		// [MOUNT], [MOUNT_EXOTIC], [PACK_ANIMAL], [BONECARN] etc etc..
		// Special tags to look for:
		// ![GNAWER:adj]


	constructor(filename: string, id: string) {
		super(filename, "CREATURE", id);
	}
}