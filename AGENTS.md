# Repository Guidelines

## Project Structure & Module Organization
This is a small Rust CLI. Source code lives in `src/`. The entry point is `src/main.rs`, which delegates to feature modules in folders: `src/cli/mod.rs` (CLI flags/help), `src/suggest/` (suggest workflow), `src/history/` (zsh history parsing), and `src/registry/` (persisted registry). Each feature folder is split into small files (e.g., `types.rs`, `path.rs`) for readability. There are no assets or tests yet.

## Build, Test, and Development Commands
- `cargo build`: compile the binary.
- `cargo run -- --help`: show CLI help (via `clap`).
- `cargo run -- --suggest`: recommend commands from the last 100 zsh history entries and register them.
- `cargo test`: run tests when they are added (none currently).

## Coding Style & Naming Conventions
Use standard Rust style: 4-space indentation, `snake_case` for functions/modules, `CamelCase` for types, and `SCREAMING_SNAKE_CASE` for constants. Prioritize readability: keep modules small, split methods into smaller units wherever possible, and favor straightforward control flow. Write code comments in detailed Japanese that explain behavior and intent. Run `cargo fmt` before submitting changes (default rustfmt settings).

## Testing Guidelines
No testing framework is set up yet. Unit tests live in a test-only submodule folder with file splits (e.g., `src/history/tests/mod.rs` plus `src/history/tests/parse.rs`, `src/history/tests/recent.rs`) and are wired from the module with `#[cfg(test)] mod tests;`. Integration tests live under `tests/` and are grouped by folders (e.g., `tests/public_api/mod.rs` with a small `tests/public_api_tests.rs` wrapper). Name tests to reflect behavior (e.g., `parses_zsh_extended_history`).

## Commit & Pull Request Guidelines
There is no commit history yet, so no established convention. Use concise, imperative messages (e.g., “Add suggest registry in TOML”). For pull requests, include a short summary, the commands you ran, and any behavioral changes (CLI flags, file locations).

## Configuration & Data Locations
This tool is macOS-only and reads history from `~/.zsh_history`. Persistent data is stored under `~/.gclip/`, currently `~/.gclip/registered.toml` and `~/.gclip/recent.toml`. If you change data formats or locations, update the docs and migration notes.
