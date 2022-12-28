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

There is a [Typescript definition file](./typing.d.ts) for the format of the generated JSON. Here are detailed
descriptions of Rust structs used when performing the serde_json serialization.

#### DF Info File

It parses the info.txt file in each raw module directory it goes through.

| Property                              | Description                                                           | Type     |
| ------------------------------------- | --------------------------------------------------------------------- | -------- |
| identifier                            | id of the raw module                                                  | `String` |
| sourced_directory                     | directory found in, `vanilla`, `mods` or `installed_mods`             | `String` |
| numeric_version                       | version of raw module represented as a number                         | `u32`    |
| displayed_version                     | human-readable display for `numeric_version`                          | `String` |
| earliest_compatible_numeric_version   | earliest "upgrade-compatible" version of this raw module, as a number | `u32`    |
| earliest_compatible_displayed_version | human readable display for `earliest_compatible_numeric_version`      | `String` |
| author                                | author of the raw module                                              | `String` |
| name                                  | human readable name of the module                                     | `String` |
| description                           | description of the module                                             | `String` |
| display_title                         | built-in formatting for `{name} v{displayed_version}`                 | `String` |

#### Creature Token

Creature data and its castes are included. However, currently doesn't apply COPY_TAG_FROM.

| Property           | Description                                   | Type                               |
| ------------------ | --------------------------------------------- | ---------------------------------- |
| identifier         | defined in CREATURE token                     | `String`                           |
| parent_raw         | name of the raw file its located              | `String`                           |
| raw_module         | id of the raws module raw is from             | `String`                           |
| raw_module_version | version of the raws module raw is from        | `String`                           |
| objectId           | unique id for creature                        | `String`                           |
| name               | species name                                  | `String`                           |
| names_map          | names by castes                               | `HashMap<String, Vec<String>>`     |
| descriptions       | descriptions by castes                        | `HashMap<String, String>`          |
| max_age            | max age by castes                             | `HashMap<String, [u16; 2]>`        |
| clutch_size        | clutch size by castes                         | `HashMap<String, [u16; 2]>`        |
| based_on           | defined by copy_tags_from token               | `String`                           |
| biomes             | biomes creature found in                      | `Vec<String>`                      |
| cluster_range      | cluster range (how many appear at once)       | `[u16; 2]`                         |
| underground_depth  | depth found                                   | `[u16; 2]`                         |
| body_size          | body size by castes                           | `HashMap<String, Vec<DFBodySize>>` |
| grown_at           | age when adult by castes                      | `HashMap<String, u32>`             |
| child_at           | age when adolescent by castes                 | `HashMap<String, u32>`             |
| egg_sizes          | egg size by castes                            | `HashMap<String, u32>`             |
| pet_value          | pet value by castes                           | `HashMap<String, u16>`             |
| intelligence       | intelligence by castes                        | `HashMap<String, [bool; 2]>`       |
| flier              | flier by castes                               | `HashMap<String, bool>`            |
| gnawer             | gnawer by castes                              | `HashMap<String, bool>`            |
| trainable          | trainability by castes                        | `HashMap<String, u8>`              |
| active_time        | active time by castes                         | `HashMap<String, u8>`              |
| inactive_season    | NO_SEASON by castes                           | `HashMap<String, u8>`              |
| creature_class     | creature class by castes                      | `HashMap<String, Vec<String>>`     |
| tags               | tags on the creature                          | `Vec<CreatureTag>`                 |
| caste_tags         | tags on each caste                            | `HashMap<String, Vec<CasteTag>>`   |
| difficulty         | difficulty by castes                          | `HashMap<String, u32>`             |
| grass_trample      | grass trample by castes                       | `HashMap<String, u8>`              |
| grazer             | grazer by castes                              | `HashMap<String, u32>`             |
| low_light_vision   | low light vision by castes                    | `HashMap<String, u32>`             |
| pop_ratio          | population ratio by castes                    | `HashMap<String, u16>`             |
| milkable           | milk production by castes                     | `HashMap<String, DFMilkable>`      |
| pref_string        | preference string for creature                | `Vec<String>`                      |
| population_number  | pop num (how many exist per valid world tile) | `[u16; 2]`                         |

## Rust Program

An example rust program which will parse a directory for DF raw files and then output the raws as JSON
to be consumed by the simple web client in www/ for simple search and display functionality.
The rust program is capable of serving the web client itself.

You can run this program after cloning this repository:

```bash
cargo run --example cli -- --help
```

### cli.rs

```sh
Library which parses Dwarf Fortress raw files into JSON

Usage: cli.exe [OPTIONS]

Options:
  -g, --game-dir <GAME_DIR>
          Specify the directory where Dwarf Fortress is installed.

          This directory will likely include the 'gamelog.txt' file, and it should have a 'data' subdirectory.

          [default: ]

  -o, --out-dir <OUT_DIR>
          Specify the directory that the JSON database should be saved into.

          If raw files are parsed, a JSON database (an array of objects) is
          saved to disk in a location specified by this argument. This will
          create an 'raws.json' file in the directory specified by this argument.

          Alongside raws.json will be a modules.json which is a JSON database for the
          raw modules that were found and parsed.

          [default: ./www/]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

### How to use

The [example](examples/cli.rs) is a usable tool to write the parsed raws into JSON, this could then be uploaded
to Algolia search for searching through, or can be easily ingested by a website with javascript.

There is a [Typescript definition file](./typing.d.ts) for the format of the generated JSON.

#### Existing Projects

These are projects which use this library.

- [Overseer's Reference Manual for Dwarf Fortress](https://github.com/nwesterhausen/overseers-manual-df)

## Perl script

This all started with a perl script, I've archived that to a
[gist](https://gist.github.com/nwesterhausen/2fe7775aef7d5f40fd0ababf7d711fa7) since it lost feature
parity (and object similarity) to this project.
