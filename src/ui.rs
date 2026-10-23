use ratatui::widgets::{Widget, StatefulWidget, Block,
    Paragraph, List, ListItem, Wrap, Padding};
use ratatui::layout::{Layout, Rect, Constraint, Direction, Alignment, Flex};
use std::rc::Rc;
use ratatui::symbols::*;
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::style::{Color, Stylize};
use crate::app::{App, AppState};
use crate::icons::*;
use crate::clock::Clock;
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
}
impl Widget for &mut App{
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            AppState::Menu => self.render_menu(area, buf),
            AppState::PomodoroState => self.render_timer_widget(area, buf),
            AppState::ConfigSelection => {},
            AppState::NewConfig => {},
            AppState:: ConfirmReturnToMenu => {}
        }
    }
}
