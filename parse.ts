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
 * CREATURE:
 *      CREATURE_TILE
 *      PREFSTRING (s)
 *      BIOME (s)
 *      Attributes:
 *          FLIER
 *      Tamed Attributes:
 *          PETVALUE
 *          EGG_LAYER
 *          PET_EXOTIC
 *          TRAINABLE, TRAINABLE_WAR, TRAINABLE_HUNTING
 *          Breeding (has both MALE and FEMALE)
 *      Age:
    *      Adult at (CHILD)
    *      Max age (MAXAGE)
 *      Size:
 *          Birth | Mid | Max
 *      Food Products
 *          Eggs (CLUTCH_SIZE)
 *          !! EGG_SIZE
 *      
 *   !? Variations (need to cross reference other results for this)
 */

import * as fs from 'fs';
import * as path from 'path';
import * as re from './regular-expressions';
import * as readline from 'readline';

const rawspath = "C:\\Users\\nwesterhausen\\Sync\\Dwarf Fortress Files\\Mods\\vanilla-expanded_primal\\raw"
const rawspath2 = "C:\\Users\\nwesterhausen\\Sync\\Dwarf Fortress Files\\Mods\\vanilla-expanded_primal\\raw\\objects"

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
    .replace(/ /g,'-')
    .replace(/[^\w-]+/g,'');
}

function parseRawsFile(filepath: string): Promise<rawObject[]> {
    return new Promise(function(resolve, reject) {
        let results: rawObject[] = [];

    let filename = "unknown";
    let linenumber = 0;
    
    let typere: RegExp;
    let currId = "";

    let currName: string[] = [];
    let currDesc = "";
    let currType = "";
    let currClutch = "";
    let currSize: string[] = [];
    let currLitter = "";
    let currTokens: string[] = [];

    let filedata = fs.createReadStream(filepath);
    const rl = readline.createInterface({
        input: filedata
    });

    rl.on("line", function(line) {
        if (linenumber == 0) {
            filename = line;
            linenumber++;
            return;
        }
        if (re.objectToken.test(line)) {
            currType = line.match(re.objectToken)![1];
            typere = re.typeToken(currType);
            console.dir(typere)
            return;
        }
        if (currType !== "CREATURE") {
            return;
        }
        if (typere !== undefined && typere.test(line)) {
            if (currId !== "" && currName.length > 0) {
                let builtObj: rawObject = {
                    objectID: `${filename}_${currType}_${slugify(currId)}`,
                    id: currId,
                    type: currType,
                    name_singular: currName![0],
                    name_plural: currName![1],
                    name_adjective: currName![2],
                    description: currDesc,
                    parentFilename: filename,
                    sizes: currSize,
                    allTokens: currTokens
                };
                results.push(builtObj);
            }
            currId = line.match(typere)![1];
            currName = [];
            currDesc = "";
            currSize = [];
            currTokens = [];
            return;
        }
        if (re.descriptionToken.test(line)) {
            currDesc = line.match(re.descriptionToken)![1];
            return;
        }
        if (re.nameToken.test(line)) {
            currName = line.match(re.nameToken)![1].split(':');
            return;
        }
        if (re.bodySizeToken.test(line)) {
            currSize.push(line.match(re.bodySizeToken)![1]);
            return;
        }
        if (re.anyToken.test(line)) {
            currTokens.push(line.match(re.anyToken)![1]);
        }
        // if (re.litterSizeToken.test(line)) {
        //     currClutch = line.match(re.litterSizeToken)![1];
        //     return;
        // }
        // if (re.clutchSizeToken.test(line)) {
        //     currLitter = line.match(re.clutchSizeToken)![1];
        //     return;
        // }
    });

    rl.on("close", () => {
        resolve(results);
    })
});
}
if (fs.existsSync("out.json"))
    fs.rmSync("out.json");
let rawfiles = fs.readdirSync(rawspath);
let parsepromies: Promise<rawObject[]>[] = [];
for (let fname of rawfiles) {
    if (fs.statSync(path.join(rawspath, fname)).isFile() && path.extname(fname) === ".txt") {
        parsepromies.push(parseRawsFile(path.join(rawspath, fname)))
    }
}
Promise.all(parsepromies).then(data => {
    for (let d of data) {
        if (d.length > 0) {
            fs.writeFileSync("out.json",JSON.stringify(d),{
            flag: "a"
            })
        }
    }
})
rawfiles = fs.readdirSync(rawspath2);
parsepromies = [];
for (let fname of rawfiles) {
    if (fs.statSync(path.join(rawspath2, fname)).isFile() && path.extname(fname) === ".txt") {
        parsepromies.push(parseRawsFile(path.join(rawspath2, fname)))
    }
}
Promise.all(parsepromies).then(data => {
    for (let d of data) {
        if (d.length > 0) {
            fs.writeFileSync("out.json",JSON.stringify(d),{
            flag: "a"
            })
        }
    }
})