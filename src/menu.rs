pub struct Menu {
    //0 = Configs, 1 = start timer, 2 = new config
    selected_option: usize
}

impl Menu {
    pub fn new() -> Menu {
        Self {
            selected_option: 1
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
