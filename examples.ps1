# Test the example cli on a single raw file
cargo run --example cli -- --parse raw-file -o examples\out\single-inoragnic-stone.json -t "E:\SteamLibrary\steamapps\common\Dwarf Fortress\data\vanilla\vanilla_materials\objects\inorganic_stone_mineral.txt"
cargo run --example cli -- --parse raw-file -o examples\out\single-plant-crops.json -t "E:\SteamLibrary\steamapps\common\Dwarf Fortress\data\vanilla\vanilla_plants\objects\plant_crops.txt"
cargo run --example cli -- --parse raw-file -o examples\out\single-creature-standard.json -t "E:\SteamLibrary\steamapps\common\Dwarf Fortress\data\vanilla\vanilla_creatures\objects\creature_standard.txt"

# Test the example cli on a raw module
cargo run --example cli -- --parse raw-module -o examples\out\vanilla-creatures.json -t "E:\SteamLibrary\steamapps\common\Dwarf Fortress\data\vanilla\vanilla_creatures"
cargo run --example cli -- --parse raw-module -o examples\out\vanilla-plants.json -t "E:\SteamLibrary\steamapps\common\Dwarf Fortress\data\vanilla\vanilla_plants"
cargo run --example cli -- --parse raw-module -o examples\out\vanilla-materials.json -t "E:\SteamLibrary\steamapps\common\Dwarf Fortress\data\vanilla\vanilla_materials"

# Test the example cli on each specific location
cargo run --example cli -- -o examples\out\location-vanilla.json --parse-vanilla -t "E:\SteamLibrary\steamapps\common\Dwarf Fortress\"
cargo run --example cli -- -o examples\out\location-mods.json --parse-mods-downloaded -t "E:\SteamLibrary\steamapps\common\Dwarf Fortress\"
cargo run --example cli -- -o examples\out\location-installed-mods.json --parse-mods-installed -t "E:\SteamLibrary\steamapps\common\Dwarf Fortress\"

# Test the example cli on all locations
cargo run --example cli -- -o examples\out\all.json -t "E:\SteamLibrary\steamapps\common\Dwarf Fortress\"
