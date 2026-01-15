# Bolt Configuration Cookbook

Bolt uses `bolt.yml` (or `bolt.yaml`) to define tasks. This cookbook covers common patterns and advanced usage.

## Basic Structure

The simplest way to define tasks is using the **Simplified Format**:

```yaml
# bolt.yml
start:
  desc: Start the development server
  cmds:
    - npm run dev
```

## Recipe Book

### 🔗 Multiple Commands (Sequential)
To run commands in order, list them under `cmds`. Bolt joins them with `&&`, so if one fails, the chain stops.

```yaml
deploy:
  desc: Build and deploy the application
  cmds:
    - echo "Building..."
    - npm run build
    - echo "Deploying..."
    - ./scripts/deploy.sh
```

### ⚡ Pipes and Redirects
Since Bolt runs commands in your shell (`sh -c`), you can use standard Shell operators like pipes (`|`) and redirects (`>`).

```yaml
audit:
  desc: Check for security vulnerabilities and save report
  cmds:
    # Pipe output to a file
    - npm audit --json > audit_report.json
    # Pipe output to another command
    - cat audit_report.json | jq .metadata
```

### 📜 Multiline Scripts
For complex logic, use YAML's block scalar syntax (`|`) to write multi-line scripts without creating separate files.

```yaml
setup:
  desc: Complex setup script
  cmds:
    - |
      echo "Setting up environment..."
      if [ ! -d "node_modules" ]; then
        npm install
      else
        echo "Dependencies already installed."
      fi
      echo "Setup complete!"
```

### 🌍 Environment Variables
You can set environment variables inline for specific commands.

```yaml
test:
  desc: Run tests with custom environment
  cmds:
    - NODE_ENV=test npm test
    - DATABASE_URL=postgres://localhost:5432/test_db cargo test
```

### 🛡️ Ignoring Errors
By default, the sequence stops if a command fails. To continue regardless of failure, append `|| true`.

```yaml
clean:
  desc: Clean up build artifacts (ignoring errors if files don't exist)
  cmds:
    - rm -rf dist || true
    - rm -rf coverage || true
    - echo "Clean complete"
```

### 📂 Running Tasks in Background
Tasks are interactive by default. For long-running processes (repls, servers), just define them normally. `Ctrl+C` will stop the task but keep Bolt running.

```yaml
serve:
  desc: Run the backend server
  cmds:
    - cargo run
```
