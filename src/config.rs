use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use anyhow::Result;

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

// Explicit or Simplified format structures


#[derive(Debug, Deserialize)]
struct TaskDefinition {
    desc: Option<String>,
    #[serde(default)]
    cmds: Option<Vec<serde_yaml::Value>>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        
        // Parse as generic YAML Value first to determine structure
        let value: serde_yaml::Value = serde_yaml::from_str(&content)?;

        // Logic: If 'tasks' key exists, treat as Explicit format (tasks under 'tasks' key). 
        // Otherwise, treat the whole file as a map of Tasks (Simplified format).
        let tasks_map: HashMap<String, TaskDefinition> = if let Some(tasks_val) = value.get("tasks") {
            serde_yaml::from_value(tasks_val.clone())?
        } else {
            // Attempt to parse the root as a map of tasks, ignoring keys that don't look like tasks.
            serde_yaml::from_value(value)?
        };

        let mut tasks: Vec<Task> = tasks_map
            .into_iter()
            .filter_map(|(name, def)| {
                // If it's a version key or similar non-task key that slipped through in simplified mode,
                // we might want to skip it. But generic parsing is lenient.
                // For now, valid tasks must have commands.
                
                let cmds = def.cmds?;
                if cmds.is_empty() {
                    return None;
                }
                
                // Filter only string commands, ignore internal task calls for now
                let valid_cmds: Vec<String> = cmds.into_iter()
                    .filter_map(|cmd| {
                        if let serde_yaml::Value::String(s) = cmd {
                            Some(s)
                        } else {
                            None
                        }
                    })
                    .collect();

                if valid_cmds.is_empty() {
                    return None;
                }

                // Join multiple commands with && for simple shell execution
                let command = valid_cmds.join(" && ");
                
                Some(Task {
                    name,
                    command,
                    description: def.desc.unwrap_or_default(),
                })
            })
            .collect();

        // Sort by name for consistent display
        tasks.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Config { tasks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_config() {
        let yaml = r#"
        build:
          desc: Build project
          cmds:
            - cargo build
        "#;
        
        let value: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
        // We simulate the logic inside Config::load manually here or just test serde structure
        let tasks_map: HashMap<String, TaskDefinition> = serde_yaml::from_value(value).unwrap();
        
        assert!(tasks_map.contains_key("build"));
    }
}
