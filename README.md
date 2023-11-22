# dfraw_json_parser

[![supports df 50.xx](https://img.shields.io/badge/Supports%20Dwarf%20Fortress-0.50.xx-%235E3E0D?style=plastic)](https://bay12games.com/dwarves/)
[![Clippy Check](https://github.com/nwesterhausen/dfraw_json_parser/actions/workflows/clippy.yml/badge.svg)](https://github.com/nwesterhausen/dfraw_json_parser/actions/workflows/clippy.yml)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/nwesterhausen/dfraw_json_parser/badge)](https://securityscorecards.dev/viewer/?uri=github.com/nwesterhausen/dfraw_json_parser)

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
- Parses raw files for plants

### Outputs

There are generated typescript type definitions in the [bindings](/bindings/) directory.

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

### How to use

The [example](examples/cli.rs) is a usable tool to write the parsed raws into JSON, this could then be uploaded
to Algolia search for searching through, or can be easily ingested by a website with javascript.

Use the generated typescript types to help you utilize the generated JSON.

#### Existing Projects

These are projects which use this library.

- [Overseer's Reference Manual for Dwarf Fortress](https://github.com/nwesterhausen/overseers-manual-df)

## Perl script

This all started with a perl script, I've archived that to a
[gist](https://gist.github.com/nwesterhausen/2fe7775aef7d5f40fd0ababf7d711fa7) since it lost feature
parity (and object similarity) to this project.
