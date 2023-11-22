# Parse a legends export XML file
cargo run --example cli -- -l ./examples/legends/region14-00025-01-01-legends_plus.xml -o ./examples/out/parsed-raws.json .

# Parse vanilla raws
cargo run --example cli -- -o ./examples/out/vanilla-raws.json "E:\SteamLibrary\steamapps\common\Dwarf Fortress\"
# Parse vanilla raws with graphics
cargo run --example cli -- -g -o ./examples/out/vanilla-raws-graphics.json "E:\SteamLibrary\steamapps\common\Dwarf Fortress\"
# Parse vanilla and installed raws with graphics
cargo run --example cli -- --vanilla --installed -g -o ./examples/out/vanilla-and-installed-raws-graphics.json "E:\SteamLibrary\steamapps\common\Dwarf Fortress\" "E:\SteamLibrary\steamapps\common\Dwarf Fortress\raw\objects\"
