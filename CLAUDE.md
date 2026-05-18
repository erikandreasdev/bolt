# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build          # debug build
cargo build --release
cargo run            # run with bolt.yml in project root
cargo test           # run all tests
cargo test <name>    # run a single test by name
cargo clippy -- -D warnings   # lint (warnings are errors)
cargo fmt            # format code
cargo check          # fast compile check without producing a binary
```

## Architecture

Bolt is a terminal-based task runner TUI written in Rust with four modules:

- **`src/config.rs`** — Parses `bolt.yml` into `Vec<Task>`. Supports two YAML formats: explicit (`tasks:` wrapper) and root-level shorthand. Multiple `cmds` entries are joined with ` && `. Tasks without `cmds` are silently dropped. Tasks are sorted alphabetically by name.

- **`src/app.rs`** — Holds all runtime state in `App`. Two modes: `Browse` (fuzzy search + list navigation) and `ParamInput` (collecting `{placeholder}` values one by one). When a task is selected, `execute_selected` checks for `{param}` placeholders via `extract_params`; if any exist, transitions to `ParamInput` mode to collect values before building the final command string. `should_quit = true` signals `main` to exit the TUI and run the command.

- **`src/ui.rs`** — Pure rendering; reads `App` state, emits ratatui widgets. Layout: header row (logo + search bar) / task list / footer. When `mode == ParamInput`, renders a centered popup over the list showing the command template, already-filled values, and the current input field.

- **`src/main.rs`** — Entry point and event loop. Handles terminal setup/teardown via `TerminalCleanup` (RAII guard). After the TUI exits with a `selected_command`, runs it via `sh -c` (Unix/macOS) or `cmd /C` (Windows) in the normal terminal, then prompts the user to return to the menu or quit. `ctrlc` handler is set to a no-op so Ctrl+C in the child process doesn't kill the parent.

## Config file resolution order

1. `.local/bolt.yml` / `.local/bolt.yaml`
2. `bolt.yml` / `bolt.yaml` in the project root
3. `--config <path>` CLI flag (overrides all)

## UI color conventions

- Yellow — logo, command preview lines (`$ ...`), current param input value
- Green — task names, task list border/title
- Cyan — search bar border, footer hints, param popup border/title/hints
