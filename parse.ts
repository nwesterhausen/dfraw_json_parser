/**
 * Raw file syntax is the following:
 * 
 * Line 1: filename
 * Lines 2-n: content of file
 * 
 * The content of the file is both tags and comments (and  blank lines).
 * 
 * Tags will always be within brackets '[' ']' 
 * 
 * There are two special indicators for each object, a type and an id.
 * 
 * [OBJECT:TYPE] where TYPE is one of the accepted types
 * [TYPE:ID] where TYPE matches a preceding [OBJECT:] flag (? unproven), 
 *      and ID is unique identifier for the object
 * 
 * This program is just concerned with presenting a searchable interface to find
 * details about creatures/objects that are in the game (maybe you added some 
 * mods and want to view descriptions or details easily).
 * 
 * Optionally run this to capture all tags, but by default it only grabs the information
 * that the DF Wiki deemed important to sidebar on its main pages:
 * 
 */

import * as fs from 'fs';
import * as path from 'path';
import { CreatureCaste, CreatureRaw, EggDescription } from './raw_definitions/creature';
import * as readline from 'readline';
import { DFRaw } from './raw_definitions/common';

const rawspath = "C:\\Users\\nwesterhausen\\Sync\\Dwarf Fortress Files\\Mods\\vanilla-expanded_primal\\raw"
const rawspath2 = "C:\\Users\\nwesterhausen\\Sync\\Dwarf Fortress Files\\Mods\\vanilla-expanded_primal\\raw\\objects"

