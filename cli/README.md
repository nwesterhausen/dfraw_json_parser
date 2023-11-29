# dfraw_json_parser CLI

```sh

Usage: dfraw-json-parser [OPTIONS] <dwarf-fortress-path>

Default behavior:
    - Parse the vanilla raws
    - Parse all object types
    - Print a summary of the parsed raws
    - Save the parsed raws to 'parsed-raws.json'
    - Log at the 'info' level

The following options are supported:
    -c, --creature      Parse creature raws
    -p, --plant         Parse plant raws
    -e, --entity        Parse entity raws
    -i, --inorganic     Parse inorganic raws
    -g, --graphics      Parse graphics raws

    -s, --summary       Print a summary of the parsed raws

    -P, --pretty        Pretty-print the parsed raws
        This is only used when saving the parsed raws to a file.

    -M, --metadata      Attach metadata to the parsed raws
        This includes the raws' file paths and other information about the
        raws' source.

    -o, --output PATH   Set the output path for the parsed raws
        Default value: 'parsed-raws.json'

    -r, --raw PATH      Parse a raw file
        This can be included multiple times to parse multiple raw files
        directly.

    -l, --legends PATH  Parse a legends export
        This can be included multiple times to parse multiple legends
        exports. These should be 'legends-plus' exports from DFHack.

    -m, --module PATH   Parse a raw module
        This can be included multiple times to parse multiple raw modules
        directly. This could be to specify a single raw module to parse, or
        to specify a raw module to parse in addition to the raw modules
        specified by the --vanilla, --mods, and --installed flags.

    -v, --verbose       Increase the verbosity of the output
        Default log level: 'info'

        This supports up to two levels of verbosity. The first level is
        'debug', and the second level is 'trace'. (e.g. '-vv' or '--verbose --verbose')

    -q, --quiet         Decrease the verbosity of the output
        Default log level: 'info'

        This supports up to two levels of verbosity. The first level is
        'warn', and the second level is 'error'. (e.g. '-qq' or '--quiet --quiet')

    --vanilla           Parse the vanilla raws
    --mods              Parse the raws from all mods
    --installed         Parse the raws from installed mods

    -h, --help              Print this help message
    -V, --version           Print the version number

```

## Example Usage

Install:

```sh
cargo install dfraw_json_parser-cli
```

Use:

```sh
❯ dfraw_json_parser-cli --vanilla "E:\SteamLibrary\steamapps\common\Dwarf Fortress"
2023-11-29T17:20:31.708910Z  INFO dfraw_json_parser: Dispatching parse for vanilla raws
2023-11-29T17:20:31.715698Z  INFO dfraw_json_parser: Found 23 raw modules in Vanilla
2023-11-29T17:20:31.721361Z  INFO dfraw_json_parser: Parsing objects for vanilla_bodies v50.01
2023-11-29T17:20:31.746482Z  INFO dfraw_json_parser: Parsing objects for vanilla_buildings v50.01
2023-11-29T17:20:31.756534Z  INFO dfraw_json_parser: Parsing objects for vanilla_creatures v50.08
2023-11-29T17:20:32.013094Z  INFO dfraw_json_parser: Parsing objects for vanilla_descriptors v50.01
2023-11-29T17:20:32.043585Z  INFO dfraw_json_parser: Parsing objects for vanilla_entities v50.01
2023-11-29T17:20:32.058126Z  INFO dfraw_json_parser: Parsing objects for vanilla_interactions v50.01
2023-11-29T17:20:32.068453Z  INFO dfraw_json_parser: Parsing objects for vanilla_items v50.01
2023-11-29T17:20:32.124778Z  INFO dfraw_json_parser: Parsing objects for vanilla_languages v50.01
2023-11-29T17:20:32.170190Z  INFO dfraw_json_parser: Parsing objects for vanilla_materials v50.01
2023-11-29T17:20:32.214593Z  INFO dfraw_json_parser: Parsing objects for vanilla_music v50.04
2023-11-29T17:20:32.230923Z  INFO dfraw_json_parser: Parsing objects for vanilla_plants v50.09
2023-11-29T17:20:32.277343Z  INFO dfraw_json_parser: Parsing objects for vanilla_reactions v50.01
2023-11-29T17:20:32.294010Z  INFO dfraw_json_parser: Parsing objects for vanilla_text v50.01
2023-11-29T17:20:32.523187Z  INFO dfraw_json_parser::parser::helpers::absorb_select_creature: looking at 43 SELECT_CREATURE of 1306 raws
2023-11-29T17:20:32.529830Z  INFO dfraw_json_parser::parser::helpers::apply_copy_from: updating 364 of 1306 raws from 185 creatures
2023-11-29T17:20:32.540825Z  INFO dfraw_json_parser::parser::helpers::apply_copy_from: copied tags to 364 creatures
2023-11-29T17:20:32.544346Z  INFO dfraw_json_parser::parser::helpers::apply_copy_from: purged 364 raws
2023-11-29T17:20:32.544540Z  INFO dfraw_json_parser::parser::helpers::apply_copy_from: finished with 1306 raws (net 0 lost)
2023-11-29T17:20:32.551432Z  INFO dfraw_json_parser: Found 23 raw modules in "vanilla"
2023-11-29T17:20:32.555398Z  INFO dfraw_json_parser_cli: Opened parsed-raws.json for writing
2023-11-29T17:20:33.627940Z  INFO dfraw_json_parser_cli: Wrote 1306 parsed raws and 21 parsed 'info.txt' files to parsed-raws.json
```
