use ratatui::widgets::{Widget, StatefulWidget, Block,
    Paragraph, List, ListItem, Wrap, Padding};
use ratatui::layout::{Layout, Rect, Constraint, Direction, Alignment, Flex};
use std::rc::Rc;
use ratatui::widgets::canvas::{Canvas, Line, Rectangle, FilledLine, Context};
use ratatui::symbols::*;
use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::style::{Color, Stylize};
use crate::app::{App, AppState};
use crate::icons::*;
use crate::icons::MenuIcon;



impl App {
    //TODO
    //Add menu rendering
    pub fn render(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn render_menu(&self, area: Rect, buf: &mut Buffer) {
        let layout = App::vertical_menu_layout(area);
        let icon_areas = App::split_area_in_thirds_flex(layout[1]);
        let colors = self.generate_menu_icon_colors();
        self.render_icon(MenuIcon::Configs, &colors, icon_areas[0], buf);
        self.render_icon(MenuIcon::Apple, &colors, icon_areas[1], buf);
        self.render_icon(MenuIcon::NewConfig, &colors, icon_areas[2], buf);
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
        let block1 = Block::new();
        let block3 = Block::new();
        let text1 = Paragraph::new("").block(block1);
        self.render_timer(timer_layout[2], buf);
        let text3 = Paragraph::new("■ ■ □").centered().block(block3);
        self.timer_work_icon(timer_layout[0], buf);
        text1.render(timer_layout[1], buf);
        text3.render(timer_layout[3], buf);
    }

    fn render_timer(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let clock_text = self.pomodoro.to_string();
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
