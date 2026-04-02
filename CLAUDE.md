# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build          # build
cargo run            # run the TUI
cargo test           # run all tests
cargo test <name>    # run a specific test
cargo clippy         # lint
cargo fmt            # format
```

## Architecture

This is a Rust terminal UI application for browsing/managing NuGet packages, built with:

- **ratatui** — TUI rendering framework (widgets, layout, styling)
- **crossterm** — cross-platform terminal input/output backend
- **tui-input** — text input widget for search fields
- **tokio** — async runtime for non-blocking HTTP calls
- **reqwest** — HTTP client for NuGet API requests
- **serde/serde_json** — JSON deserialization of NuGet API responses
- **anyhow/thiserror** — error handling
- **tracing/tracing-subscriber** — structured logging
- **config + directories** — user config file loading (XDG-aware paths)

The app currently lives entirely in `src/main.rs` and follows the standard ratatui pattern: an `App` struct holds state, a `run` loop calls `terminal.draw` + `handle_events`, and widgets are rendered via `Widget` implementations. The intended expansion is toward a full NuGet package browser with search, version listing, and package details.
