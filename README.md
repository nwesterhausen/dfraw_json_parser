# df-raw-lookup

A solution to present information about what raws are in your save game, in a searchable format.

I made this because I was playing with [Splint's Vanilla Expanded Mod](http://www.bay12forums.com/smf/index.php?topic=177593.0)
and [Primal](http://www.bay12forums.com/smf/index.php?topic=172869.15) and at the prepare carefully
screen I had no way to figure out what some of the animals were.

Yes there is the [raw explorer]() program but I found that difficult to search with and the information
was displayed in basically the same format as the raw file itself, so it was hard to read.

## Current Functionality

Both the perl script and the rust program create JSON from Dwarf Fortress raw files. This JSON
can be used with the small web client to search through, be plopped into Algolia and search or, 
you could simply CTRL+F or grep for what you are looking for in the file itself. I find the JSON
easier to read than the RAW txt files, and it currently doesn't include a lot of items that were
not important to me when looking up creatures. I was most concerned with the description of the 
animal, if they laid eggs, if they were milkable, and how big they were. 

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

There is a powershell runner `run-rust.ps1` which you can put the directory you want to search into
and it will create the out.json file.

### How to use

#### Download Release

1. run the `df-raw-lookup` executable of your choice

#### Build Yourself

(Requires rust and cargo installed on your local machine.)

1. clone this repo
2. run `cargo build`
3. edit `run-rust.ps1` to include the correct directory
4. run `run-rust.ps1` and the `out.json` file will be created in the `www/` directory

## Web Client

The web client is just some HTML and javascript (including bootstrap via CDN) to display the raws
and provide a search box. It won't properly grab the out.json file unless the index.html is being 
served by a web server (can be done locally by running one).

Right now there is no http-server built into this program, that is an exercise for the reader (at this time).

## Perl script

A perl script to selectively parse raw files. Could be expanded to pull more data out of raw files 
instead of just creatures.

### How to use

(Requires perl installed on your local machine.)

1. clone this repo
3. edit `run-perl.ps1` to include the correct directory
4. run `run-perl.ps1` and the `out.json` file will be created in the `www/` directory 