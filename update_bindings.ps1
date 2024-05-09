Remove-Item '.\lib\bindings\*' -Force
cargo test --all-features
bunx @biomejs/biome format --write lib/bindings/*.ts
