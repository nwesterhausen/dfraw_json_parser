Remove-Item '.\lib\bindings\*' -Force
cargo test --all-features
bunx @biomejs/biome format --write "./lib/bindings/DFRawJson.d.ts"
bunx @biomejs/biome format --write "./lib/bindings/DFRawJson-Tauri.d.ts"
