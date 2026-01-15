# Bolt ⚡

> **The command center for your terminal.**

![License](https://img.shields.io/badge/license-MIT-blue.svg) ![Rust](https://img.shields.io/badge/built_with-Rust-orange.svg)

![Bolt Demo](assets/demo.gif)

---

## ⚡ Why Bolt?

**Speed. Aesthetics. Simplicity.**
Turn your messy scripts into a clean, searchable, and interactive dashboard.

## 📚 Documentation

Everything you need to master Bolt:

| Topic | Description |
| :--- | :--- |
| [**Installation**](docs/INSTALLATION.md) | Get up and running in seconds |
| [**Configuration**](docs/CONFIGURATION.md) | The `bolt.yml` cookbook |
| [**Usage**](docs/USAGE.md) | Navigation and controls |
| [**Contributing**](docs/CONTRIBUTING.md) | How to improve Bolt |

## 🚀 Quick Start

1. **Install Bolt**:
   ```bash
   cargo install --git https://github.com/erik/bolt
   ```

2. **Create `bolt.yml`**:

   ```yaml
   tasks:
      run:
      desc: Run the Spring Boot app
      cmds:
         - mvn spring-boot:run
      test:
      desc: Run unit tests
      cmds:
         - mvn test
   ```

3. **Run**:
   ```bash
   bolt
   ```

---
[MIT License](LICENSE) • Inspired by [Taskfile.dev](https://taskfile.dev)
