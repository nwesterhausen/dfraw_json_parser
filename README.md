# df-raw-lookup

A solution to present information about what raws are in your save game, in a searchable format.

I made this because I was playing with [Splint's Vanilla Expanded Mod](http://www.bay12forums.com/smf/index.php?topic=177593.0)
and [Primal](http://www.bay12forums.com/smf/index.php?topic=172869.15) and at the prepare carefully
screen I had no way to figure out what some of the animals were.

Yes there is the [raw explorer]() program but I found that difficult to search with and the information
was displayed in basically the same format as the raw file itself, so it was hard to read.

## Current Functionality

Creates JSON from Dwarf Fortress raw files. This JSON can be used with the small web client to search 
through, be plopped into Algolia and search or, you could simply CTRL+F or grep for what you are looking 
for in the file itself. I find the JSON easier to read than the RAW txt files, and it currently doesn't 
include a lot of items that were not important to me when looking up creatures. I was most concerned with 
the description of the animal, if they laid eggs, if they were milkable, and how big they were. 

- Parses raw files for creatures
- Saves the following creature information to the JSON databse:
    - name
    - description
    - egg size
    - clutch size
    - max age
    - copy tags from (id for what creature the creature is based on) *handled in the web client*

## Rust Program

A rust program which will parse a directory for DF raw files and then output the raws as JSON
to be consumed by the simple web client in www/ for simple search and display functionality.
The rust program is capable of serving the web client itself.

```
USAGE:
    df-raw-lookup.exe [OPTIONS]

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
            NOT IMPLEMENTED Specify the port to run the web server on.

            [default: 4501]

    -r, --raws-dir <RAWS_DIR>
            Specify the directory containing the raw files.

            This usually is a directory named 'raw' in the save or game directory.
            If this is left unspecified, no raws will be parsed when running.

            [default: ]

    -s, --serve
            NOT IMPLEMENTED Use this flag to start a web server for the web search client.

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

#### Download Release

1. download a build from the [releases]() page
2. run `df-raw-lookup --raws-dir [DIRECTORY] --serve`

    It will parse your raws in the directory provided, create a JSON file, and then serve
    a small web client for easy searching.

#### Build Yourself

(Requires rust and cargo installed on your local machine.)

1. clone this repo
2. run `cargo build`
3. edit `run-rust.ps1` to include the correct directory
4. run `run-rust.ps1` and the `out.json` file will be created in the `www/` directory

## Perl script

This all started with a perl script, I've archived that to a 
[gist](https://gist.github.com/nwesterhausen/2fe7775aef7d5f40fd0ababf7d711fa7) since it lost feature 
parity (and object similarity) to this project.