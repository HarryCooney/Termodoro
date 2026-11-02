pub struct Menu {
    //0 = Configs, 1 = start timer, 2 = new config
    selected_option: usize,
    num_of_items: usize,
    page_num: usize,
}

impl Menu {
    pub fn new() -> Menu {
        Self {
            selected_option: 1,
            num_of_items: 3,
            page_num: 0
        }
    }
    pub fn choose_next_option(&mut self) {
        self.selected_option = self.selected_option.saturating_add(1) % self.num_of_items;
    }
    pub fn choose_previous_option(&mut self) {
        if self.selected_option == 0 { self.selected_option = self.num_of_items - 1 }
        else {
            self.selected_option = self.selected_option.saturating_sub(1) % self.num_of_items;
        }
    }

    pub fn update_current_page(&mut self) {
        self.page_num = self.selected_option / 3
    }

    pub fn selected(&self) -> usize {
        self.selected_option
    }

    pub fn select(&mut self, index: usize) {
        if index < self.num_of_items {
            self.selected_option = index
        }
    }

    pub fn get_page_num(&self) -> usize { self.page_num }
    pub fn set_page_num(&mut self, page_num: usize) { self.page_num = page_num.clamp(0, 1) }

    pub fn update_state(&mut self, timer_active: bool) {
        let mut items = 3;
        if timer_active {
            items += 1;
        }
        self.num_of_items = items;
    }
    pub fn get_num_of_items(&self) -> usize { self.num_of_items }

}
