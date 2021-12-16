
/**
 * Get the object token with a group capturing the type
 */
export const objectToken = new RegExp(/\[OBJECT:(?<type>[A-Z]+)]/)

/**
 * Generate a new regular expression to match TYPE tags for the current object type. Has a group to capture the ID.
 * @param type the TYPE fromm the objectToken lookup
 * @returns a new regular expression object to match TYPE tags
 */
export function typeToken(type: string): RegExp {
    console.log(`Building regexp for type ${type}`);
    return new RegExp('\\'+'['+type+':'+'('+'['+'^'+'\\'+']'+']'+'+'+')'+']');
}

/**
 * Matches a description token and captures the description text
 */
export const nameToken = new RegExp(/\[NAME:([^\]]+)]/);

/**
 * Matches a description token and captures the description text
 */
 export const descriptionToken = new RegExp(/\[DESCRIPTION:([^\]]+)]/);

 /**
  * Matches egg clutch size, MIN:MAX
  */
 export const clutchSizeToken = new RegExp(/\[CLUTCH_SIZE:([^\]]+)]/);
 /**
  * Matches egg size.
  */
 export const eggSizeToken = new RegExp(/\[EGG_SIZE:([^\]]+)]/);

 /**
  * Matches litter size for non-egg creatures
  */
 export const litterSizeToken = new RegExp(/\[LITTER_SIZE:([^\]]+)]/);

 /**
  * How many years before not a baby
  */
 export const babyToken = new RegExp(/\[BABY:([^\]]+)]/);
 /**
  * Body size matched, YEARS:DAYS:SIZE
  */
 export const bodySizeToken = new RegExp(/\[BODY_SIZE:([^\]]+)]/);

 /**
  * Match every token and groups the token contents.
  */
 export const anyToken = new RegExp(/\[([^\]]+)]/);
