use crate::config::Task;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use ratatui::widgets::ListState;
use std::cmp::Reverse;

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

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("tasks", &self.tasks)
            .field("filtered_indices", &self.filtered_indices)
            .field("search_query", &self.search_query)
            .field("should_quit", &self.should_quit)
            .field("mode", &self.mode)
            .field("matcher", &"<SkimMatcherV2>")
            .finish_non_exhaustive()
    }
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

    pub fn reset(&mut self) {
        self.should_quit = false;
        self.selected_command = None;
        self.mode = Mode::Browse;
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

            matches.sort_by_key(|&(score, _)| Reverse(score));
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
            let current = self.selected_index();
            let len = self.filtered_indices.len();
            self.list_state.select(Some((current + len - 1) % len));
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
// Names must be non-empty and contain only alphanumeric characters or underscores.
// Nested or malformed braces (e.g. `{a{b}`, `{}`, `{a-b}`) are skipped gracefully.
fn extract_params(command: &str) -> Vec<String> {
    let mut params: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut rest = command;

    while let Some(open) = rest.find('{') {
        rest = &rest[open + 1..];
        let end = rest.char_indices().find(|&(_, c)| c == '}' || c == '{');
        match end {
            Some((i, '}')) if i > 0 => {
                let name = &rest[..i];
                if name.chars().all(|c| c.is_alphanumeric() || c == '_')
                    && seen.insert(name.to_string())
                {
                    params.push(name.to_string());
                }
                rest = &rest[i + 1..];
            }
            Some((i, _)) => {
                // Nested '{' or empty '}': reposition before that char and re-scan.
                rest = &rest[i..];
            }
            None => break,
        }
    }

    params
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Task;

    fn make_app(names: &[&str]) -> App {
        let tasks = names
            .iter()
            .map(|&n| Task {
                name: n.to_string(),
                command: format!("echo {}", n),
                description: String::new(),
            })
            .collect();
        App::new(tasks)
    }

    // --- extract_params ---

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

    #[test]
    fn test_extract_params_adjacent_braces() {
        assert_eq!(extract_params("cmd {a}{b}"), vec!["a", "b"]);
    }

    #[test]
    fn test_extract_params_empty_braces() {
        assert!(extract_params("cmd {}").is_empty());
    }

    #[test]
    fn test_extract_params_invalid_chars_in_name() {
        assert!(extract_params("cmd {a-b}").is_empty());
    }

    #[test]
    fn test_extract_params_nested_open_brace() {
        // {a{b} — outer brace encounters nested '{', skips it; inner {b} is valid
        assert_eq!(extract_params("cmd {a{b}"), vec!["b"]);
    }

    // --- App::new ---

    #[test]
    fn test_app_new_selects_first() {
        let app = make_app(&["alpha", "beta"]);
        assert_eq!(app.list_state.selected(), Some(0));
        assert_eq!(app.filtered_indices, vec![0, 1]);
        assert_eq!(app.mode, Mode::Browse);
        assert!(!app.should_quit);
        assert!(app.selected_command.is_none());
    }

    #[test]
    fn test_app_new_empty_tasks() {
        let app = make_app(&[]);
        assert_eq!(app.list_state.selected(), None);
        assert!(app.filtered_indices.is_empty());
    }

    // --- filter ---

    #[test]
    fn test_update_filter_narrows_list() {
        let mut app = make_app(&["alpha", "zzz", "zzz2"]);
        app.on_key('a');
        app.on_key('l');
        app.on_key('p');
        let names: Vec<&str> = app
            .filtered_indices
            .iter()
            .map(|&i| app.tasks[i].name.as_str())
            .collect();
        assert!(names.contains(&"alpha"), "alpha must appear in filtered results for 'alp'");
        assert!(!names.contains(&"zzz"), "zzz should not match 'alp'");
    }

    #[test]
    fn test_update_filter_empty_query_shows_all() {
        let mut app = make_app(&["alpha", "beta"]);
        app.on_key('a');
        app.on_backspace();
        assert_eq!(app.filtered_indices.len(), 2);
    }

    // --- navigation ---

    #[test]
    fn test_select_next_wraps() {
        let mut app = make_app(&["a", "b", "c"]);
        app.list_state.select(Some(2));
        app.select_next();
        assert_eq!(app.list_state.selected(), Some(0));
    }

    #[test]
    fn test_select_previous_wraps() {
        let mut app = make_app(&["a", "b", "c"]);
        app.list_state.select(Some(0));
        app.select_previous();
        assert_eq!(app.list_state.selected(), Some(2));
    }

    // --- execute_selected ---

    #[test]
    fn test_execute_selected_no_params() {
        let mut app = make_app(&["build"]);
        app.execute_selected();
        assert!(app.should_quit);
        assert_eq!(app.selected_command, Some("echo build".to_string()));
    }

    #[test]
    fn test_execute_selected_with_params_enters_param_mode() {
        let tasks = vec![Task {
            name: "greet".to_string(),
            command: "echo {name}".to_string(),
            description: String::new(),
        }];
        let mut app = App::new(tasks);
        app.execute_selected();
        assert_eq!(app.mode, Mode::ParamInput);
        assert_eq!(app.param_names, vec!["name"]);
        assert!(!app.should_quit);
    }

    // --- param input ---

    #[test]
    fn test_param_enter_completes_command() {
        let tasks = vec![Task {
            name: "greet".to_string(),
            command: "echo {name}".to_string(),
            description: String::new(),
        }];
        let mut app = App::new(tasks);
        app.execute_selected();
        for c in "world".chars() {
            app.on_param_key(c);
        }
        app.on_param_enter();
        assert!(app.should_quit);
        assert_eq!(app.selected_command, Some("echo world".to_string()));
    }

    #[test]
    fn test_param_enter_multi_param() {
        let tasks = vec![Task {
            name: "copy".to_string(),
            command: "cp {src} {dst}".to_string(),
            description: String::new(),
        }];
        let mut app = App::new(tasks);
        app.execute_selected();
        // Fill first param
        for c in "a.txt".chars() {
            app.on_param_key(c);
        }
        app.on_param_enter();
        assert!(!app.should_quit, "should not quit after first param");
        // Fill second param
        for c in "b.txt".chars() {
            app.on_param_key(c);
        }
        app.on_param_enter();
        assert!(app.should_quit);
        assert_eq!(app.selected_command, Some("cp a.txt b.txt".to_string()));
    }

    #[test]
    fn test_cancel_param_input_returns_to_browse() {
        let tasks = vec![Task {
            name: "greet".to_string(),
            command: "echo {name}".to_string(),
            description: String::new(),
        }];
        let mut app = App::new(tasks);
        app.execute_selected();
        assert_eq!(app.mode, Mode::ParamInput);
        app.cancel_param_input();
        assert_eq!(app.mode, Mode::Browse);
        assert!(app.param_names.is_empty());
    }

    // --- reset ---

    #[test]
    fn test_reset_clears_quit_and_command() {
        let mut app = make_app(&["build"]);
        app.execute_selected();
        assert!(app.should_quit);
        app.reset();
        assert!(!app.should_quit);
        assert!(app.selected_command.is_none());
        assert_eq!(app.mode, Mode::Browse);
    }
}
