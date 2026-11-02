use ratatui::widgets::{Widget, StatefulWidget, Block,
    Paragraph, List, ListItem, Wrap, Padding};
use ratatui::layout::{Layout, Rect, Constraint, Direction, Alignment, Flex};
use ratatui::style::{Color, Stylize};
use std::rc::Rc;
use ratatui::buffer::Buffer;
use ratatui::Frame;
use crate::app::{App, AppState};
use crate::icons::MenuIcon;
use crate::pomodoro::PomodoroTimerState;


impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn render_menu(&self, area: Rect, buf: &mut Buffer) {

        if self.menu.get_num_of_items() == 4 {
            self.render_scrollable_menu(area, buf);
        } else {
            self.render_fixed_menu(area, buf);
        }
    }

    fn render_fixed_menu(&self, area: Rect, buf: &mut Buffer) {
        let layout = App::vertical_menu_layout(area);
        let icon_areas = App::split_area_in_thirds_flex(layout[1]);
        let colors = self.generate_menu_icon_colors();
        self.render_icon(MenuIcon::Configs, colors[0], icon_areas[0], buf);
        self.render_icon(MenuIcon::Apple, colors[1], icon_areas[1], buf);
        self.render_icon(MenuIcon::NewConfig, colors[2], icon_areas[2], buf);
    }

    fn render_scrollable_menu(&self, area: Rect, buf: &mut Buffer) {
        let layout = App::vertical_menu_layout(area);
        let icon_areas = App::split_area_in_thirds_flex(layout[1]);
        let colors = self.generate_menu_icon_colors();
        if self.menu.get_page_num() == 0 {
            self.render_icon(MenuIcon::Configs, colors[0], icon_areas[0], buf);
            self.render_icon(MenuIcon::Play, colors[1], icon_areas[1], buf);
            self.render_icon(MenuIcon::Apple, colors[2], icon_areas[2], buf);
        } else {
            self.render_icon(MenuIcon::NewConfig, colors[0], icon_areas[1], buf);
        }
    }


    fn render_timer_widget(&self, area: Rect, buf: &mut Buffer) {
        let timer_rect = App::timer_area(area);
        let timer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(17),    //Icon
                Constraint::Length(1),  //Timer
                Constraint::Length(1),  //Blank space
                Constraint::Length(1)   //Cycles
            ]).split(timer_rect);
        let cycles_block = Block::new();
        self.render_timer(timer_layout[1], buf);
        let text3 = Paragraph::new(self.pomodoro.cycle_string_cache.as_str()).centered().block(cycles_block);
        self.handle_timer_icon(timer_layout[0], buf);
        text3.render(timer_layout[3], buf);
    }

    fn render_timer(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let clock_text = self.pomodoro.clock_text_to_string();
        let clock = Paragraph::new(clock_text).centered().block(block);
        clock.render(area, buf);
    }

    fn split_area_in_thirds_flex(area: Rect) -> Rc<[Rect]> {
        Layout::horizontal([
                Constraint::Length(25),
                Constraint::Length(25),
                Constraint::Length(25)
            ])
            .flex(Flex::SpaceAround)
            .split(area)
    }
    fn vertical_menu_layout(area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(12),
                Constraint::Fill(1),
            ])
            .split(area)
    }
    fn timer_area(frame_area: Rect) -> Rect {
        let widget_size = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(42),
            Constraint::Fill(1)
        ]).split(frame_area)[1];

        Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(24),
            Constraint::Fill(1)
        ]).split(widget_size)[1]
    }

    fn handle_timer_icon(&self, area: Rect, buf: &mut Buffer) {
        match self.pomodoro.timer_state {
            PomodoroTimerState::Working => {
                if self.animations[0].is_running() {
                    self.render_timer_animation(area, buf);
                } else {
                    self.timer_work_icon(area, buf);
                }
            },
            PomodoroTimerState::Resting => self.render_resing_animation(area, buf),
            PomodoroTimerState::Done => self.render_resing_animation(area, buf),
        }
    }

    fn render_timer_animation(&self, area: Rect, buf: &mut Buffer) {
        //This could be wayy better but I have no clue how to use HOFs now.
        match self.animations[0].current_frame {
            0 => self.timer_work_animation_frame_one_icon(area, buf),
            1 => self.timer_work_animation_frame_one_icon(area, buf),
            2 => self.timer_work_animation_frame_two_icon(area, buf),
            3 => self.timer_work_animation_frame_two_icon(area, buf),
            4 => self.timer_work_animation_frame_three_icon(area, buf),
            5 => self.timer_work_animation_frame_three_icon(area, buf),
            _ => self.timer_work_icon(area, buf)
        }
    }

    fn render_resing_animation(&self, area: Rect, buf: &mut Buffer) {
        match self.animations[1].current_frame {
            0 => self.timer_break_icon_frame_two(area, buf),
            _ => self.timer_break_icon_frame_one(area, buf)
        }
    }

    fn render_new_config_popup(&self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect(30, 30, area);
        let split_area = App::split_area_in_thirds_flex(popup_area);
        Paragraph::new("Work 00").centered().block(Block::new())
            .render(split_area[0], buf);
        Paragraph::new("Rest 00").centered().block(Block::new())
            .render(split_area[1], buf);
        Paragraph::new("Cycles 2").centered().block(Block::new())
            .render(split_area[2], buf);
    }

    fn render_config_list(&mut self, area: Rect, buf: &mut Buffer) {
        let outer_area = centered_rect(60, 20, area);
        let inner_area = centered_rect(50, 10, area);
        let list_outer_block = Block::new().bg(Color::DarkGray);
        let list_inner_block = Block::bordered().bg(Color::DarkGray);
        let config_strings = self.config_menu.in_string_format();
        let list = List::new(config_strings)
            .block(list_inner_block)
            .highlight_symbol("> ");
        StatefulWidget::render(list, inner_area, buf, &mut self.config_menu.list_state);
        list_outer_block.render(outer_area, buf);
    }
}
impl Widget for &mut App{
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            AppState::Menu => self.render_menu(area, buf),
            AppState::PomodoroState => self.render_timer_widget(area, buf),
            AppState::ConfigSelection => self.render_config_list(area, buf),
            AppState::NewConfig => self.render_new_config_popup(area, buf),
            AppState:: ConfirmReturnToMenu => {}
        }
    }
}

fn centered_rect(x_percent: u16, y_percent: u16, area: Rect) -> Rect {
    let horizontal_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - x_percent) / 2),
            Constraint::Percentage(x_percent),
            Constraint::Percentage((100 - x_percent) / 2)
        ]).split(area);
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - y_percent) / 2),
            Constraint::Percentage(y_percent),
            Constraint::Percentage((100 - y_percent) / 2)
        ])
        .split(horizontal_area[1])[1]
}


