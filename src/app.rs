use crate::config::Task;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

pub struct App {
    pub tasks: Vec<Task>,
    pub filtered_tasks: Vec<Task>,
    pub search_query: String,
    pub selected_index: usize,
    matcher: SkimMatcherV2,
    pub should_quit: bool,
    pub selected_command: Option<String>,
}

impl App {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self {
            tasks: tasks.clone(),
            filtered_tasks: tasks, // Initially all tasks shown
            search_query: String::new(),
            selected_index: 0,
            matcher: SkimMatcherV2::default(),
            should_quit: false,
            selected_command: None,
        }
    }

    pub fn on_key(&mut self, c: char) {
        self.search_query.push(c);
        self.update_filtered_tasks();
    }

    pub fn on_backspace(&mut self) {
        self.search_query.pop();
        self.update_filtered_tasks();
    }

    pub fn update_filtered_tasks(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_tasks = self.tasks.clone();
        } else {
            let mut matches: Vec<(i64, Task)> = self
                .tasks
                .iter()
                .filter_map(|task| {
                    self.matcher
                        .fuzzy_match(&task.name, &self.search_query)
                        .map(|score| (score, task.clone()))
                })
                .collect();

            // Sort by score (descending)
            matches.sort_by(|a, b| b.0.cmp(&a.0));
            self.filtered_tasks = matches.into_iter().map(|(_, task)| task).collect();
        }
        
        // Reset selection if out of bounds or just to be safe
        self.selected_index = 0;
    }

    pub fn select_next(&mut self) {
        if !self.filtered_tasks.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.filtered_tasks.len();
        }
    }

    pub fn select_previous(&mut self) {
        if !self.filtered_tasks.is_empty() {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = self.filtered_tasks.len() - 1;
            }
        }
    }

    pub fn execute_selected(&mut self) {
        if let Some(task) = self.filtered_tasks.get(self.selected_index) {
            self.selected_command = Some(task.command.clone());
            self.should_quit = true;
        }
    }
}
