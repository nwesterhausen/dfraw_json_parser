Remove-Item '.\lib\bindings\*' -Force
cargo test --all-features
pnpx prettier --write lib/bindings/*.ts
