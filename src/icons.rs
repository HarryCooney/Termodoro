use ratatui::buffer::Buffer;
use ratatui::widgets::canvas::{Canvas, Line, Rectangle, FilledLine, Context};
use ratatui::widgets::{Widget, StatefulWidget, Block, Paragraph, List, ListItem, Wrap, Padding};
use ratatui::text::Span;
use ratatui::layout::{Layout, Rect, Constraint, Direction, Alignment, Flex};
use ratatui::style::{Color, Stylize, Style};
use ratatui::symbols::*;
use crate::app::App;

pub enum MenuIcon {
    Apple,
    Configs,
    NewConfig
}
impl App<'_> {
    pub fn menu_icon_apple(&self, color: Color, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let canvas = Canvas::default()
            .x_bounds([0.0, 23.0])
            .y_bounds([0.0, 11.0])
            .marker(Marker::HalfBlock)
            .block(block)
            .paint(|ctx| {
                ctx.draw(&FilledLine::new(5.0, 3.1, 18.0, 3.0, 7.5, color));
                ctx.draw(&FilledLine::new(7.0, 2.5, 16.0, 2.5, 8.5, color));
                ctx.draw(&Line::new(12.0, 7.5, 12.0, 10.0, color));
                ctx.draw(&Line::new(10.0, 10.5, 10.0, 10.5, color));
            });
        canvas.render(area, buf);
    }
    pub fn menu_icon_config(&self, color: Color, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let canvas = Canvas::default()
            .x_bounds([0.0, 23.0])
            .y_bounds([0.0, 11.0])
            .marker(Marker::Quadrant)
            .block(block)
            .paint(|ctx| {
                ctx.draw(&Line::new(7.0, 8.5, 7.0, 3.0, color));
                ctx.draw(&Line::new(16.5, 8.5, 16.5, 3.0, color));
                ctx.draw(&Line::new(7.5, 9.0, 16.0, 9.0, color));
                ctx.draw(&Line::new(7.5, 2.5, 16.0, 2.5, color));
                ctx.draw(&Line::new(9.0, 4.0, 14.5, 4.0, color));
                ctx.draw(&Line::new(9.0, 6.0, 14.5, 6.0, color));
                ctx.draw(&Line::new(9.0, 7.5, 14.5, 7.5, color));
            });
        canvas.render(area, buf);
    }
    pub fn menu_icon_new_config(&self, color: Color, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let icon = Canvas::default()
            .x_bounds([0.0, 23.0])
            .y_bounds([0.0, 11.0])
            .marker(Marker::Quadrant)
            .block(block)
            .paint(|ctx| {
                ctx.draw(&Line::new(7.0, 8.5, 7.0, 3.0, color));
                ctx.draw(&Line::new(16.5, 6.0, 16.5, 3.0, color));
                ctx.draw(&Line::new(7.5, 9.0, 13.0, 9.0, color));
                ctx.draw(&Line::new(7.5, 2.5, 16.0, 2.5, color));
                ctx.draw(&Line::new(18.0, 9.0, 13.0, 6.0, color));
                ctx.draw(&Line::new(18.5, 9.5, 13.5, 6.5, color));
                ctx.draw(&Line::new(9.0, 4.0, 14.5, 4.0, color));
                ctx.draw(&Line::new(9.0, 6.0, 10.5, 6.0, color));
                ctx.draw(&Line::new(9.0, 7.5, 11.5, 7.5, color));
            });
        icon.render(area, buf);
    }

    pub fn render_icon(&self, icon: MenuIcon, colors: &Vec<Color>, area: Rect, buf: &mut Buffer) {
        let icon_layout = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1)
        ]).split(area);
        match icon {
            MenuIcon::Configs => {
                self.menu_icon_config(colors[0], icon_layout[0], buf);
                self.render_icon_text("Configs", colors[0], icon_layout[1], buf);
            },
            MenuIcon::Apple => {
                self.menu_icon_apple(colors[1], icon_layout[0], buf);
                self.render_icon_text("Start", colors[1], icon_layout[1], buf);
            },
            MenuIcon::NewConfig => {
                self.menu_icon_new_config(colors[2], icon_layout[0], buf);
                self.render_icon_text("New Config", colors[2], icon_layout[1], buf);
            }
        };
    }
    fn render_icon_text(&self, text: &str, color: Color, area: Rect, buf: &mut Buffer) {
        let span = Span::styled(text, Style::default().fg(color));
        let text = Paragraph::new(span).centered();
        text.render(area, buf);
    }
    
    ///Generates the colours used by each menu icon
    ///
    ///Reduces pattern matches by generating colors for each frame once for both
    ///icons and text, instead of doing so for each icon individually
    ///icon_colors[0] = Config, icon_colors[1] = Apple, icon_colors[2] = NewConfig
    pub fn generate_menu_icon_colors(&self) -> Vec<Color> {
        let mut icon_colors = vec![Color::DarkGray, Color::DarkGray, Color::DarkGray];
        match self.menu.selected() {
            0 => icon_colors[0] = Color::White,
            1 => icon_colors[1] = Color::White,
            2 => icon_colors[2] = Color::White,
            _ => {}
        }
        icon_colors
    }
}
