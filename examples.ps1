# Test the cli using a single raw
cargo run --example cli -- -r "E:\SteamLibrary\steamapps\common\Dwarf Fortress\data\vanilla\vanilla_materials\objects\inorganic_stone_mineral.txt"  -o examples/out  
# Test the cli using a single raw module
cargo run --example cli -- -m "E:\SteamLibrary\steamapps\common\Dwarf Fortress\data\vanilla\vanilla_creatures" -o examples/out  
# Test the cli using a raw module location
cargo run --example cli -- -l "E:\SteamLibrary\steamapps\common\Dwarf Fortress\data\vanilla" -o examples/out   
# Test the cli using the game directory
cargo run --example cli -- -g "E:\SteamLibrary\steamapps\common\Dwarf Fortress\" -o examples/out   