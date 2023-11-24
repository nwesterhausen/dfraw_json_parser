#!/bin/sh
# Save df_dir path to variable
df_dir="/Users/nwesterhausen/Library/Containers/com.isaacmarovitz.Whisky/Bottles/CCC4A738-378C-472A-A1A5-F2F4259D1FD9/drive_c/Program Files (x86)/Steam/steamapps/common/Dwarf Fortress"
legends_file="examples/legends/region12-00005-01-01-legends_plus.xml"
raw_file="$df_dir/data/vanilla/vanilla_creatures/objects/creature_standard.txt"

# Vanilla
#cargo run --example cli -- -P --vanilla -o ./examples/out/vanilla.json "$df_dir"
# Vanilla with mods
#cargo run --example cli -- -P --vanilla --installed -o ./examples/out/vanilla-installed.json "$df_dir"
# Everything
#cargo run --example cli -- -P --vanilla --installed --mods -o ./examples/out/vanilla-installed-mods.json "$df_dir"

# Vanilla and a legends export
#cargo run --example cli -- -P --vanilla -o ./examples/out/vanilla-legends.json "$df_dir" -l "$legends_file"
# A specified raws file
cargo run --example cli -- -P -o ./examples/out/rawfile.json "$df_dir" -r "$raw_file"