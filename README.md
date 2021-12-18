# df-raw-lookup

A perl script to selectively parse raw files. Could be expanded to pull more data out
of raw files instead of just creatures.

I made this because I was playing with [Splint's Vanilla Expanded Mod](http://www.bay12forums.com/smf/index.php?topic=177593.0)
and [Primal](http://www.bay12forums.com/smf/index.php?topic=172869.15) and at the prepare carefully
screen I had no way to figure out what some of the animals were.

This perl script outputs what it takes from the raws as JSON that can be plopped into Algolia and search
(or, you could simply CTRL+F or grep or whatever in the file instead). I was concerned with the description
of the animal and if they laid eggs or were milkable and how big they were. 