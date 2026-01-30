pub struct App {
    pub should_quit: bool,
    pub pods: Vec<String>, // Start with just names for now
    pub selected_index: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_quit: false,
            pods: vec![],
            selected_index: 0,
        }
    }
}