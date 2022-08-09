/*!
dfraw_json_parser provides a way to turn raw files from Dwarf Fortress into JSON.
It functions using a regular expression to break apart the tokens and then checks the
key in the token against a long list of ones it knows about. Any matches are utilized to
build its own representation of the raw before a final step where it dumps it to JSON.

Currently the only raws that are parsed are Creature raws.

Currently the JSON is returned as a string
*/

use std::path::Path;

mod parser;

//Todo: add an option to recurse into the directory
//Todo: add an option to apply any COPY_TAG_FROM and APPLY_CREATURE_VARIATION tags
//Todo: add an option to specify what kinds of raws to parse
//Todo: add a parameter for JSON_FILE name (currently hard-coded to out.json)

/**
This will return JSON (in string format) of the `rawfile`.

# Examples

```
use dfraw_json_parser::parse_rawfile_to_json_string;

let jsonString = parse_rawfile_to_json_string("creature_next_underground.txt");
```
*/
pub fn parse_rawfile_to_json_string(rawfile: &str) -> String  {
    parser::parse_directory_to_json_string(rawfile)
}

/**
This will parse the file it finds at `rawfile` and save the parsed JSON
to an 'out.json' file in the out_directory.

# Examples

```
use dfraw_json_parser::parse_rawfile_to_json_file;

// This saves an 'out.json' file in the current directory
parse_rawfile_to_json_file("creature_birds.txt", &std::env::current_dir().unwrap());
```
*/
pub fn parse_rawfile_to_json_file(rawfile: &str, out_directory: &Path) {
    parser::parse_directory_to_json_file(rawfile, out_directory);
}

/**
This will return JSON (in string format) of the raw files in raws_directory.

# Examples

```
use dfraw_json_parser::parse_directory_to_json_string;

let jsonString = parse_directory_to_json_string("data/save/region1/raw");
```
*/
pub fn parse_directory_to_json_string(raws_directory: &str) -> String {
    parser::parse_directory_to_json_string(raws_directory)
}

/**
This will parse what raw files it finds at raws_directory and save the parsed JSON
to an 'out.json' file in the out_directory.

# Examples

```
use dfraw_json_parser::parse_directory_to_json_file;

// This saves an 'out.json' file in the current directory
parse_directory_to_json_file("data/save/region1/raw", &std::env::current_dir().unwrap());
```
*/
pub fn parse_directory_to_json_file(raws_directory: &str, out_directory: &Path) {
    parser::parse_directory_to_json_file(raws_directory, out_directory);
}
