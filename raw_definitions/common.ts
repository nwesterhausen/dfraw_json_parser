/**
 * All raw definitions have these properties:
 * 
 *      filename -- the raw filename, defined in the 1st line of the raw file
 *      type -- the parent [OBJECT:<type>], also is the first part of the tag [<type>:<id>]
 *      id -- an identifier for the object, [<type>:<id>]
 *      
 */


export class DFRaw {
    filename: string;
    type: string;
    id: string;

    constructor(filename: string, type: string, id: string) {
        this.filename = filename;
        this.type = type;
        this.id = id;
    }


}