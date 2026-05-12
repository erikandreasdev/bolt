use crate::config::Task;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use ratatui::widgets::ListState;

pub struct App {
    pub tasks: Vec<Task>,
    pub filtered_indices: Vec<usize>,
    pub search_query: String,
    pub list_state: ListState,
    matcher: SkimMatcherV2,
    pub should_quit: bool,
    pub selected_command: Option<String>,
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
            self.selected_command = Some(self.tasks[task_i].command.clone());
            self.should_quit = true;
        }
    }
}