const tokenRE = new RegExp(/(\[(?<key>[^\[:]+):?(?<value>[^\]\[]*)])/, 'g');
const tokenRegex = /(\[(?<key>[^\[:]+):?(?<value>[^\]\[]*)])/gm;

type rawObject = {
    objectID: string,
    name_singular: string,
    name_plural: string,
    name_adjective: string,
    description: string,
    id: string,
    type: string,
    parentFilename: string,
    sizes: string[],
    allTokens: string[]
}

function slugify(str: string): string {
    return str.toLowerCase()
        .replace(/ /g, '-')
        .replace(/[^\w-]+/g, '');
}

function parseRawsFile(filepath: string): Promise<DFRaw[]> {
    return new Promise(function (resolve, reject) {
        let results: DFRaw[] = [];

        let filename = "unknown";
        let linenumber = 0;

        // let filedata = fs.createReadStream(filepath);
        // const rl = readline.createInterface({
        //     input: filedata
        // });
        let fdata = fs.readFileSync(filepath, {
            encoding: 'utf-8',
            flag: 'r'
        });

        let currObject: DFRaw | CreatureRaw = new DFRaw(filename, "UNKNOWN", "NULL");
        let currCasteInd = -1;

        // if (linenumber == 0) {
        //     filename = line;
        //     linenumber++;
        //     return;
        // }
        // !!! This should be a yield function?
        let m: RegExpExecArray | null;
        while ((m = tokenRegex.exec(fdata)) !== null) {
            // This is necessary to avoid infinite loops with zero-width matches
            if (m.index === tokenRegex.lastIndex) {
                tokenRegex.lastIndex++;
            }
            let key = "", val = "";
            // The result can be accessed through the `m`-variable.
            m.forEach((match, groupIndex) => {
                // Group 2 is key
                if (groupIndex == 2) {
                    key = match;
                }
                // Group 3 is value
                if (groupIndex == 3) {
                    val = match;
                }
            });
            if (key !== "") {
                console.log(`Parsing ${key}:${val}`);
                if (val === "" && currObject !== undefined) {
                    if (key.includes("ATTACK_FLAG")
                        || key.includes("TL_")) {
                        return;
                    }
                    // catch-all for all attribute tags
                    if (currObject.type === "CREATURE") {
                        (currObject as CreatureRaw).attributeTags.push(key);
                    }
                    // return;
                }

                    console.log(`Switching on ${key}:${val}`);
                    switch (key) {
                        case "CREATURE":
                            // Start of a creature object
                            if (currObject !== undefined) {
                                // if we already had an object, flush it
                                results.push(currObject);
                            }
                            currObject = new CreatureRaw(filename, val);
                            console.info("resetting currcasteind");
                            currCasteInd = -1;
                            break;
                        case "NAME":
                            // The name field has 3 values in the value field
                            let names: string[] = val.split(':');
                            (currObject as CreatureRaw).name = names[0];
                            (currObject as CreatureRaw).nameList = names;
                            break;
                        case "DESCRIPTION":
                            // Description is what it says on the tin
                            (currObject as CreatureRaw).description = val;
                            break;
                        case "CHILD":
                            // The child tag describes when the creature is done being a child
                            (currObject as CreatureRaw).ageGrown = parseInt(val);
                            break;
                        case "MAXAGE":
                            // The maxage tag describes a min and max for maximum age
                            let maxageRange: number[] = val.split(':').map((s): number => parseInt(s));
                            (currObject as CreatureRaw).oldAgeMax = maxageRange[1];
                            (currObject as CreatureRaw).oldAgeMin = maxageRange[0];
                            break;
                        case "PREFSTRING":
                            // Why is the creature admired
                            (currObject as CreatureRaw).prefStrings.push(val);
                            break;
                        case "PETVALUE":
                            (currObject as CreatureRaw).petValue = parseInt(val);
                            break;
                        case "CREATURE_TILE":
                            (currObject as CreatureRaw).tile = val;
                            break;
                        case "CREATURE_CLASS":
                            if ((currObject as CreatureRaw).class === "") {
                                (currObject as CreatureRaw).class = val;
                            }
                            break;
                        case "HOMEOTHERM":
                            (currObject as CreatureRaw).bodyTemperature = parseInt(val);
                            break;
                        case "CLUSTER_SIZE":
                            let clusterRange: number[] = val.split(':').map((s): number => parseInt(s));
                            (currObject as CreatureRaw).clusterMin = clusterRange[0];
                            (currObject as CreatureRaw).clusterMax = clusterRange[1];
                            break;
                        case "POPULATION_NUMBER":
                            let popRange: number[] = val.split(':').map((s): number => parseInt(s));
                            (currObject as CreatureRaw).populationMin = popRange[0];
                            (currObject as CreatureRaw).populationMax = popRange[1];
                            break;
                        case "DIFFICULTY":
                            (currObject as CreatureRaw).difficulty = parseInt(val);
                            break;
                        case "BIOME":
                            (currObject as CreatureRaw).biome.push(val);
                            break;
                        case "NATURAL_SKILL":
                            let skillSplit: string[] = val.split(':');
                            (currObject as CreatureRaw).naturalSkills.push({
                                skill: skillSplit[0],
                                value: parseInt(skillSplit[1])
                            });
                            break;
                        case "BODY_SIZE":
                            let bodySizeSplit: number[] = val.split(':').map((s): number => parseInt(s));
                            (currObject as CreatureRaw).size.push({
                                years: bodySizeSplit[0],
                                days: bodySizeSplit[1],
                                size: bodySizeSplit[2]
                            })
                            break;
                        case "CASTE":
                            (currObject as CreatureRaw).castes.push(new CreatureCaste(val));
                            currCasteInd = (currObject as CreatureRaw).castes.length - 1;
                            console.log(`[CASTE:${val}] currCasteInd updated to ${currCasteInd}`);
                            break;
                        case "SELECT_CASTE":
                            currCasteInd = (currObject as CreatureRaw).castes.findIndex((c) => c.id === val);
                            console.log(`[SELECT_CASTE:${val}] currCasteInd updated to ${currCasteInd}`);
                            break;
                        case "EGG_SIZE":
                            if (currCasteInd === -1) {
                                console.warn(`${(currObject as CreatureRaw).name}: Reached EGG_SIZE outside of Caste definition`);
                                break;
                            }
                            if ((currObject as CreatureRaw).castes[currCasteInd].egg === null) {
                                (currObject as CreatureRaw).castes[currCasteInd].egg = new EggDescription();
                            }
                            (currObject as CreatureRaw).castes[currCasteInd].egg!.eggSize = parseInt(val);
                            break;
                        case "CLUTCH_SIZE":
                            if (currCasteInd < 0) {
                                console.warn("Reached CLUTCH_SIZE outside of Caste definition");
                                break;
                            }
                            if ((currObject as CreatureRaw).castes[currCasteInd].egg === null) {
                                (currObject as CreatureRaw).castes[currCasteInd].egg = new EggDescription();
                            }
                            let clutchRange: number[] = val.split(':').map((s): number => parseInt(s));
                            (currObject as CreatureRaw).castes[currCasteInd].egg!.clutchMin = clutchRange[0];
                            (currObject as CreatureRaw).castes[currCasteInd].egg!.clutchMax = clutchRange[1];
                            break;
                        case "CASTE_NAME":
                            if (currCasteInd < 0) {
                                console.warn("Reached CASTE_NAME outside of Caste definition");
                                break;
                            }
                            (currObject as CreatureRaw).castes[currCasteInd].names = val.split(':');
                            break;
                    }
                
            }
        }
        console.dir(currObject);
        
        if (currObject !== undefined) {
    results.push(currObject);
        }
    // process.exit(1);
    resolve(results);
    });
}
if (fs.existsSync("out.json"))
    fs.rmSync("out.json");

parseRawsFile('creature_domestic.txt').then(res => {
    console.log(JSON.stringify(res, null, 2))
});