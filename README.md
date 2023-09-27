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

Use the generated typescript types to help you utilize the generated JSON.

#### Existing Projects

These are projects which use this library.

- [Overseer's Reference Manual for Dwarf Fortress](https://github.com/nwesterhausen/overseers-manual-df)

## Perl script

This all started with a perl script, I've archived that to a
[gist](https://gist.github.com/nwesterhausen/2fe7775aef7d5f40fd0ababf7d711fa7) since it lost feature
parity (and object similarity) to this project.
