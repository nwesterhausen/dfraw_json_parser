Remove-Item '.\lib\bindings\*' -Force
cargo test --all-features
bunx prettier --write lib/bindings/*.ts
