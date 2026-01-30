use ratatui::widgets::ListState;

pub struct App {
    pub should_quit: bool,
    pub pods: Vec<String>,
    pub state: ListState, // Tracks the selection and scroll offset
}

impl Default for App {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0)); // Start with the first item selected

        Self {
            should_quit: false,
            pods: vec!["pod-1".to_string(), "pod-2".to_string(), "pod-3".to_string()],
            state,
        }
    }
}

impl App {
    pub fn next_pod(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.pods.len() - 1 { 0 } else { i + 1 }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous_pod(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 { self.pods.len() - 1 } else { i - 1 }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}