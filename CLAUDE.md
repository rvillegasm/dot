# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`dot` is a CLI dotfiles manager for Unix-like systems, written in Rust. It manages configuration files by moving them into a centralized repository and creating symbolic links at their original locations. Published to crates.io as `dot_rust`.

## Build & Development Commands

```bash
cargo build                  # Build debug
cargo build --release        # Build release
cargo test                   # Run all tests (unit + integration)
cargo test --lib             # Run unit tests only
cargo test --test integration # Run integration tests only
cargo test <test_name>       # Run a single test by name
cargo clippy                 # Lint
cargo fmt                    # Format code
```

## Architecture

The crate exposes both a library (`src/lib.rs`) and a binary (`src/main.rs`). The binary uses the library's public modules.

**Core modules:**

- **`cli`** — Clap-based CLI definition and `run()` entry point. Parses subcommands (`init`, `add`, `remove`, `sync`) and dispatches to the corresponding command struct.
- **`commands/`** — Each command implements the `Command` trait (`execute(self) -> Result<()>`). Commands are self-contained structs: `InitCommand`, `AddCommand`, `RemoveCommand`, `SyncCommand`. Each exposes a static method (e.g., `add_to_manifest`, `sync_manifest`) that separates core logic from I/O for testability.
- **`manifest`** — `Manifest` struct wrapping a `BTreeMap<PathBuf, PathBuf>` serialized as TOML (`dot.toml`). Maps local filenames to their original paths (stored with `~` prefix). Methods come in pairs: `load`/`load_from`, `save`/`save_to`, `get`/`get_with_home` — the `_with_home` variants allow injecting a fake home directory for testing.
- **`path`** — Tilde expansion/collapse utilities and lexical absolute path resolution. Same `_with_home` pattern for testability.
- **`error`** — `thiserror`-based `Error` enum and `Result` type alias used throughout.

**Key pattern:** Functions that depend on the home directory have `_with_home` variants that accept `Option<PathBuf>` to enable deterministic testing without relying on the actual home directory.

## Conventions

- Commits follow **conventional commits** format (`feat:`, `fix:`, `refactor:`, `test:`, `chore:`, etc.).
- Unit tests live alongside their modules in `#[cfg(test)] mod tests`. Integration tests are in `tests/integration.rs` and use `tempfile::TempDir` for isolation.
- The manifest file is always named `dot.toml` (constant `MANIFEST_FILE`).
