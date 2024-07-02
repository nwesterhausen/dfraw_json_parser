Remove-Item '.\jsonlib\bindings\*' -Force
cargo test --all-features
bunx @biomejs/biome format --write "./jsonlib/bindings/DFRawJson.d.ts"
bunx @biomejs/biome format --write "./jsonlib/bindings/DFRawJson-Tauri.d.ts"
