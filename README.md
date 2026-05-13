# Bolt ⚡

> **The command center for your terminal.**

![License](https://img.shields.io/badge/license-MIT-blue.svg) ![Rust](https://img.shields.io/badge/built_with-Rust-orange.svg)

![Bolt Demo](assets/demo.gif)

---

## ⚡ Why Bolt?

**Speed. Aesthetics. Simplicity.**
Turn your messy scripts into a clean, searchable, and interactive dashboard.

## 📚 Documentation

| Topic | Description |
| :--- | :--- |
| [**Installation**](docs/INSTALLATION.md) | Get up and running in seconds |
| [**Configuration**](docs/CONFIGURATION.md) | The `bolt.yml` cookbook |
| [**Usage**](docs/USAGE.md) | Navigation and controls |
| [**Contributing**](docs/CONTRIBUTING.md) | How to improve Bolt |

## 🚀 Quick Start

1. **Install Bolt**:
   ```bash
   cargo install --git https://github.com/erikandreasdev/bolt
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
     deploy:
       desc: Deploy to environment
       cmds:
         - ./deploy.sh {environment}
   ```

3. **Run**:
   ```bash
   bolt
   ```

---

## 🎮 Controls

| Key | Action |
| :--- | :--- |
| Type | Fuzzy search tasks |
| `↑` / `↓` | Navigate task list |
| `Enter` | Execute selected task |
| `Ctrl+C` | Interrupt running task |
| `Esc` / `q` | Quit |

---

## ⚙️ Configuration

Bolt reads `bolt.yml` (or `bolt.yaml`) from the current directory.

```yaml
tasks:
  task-name:
    desc: Optional description shown in the UI
    cmds:
      - command1
      - command2   # multiple commands are joined with &&
```

**Root-level shorthand** (without the `tasks:` wrapper) is also supported.

### Parameters

Use `{param_name}` placeholders in commands to prompt for input at runtime:

```yaml
tasks:
  commit:
    desc: Commit with a message
    cmds:
      - git add . && git commit -m "{message}"
  copy:
    desc: Copy file
    cmds:
      - cp {src} {dst}
```

When a task has parameters, Bolt shows a popup for each value before executing. Parameters are filled in order and substituted into the command.

---

[MIT License](LICENSE) • Inspired by [Taskfile.dev](https://taskfile.dev)
