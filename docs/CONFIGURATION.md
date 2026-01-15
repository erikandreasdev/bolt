# Bolt Configuration Cookbook

Bolt uses `bolt.yml` (or `bolt.yaml`) to define tasks. This cookbook covers common patterns and advanced usage.

## Basic Structure

The simplest way to define tasks is using the **Simplified Format**:

```yaml
# bolt.yml
start:
  desc: Start the Spring Boot application
  cmds:
    - mvn spring-boot:run
```

## Recipe Book

### 🔗 Multiple Commands (Sequential)
To run commands in order, list them under `cmds`. Bolt joins them with `&&`, so if one fails, the chain stops.

```yaml
remote-reload:
  desc: Rebuild and restart the Docker container
  cmds:
    - mvn clean package -DskipTests
    - docker-compose down
    - docker-compose up -d --build
    - echo "Application reloaded!"
```

### ⚡ Pipes and Redirects
Since Bolt runs commands in your shell (`sh -c`), you can use standard Shell operators like pipes (`|`) and redirects (`>`).

```yaml
deps:
  desc: Analyze dependencies and save to file
  cmds:
    # Save dependency tree to a file
    - mvn dependency:tree > dependencies.txt
    # Find specific conflicts
    - cat dependencies.txt | grep "conflict"
```

### 📜 Multiline Scripts
For complex logic, use YAML's block scalar syntax (`|`) to write multi-line scripts without creating separate files.

```yaml
release-check:
  desc: Check git status before releasing
  cmds:
    - |
      if [ -n "$(git status --porcelain)" ]; then
        echo "❌ Working directory not clean. Commit changes first."
        exit 1
      else
        echo "✅ Ready to release."
      fi
```

### 🌍 Environment Variables
You can set environment variables inline for specific commands.

```yaml
test:
  desc: Run tests with different profiles
  cmds:
    - SPRING_PROFILES_ACTIVE=test mvn test
    - DATABASE_URL=jdbc:postgresql://localhost:5432/test_db mvn test
```

### 🛡️ Ignoring Errors
By default, the sequence stops if a command fails. To continue regardless of failure, append `|| true`.

```yaml
clean-docker:
  desc: Remove containers (ignoring errors if they don't exist)
  cmds:
    - docker stop my-app || true
    - docker rm my-app || true
    - echo "Containers cleaned."
```

### 📂 Running Tasks in Background
Tasks are interactive by default. For long-running processes (repls, servers), just define them normally. `Ctrl+C` will stop the task but keep Bolt running.

```yaml
log:
  desc: Tail docker logs
  cmds:
    - docker logs -f my-app
```
