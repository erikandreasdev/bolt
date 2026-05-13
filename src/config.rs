use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use anyhow::Result;
use serde_yml;

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub command: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Config {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize)]
struct TaskDefinition {
    desc: Option<String>,
    #[serde(default)]
    cmds: Option<Vec<serde_yml::Value>>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::from_str(&content)
    }

    fn from_str(content: &str) -> Result<Self> {
        let value: serde_yml::Value = serde_yml::from_str(content)?;

        // If 'tasks' key exists treat as explicit format, otherwise simplified (root-level map).
        let tasks_map: HashMap<String, TaskDefinition> = if let Some(tasks_val) = value.get("tasks") {
            serde_yml::from_value(tasks_val.clone())?
        } else {
            serde_yml::from_value(value)?
        };

        let mut tasks: Vec<Task> = tasks_map
            .into_iter()
            .filter_map(|(name, def)| {
                let cmds = def.cmds?;
                if cmds.is_empty() {
                    return None;
                }

                let valid_cmds: Vec<String> = cmds
                    .into_iter()
                    .filter_map(|cmd| {
                        if let serde_yml::Value::String(s) = cmd {
                            Some(s)
                        } else {
                            None
                        }
                    })
                    .collect();

                if valid_cmds.is_empty() {
                    return None;
                }

                Some(Task {
                    name,
                    command: valid_cmds.join(" && "),
                    description: def.desc.unwrap_or_default(),
                })
            })
            .collect();

        tasks.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Config { tasks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_simplified_format() {
        let config = Config::from_str(r#"
build:
  desc: Build project
  cmds:
    - cargo build
test:
  desc: Run tests
  cmds:
    - cargo test
"#)
        .unwrap();

        assert_eq!(config.tasks.len(), 2);
        assert_eq!(config.tasks[0].name, "build");
        assert_eq!(config.tasks[0].command, "cargo build");
        assert_eq!(config.tasks[0].description, "Build project");
        assert_eq!(config.tasks[1].name, "test");
    }

    #[test]
    fn test_load_explicit_format() {
        let config = Config::from_str(r#"
tasks:
  deploy:
    desc: Deploy app
    cmds:
      - cargo build --release
      - ./deploy.sh
"#)
        .unwrap();

        assert_eq!(config.tasks.len(), 1);
        assert_eq!(config.tasks[0].name, "deploy");
        assert_eq!(config.tasks[0].command, "cargo build --release && ./deploy.sh");
        assert_eq!(config.tasks[0].description, "Deploy app");
    }

    #[test]
    fn test_tasks_sorted_by_name() {
        let config = Config::from_str(r#"
zebra:
  desc: Last
  cmds:
    - echo z
apple:
  desc: First
  cmds:
    - echo a
"#)
        .unwrap();

        assert_eq!(config.tasks[0].name, "apple");
        assert_eq!(config.tasks[1].name, "zebra");
    }

    #[test]
    fn test_empty_cmds_filtered_out() {
        let config = Config::from_str(r#"
valid:
  desc: Valid task
  cmds:
    - echo hi
empty:
  desc: No commands
  cmds: []
"#)
        .unwrap();

        assert_eq!(config.tasks.len(), 1);
        assert_eq!(config.tasks[0].name, "valid");
    }

    #[test]
    fn test_missing_cmds_filtered_out() {
        let config = Config::from_str(r#"
valid:
  desc: Valid
  cmds:
    - echo hi
no_cmds:
  desc: No cmds key
"#)
        .unwrap();

        assert_eq!(config.tasks.len(), 1);
        assert_eq!(config.tasks[0].name, "valid");
    }
}
