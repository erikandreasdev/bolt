# Bolt ⚡

**Bolt** is a lightning-fast, terminal-based task runner designed for developers who value speed, aesthetics, and simplicity. It turns your confusing scripts into a clean, searchable dashboard.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/built_with-Rust-orange.svg)

![Bolt Demo](assets/demo.gif)

## 🚀 Features

- **Zero Config Overhead**: compatible with `Taskfile` or use the simplified `bolt.yml` format.
- **Fuzzy Search**: Instantly find tasks by name.
- **Resilient Execution**: Long-running processes are handled gracefully. `Ctrl+C` stops the process, not the runner.
- **Visuals**: A sleek, high-contrast TUI (Text User Interface).

## 📖 Documentation

- [**Installation**](docs/INSTALLATION.md) - Get up and running in seconds.
- [**Configuration**](docs/CONFIGURATION.md) - Learn how to define your tasks.
- [**Usage Guide**](docs/USAGE.md) - Tips on navigating the interface.

## ⚡ Quick Start

1. **Install Bolt**:
   ```bash
   cargo install --git https://github.com/yourusername/bolt
   ```

2. **Create a `bolt.yml`**:
   ```yaml
  start:
   desc: Start the development server
   cmds:
      - npm run dev
   ```

3. **Run it**:
   ```bash
   bolt
   ```

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](docs/CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests.

## 📄 License

This project is licensed under the [MIT License](LICENSE).
