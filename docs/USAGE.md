# Bolt Usage Guide

**Bolt** is a lightning-fast terminal task runner designed solely for speed and aesthetic pleasure.

## Launching Bolt

To launch the runner, simply type:

```bash
bolt
```

Bolt looks for a `bolt.yml` in your current directory.

## Interface Navigation

- **Search**: Just start typing to filter tasks.
- **Run**: Press `Enter` to execute.
- **Navigate**: `Up`/`Down` arrows.
- **Esc**: Quit.

## Workflow

1. **Focus**: Bolt takes over your terminal to present a clean list of tasks.
2. **Execute**: It steps aside to let your command run in full interactive mode.
3. **Return**: When done (or interrupted via `Ctrl+C`), you are gently guided back to the menu with a simple keypress.

## Design Philosophy

Bolt is inspired by modern productivity tools. It values:
- **Speed**: Instant startup and fuzzy search.
- **Clarity**: High-contrast, color-coded UI.
- **Simplicity**: No complex dependency graphs, just a list of things you need to do, now.
