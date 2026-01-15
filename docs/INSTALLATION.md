# Installing Bolt

## From Source

Bolt is a Rust CLI tool.

1. Clone commands:
   ```bash
   git clone <repo>
   cd bolt
   ```

2. Install:
   ```bash
   cargo install --path .
   ```

3. Run:
   ```bash
   bolt
   ```

## Development

To hack on Bolt itself:

```bash
# Run with a test config
cargo run
```

Ensure you have a `bolt.yml` in the project root for testing (which is included in the repo).
