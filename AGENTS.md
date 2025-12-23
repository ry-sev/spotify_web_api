# AGENTS.md

## Build/Test/Lint Commands
- Build: `cargo build --all-features`
- Test all: `cargo test --all-features`
- Test single: `cargo test test_name` or `cargo test module::path::tests`
- Lint: `cargo clippy --all --all-features --tests -- -D warnings`
- Format check: `cargo fmt --all -- --check`

## Code Style
- Rust edition 2024, rustfmt style_edition 2024
- Imports: `use crate::api::prelude::*;` for endpoints; group external then internal
- No `unsafe` code (denied), no `.unwrap()` outside tests, no wildcard imports
- Use `parking_lot::Mutex/RwLock` instead of `std::sync` equivalents
- Errors: use `thiserror`, mark enums `#[non_exhaustive]`
- Endpoints: struct per API call implementing `Endpoint` trait, with `From<String>` for simple construction
- Tests: co-located in `#[cfg(test)] mod tests`, use `SingleTestClient`/`PagedTestClient` from `crate::test::client`

## Naming
- Structs/Types: PascalCase (e.g., `GetAlbum`, `SpotifyResult<T>`)
- Functions: snake_case; Constants: SCREAMING_SNAKE_CASE
- Endpoint structs named after action (e.g., `CreatePlaylist`, `GetArtistTopTracks`)
