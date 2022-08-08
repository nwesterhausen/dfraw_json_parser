# dfraw_json_parser

A solution to present information about what raws are in your save game, in a searchable format.

I made this because I was playing with [Splint's Vanilla Expanded Mod](http://www.bay12forums.com/smf/index.php?topic=177593.0)
and [Primal](http://www.bay12forums.com/smf/index.php?topic=172869.15) and at the prepare carefully
screen I had no way to figure out what some of the animals were.

Yes there is the [raw explorer](http://www.bay12forums.com/smf/index.php?topic=103360) program but I found 
that difficult to search with and the information was displayed in basically the same format as the raw file 
itself, so it was hard to read.

There is a Typescript [type definition file](./typing.d.ts).

## Current Functionality

Creates JSON from Dwarf Fortress raw files. This JSON can be used with the small web client to search
through, be plopped into Algolia and search or, you could simply CTRL+F or grep for what you are looking
for in the file itself. I find the JSON easier to read than the RAW txt files, and it currently doesn't
include a lot of items that were not important to me when looking up creatures. I was most concerned with
the description of the animal, if they laid eggs, if they were milkable, and how big they were.

- Parses raw files for creatures

### Outputs

| Property          | Description                                   | Type                             |
| ----------------- | --------------------------------------------- | -------------------------------- |
| identifier        | defined in CREATURE token                     | String                           |
| parent_raw        | name of the raw file its located              | String                           |
| objectId          | unique id for creature                        | String                           |
| name              | species name                                  | String                           |
| names_map         | names by castes                               | HashMap<String, Vec<String>>     |
| descriptions      | descriptions by castes                        | HashMap<String, String>          |
| max_age           | max age by castes                             | HashMap<String, [u16; 2]>        |
| clutch_size       | clutch size by castes                         | HashMap<String, [u16; 2]>        |
| based_on          | defined by copy_tags_from token               | String                           |
| biomes            | biomes creature found in                      | Vec<String>                      |
| cluster_range     | clust range (how many appear at once)         | [u16; 2]                         |
| underground_depth | depth found                                   | [u16; 2]                         |
| body_size         | body size by castes                           | HashMap<String, Vec<DFBodySize>> |
| grown_at          | age when adult by castes                      | HashMap<String, u32>             |
| child_at          | age when adolescent by castes                 | HashMap<String, u32>             |
| egg_sizes         | egg size by castes                            | HashMap<String, u32>             |
| pet_value         | pet value by castes                           | HashMap<String, u16>             |
| intelligence      | intelligence by castes                        | HashMap<String, [bool; 2]>       |
| flier             | flier by castes                               | HashMap<String, bool>            |
| gnawer            | gnawer by castes                              | HashMap<String, bool>            |
| trainable         | trainability by castes                        | HashMap<String, u8>              |
| active_time       | active time by castes                         | HashMap<String, u8>              |
| inactive_season   | NO_SEASON by castes                           | HashMap<String, u8>              |
| creature_class    | creature class by castes                      | HashMap<String, Vec<String>>     |
| tags              | tags on the creature                          | Vec<CreatureTag>                 |
| caste_tags        | tags on each caste                            | HashMap<String, Vec<CasteTag>>   |
| difficulty        | difficulty by castes                          | HashMap<String, u32>             |
| grass_trample     | grass trample by castes                       | HashMap<String, u8>              |
| grazer            | grazer by castes                              | HashMap<String, u32>             |
| low_light_vision  | low light vision by castes                    | HashMap<String, u32>             |
| pop_ratio         | population ratio by castes                    | HashMap<String, u16>             |
| milkable          | milk production by castes                     | HashMap<String, DFMilkable>      |
| pref_string       | preference string for creature                | Vec<String>                      |
| population_number | pop num (how many exist per valid world tile) | [u16; 2]                         |

## Rust Program

An example rust program which will parse a directory for DF raw files and then output the raws as JSON
to be consumed by the simple web client in www/ for simple search and display functionality.
The rust program is capable of serving the web client itself.

You can run this program after cloning this repository:

```bash
cargo run --example cli -- --help
```

### cli.rs

```
USAGE:
    cli.exe [OPTIONS]

OPTIONS:
    -h, --help
            Print help information

    -o, --out-dir <OUT_DIR>
            Specify the directory that the JSON database should be saved into.

            If raw files are parsed, a JSON database (an array of objects) is
            saved to disk in a location specified by this argument. This will
            create an 'out.json' file in the directory specified by this argument.

            [default: ./www/]

    -p, --port <PORT>
            Specify the port to run the web server on.

            [default: 4501]

    -r, --raws-dir <RAWS_DIR>
            Specify the directory containing the raw files.

            This usually is a directory named 'raw' in the save or game directory.
            If this is left unspecified, no raws will be parsed when running.

            [default: ]

    -s, --serve
            Include this flag to start a web server for the web search client.

            Included in the repository is a 'www' folder with a small web client
            that will fetch the JSON database created by this program (out.json)
            and present it in a searchable manner to the user.

            If you include this flag, after any parsing is done, a tiny HTTP server
            will start server files from the directory specified by 'out-dir' which
            defaults to ./www

    -V, --version
            Print version information
```

### How to use

The example is a usable tool to search raws.

See the project [Overseer's Reference Manual](https://github.com/nwesterhausen/overseers-manual-df) 
for a project that uses this library.

There is a [Typescript definition file](./typing.d.ts) for the format of the generated JSON.

## Perl script

This all started with a perl script, I've archived that to a
[gist](https://gist.github.com/nwesterhausen/2fe7775aef7d5f40fd0ababf7d711fa7) since it lost feature
parity (and object similarity) to this project.
