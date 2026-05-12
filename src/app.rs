use crate::config::Task;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use ratatui::widgets::ListState;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Browse,
    ParamInput,
}

pub struct App {
    pub tasks: Vec<Task>,
    pub filtered_indices: Vec<usize>,
    pub search_query: String,
    pub list_state: ListState,
    matcher: SkimMatcherV2,
    pub should_quit: bool,
    pub selected_command: Option<String>,
    pub mode: Mode,
    pub param_names: Vec<String>,
    pub param_values: Vec<String>,
    pub param_index: usize,
    pub param_input: String,
    pub command_template: String,
    pub selected_task_name: String,
}

impl App {
    pub fn new(tasks: Vec<Task>) -> Self {
        let count = tasks.len();
        let mut list_state = ListState::default();
        if count > 0 {
            list_state.select(Some(0));
        }
        Self {
            filtered_indices: (0..count).collect(),
            tasks,
            search_query: String::new(),
            list_state,
            matcher: SkimMatcherV2::default(),
            should_quit: false,
            selected_command: None,
            mode: Mode::Browse,
            param_names: Vec::new(),
            param_values: Vec::new(),
            param_index: 0,
            param_input: String::new(),
            command_template: String::new(),
            selected_task_name: String::new(),
        }
    }

    fn selected_index(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }

    pub fn on_key(&mut self, c: char) {
        self.search_query.push(c);
        self.update_filter();
    }

    pub fn on_backspace(&mut self) {
        self.search_query.pop();
        self.update_filter();
    }

    pub fn update_filter(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_indices = (0..self.tasks.len()).collect();
        } else {
            let mut matches: Vec<(i64, usize)> = self
                .tasks
                .iter()
                .enumerate()
                .filter_map(|(i, task)| {
                    self.matcher
                        .fuzzy_match(&task.name, &self.search_query)
                        .map(|score| (score, i))
                })
                .collect();

            matches.sort_by(|a, b| b.0.cmp(&a.0));
            self.filtered_indices = matches.into_iter().map(|(_, i)| i).collect();
        }

        self.list_state.select(if self.filtered_indices.is_empty() {
            None
        } else {
            Some(0)
        });
    }

    pub fn select_next(&mut self) {
        if !self.filtered_indices.is_empty() {
            let i = (self.selected_index() + 1) % self.filtered_indices.len();
            self.list_state.select(Some(i));
        }
    }

    pub fn select_previous(&mut self) {
        if !self.filtered_indices.is_empty() {
            let i = if self.selected_index() > 0 {
                self.selected_index() - 1
            } else {
                self.filtered_indices.len() - 1
            };
            self.list_state.select(Some(i));
        }
    }

    pub fn execute_selected(&mut self) {
        if let Some(&task_i) = self.filtered_indices.get(self.selected_index()) {
            let command = self.tasks[task_i].command.clone();
            let params = extract_params(&command);

            if params.is_empty() {
                self.selected_command = Some(command);
                self.should_quit = true;
            } else {
                self.selected_task_name = self.tasks[task_i].name.clone();
                self.command_template = command;
                self.param_names = params;
                self.param_values = Vec::new();
                self.param_index = 0;
                self.param_input = String::new();
                self.mode = Mode::ParamInput;
            }
        }
    }

    pub fn on_param_key(&mut self, c: char) {
        self.param_input.push(c);
    }

    pub fn on_param_backspace(&mut self) {
        self.param_input.pop();
    }

    pub fn on_param_enter(&mut self) {
        self.param_values.push(std::mem::take(&mut self.param_input));
        self.param_index += 1;

        if self.param_index >= self.param_names.len() {
            let mut command = self.command_template.clone();
            for (name, value) in self.param_names.iter().zip(self.param_values.iter()) {
                command = command.replace(&format!("{{{}}}", name), value);
            }
            self.selected_command = Some(command);
            self.should_quit = true;
        }
    }

    pub fn cancel_param_input(&mut self) {
        self.mode = Mode::Browse;
        self.param_names.clear();
        self.param_values.clear();
        self.param_index = 0;
        self.param_input.clear();
        self.command_template.clear();
        self.selected_task_name.clear();
    }
}

// Extracts unique {placeholder} names from a command string, in order of first appearance.
fn extract_params(command: &str) -> Vec<String> {
    let mut params = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let bytes = command.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'{' {
            let start = i + 1;
            let mut j = start;
            while j < bytes.len() && bytes[j] != b'}' && bytes[j] != b'{' {
                j += 1;
            }
            if j < bytes.len() && bytes[j] == b'}' && j > start {
                let name = &command[start..j];
                if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    if seen.insert(name.to_string()) {
                        params.push(name.to_string());
                    }
                }
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }

    params
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_params_none() {
        assert!(extract_params("cargo build").is_empty());
    }

    #[test]
    fn test_extract_params_single() {
        assert_eq!(extract_params("echo {name}"), vec!["name"]);
    }

    #[test]
    fn test_extract_params_multiple() {
        assert_eq!(
            extract_params("echo {greeting} {name}"),
            vec!["greeting", "name"]
        );
    }

    #[test]
    fn test_extract_params_deduplicates() {
        assert_eq!(extract_params("cp {src} {dst} && echo {src}"), vec!["src", "dst"]);
    }

    #[test]
    fn test_extract_params_ignores_invalid() {
        assert!(extract_params("awk '{print $1}'").is_empty());
    }
}
