### 
# This script is a wrapper around the rust program which will parse your raw files and output a
# JSON file with all the details that get parsed at this time (currently only creatures).
###

# Set $saveFolder to the folder where you want to parse the raw files. 
$saveFolder = "$env:userprofile\Sync\Dwarf Fortress Files\Mods\vanilla-raws\raw";

# Run the program
.\target\debug\df-raw-lookup.exe -r $saveFolder    