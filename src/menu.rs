pub struct Menu {
    options: Vec<MenuOptions>,
    selected_option: usize
}

pub enum MenuOptions {
    StartTimer,
    ViewConfigs,
    NewConfig
}

impl Menu {
    pub fn new() -> Menu {
        Self {
            options: Vec::new(),
            selected_option: 0
        }
    }
    pub fn choose_next_option(&mut self) {
        self.selected_option = self.selected_option.saturating_add(1).clamp(0, 2);
    }
    pub fn choose_previous_option(&mut self) {
        self.selected_option = self.selected_option.saturating_sub(1).clamp(0, 2);
    }

    pub fn selected(&self) -> usize {
        self.selected_option
    }
}
