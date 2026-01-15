# Bolt Configuration

Bolt runs on **YAML**. It uses a single configuration file named `bolt.yml` (or `bolt.yaml`).

## The Format

You can define tasks in two ways.

### 1. Simple Format (Recommended)
Clean, minimal, and perfect for 99% of projects.

```yaml
# bolt.yml

start:
  desc: Start the development server
  cmds:
    - npm run dev

build:
  desc: Build for production
  cmds:
    - npm run build

test:
  cmds:
    - npm test
```

## Features

- **Sequential Commands**: Multiple items in `cmds` are executed one by one.
- **Shell Power**: Commands are run via `sh -c`, so pipes (`|`) and redirects (`>`) work out of the box.
- **Resilience**: Bolt stays alive even if you `Ctrl+C` a running task.
