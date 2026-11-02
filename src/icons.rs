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
    NewConfig,
    Play
}
impl App {
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
                ctx.draw(&Line::new(12.0, 7.5, 11.7, 10.0, color));
                ctx.draw(&Line::new(10.5, 10.5, 10.5, 10.5, color));
            });
        canvas.render(area, buf);
    }

    //Menu Icons
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

    pub fn menu_icon_resume_study(&self, color: Color, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let icon = Canvas::default()
            .x_bounds([0.0, 23.0])
            .y_bounds([0.0, 11.0])
            .marker(Marker::Quadrant)
            .block(block)
            .paint(|ctx| {
                ctx.draw(&Line::new(8.0, 9.0, 8.0, 3.0, color));
                ctx.draw(&Line::new(8.5, 9.0, 8.5, 3.0, color));
                ctx.draw(&Line::new(9.0, 8.5, 9.0, 3.5, color));
                ctx.draw(&Line::new(9.5, 8.5, 9.5, 3.5, color));
                ctx.draw(&Line::new(10.0, 8.0, 10.0, 4.0, color));
                ctx.draw(&Line::new(10.3, 8.0, 10.3, 4.0, color));
                ctx.draw(&Line::new(10.5, 7.5, 10.5, 4.5, color));
                ctx.draw(&Line::new(11.0, 7.5, 11.0, 4.5, color));
                ctx.draw(&Line::new(11.3, 7.5, 11.3, 4.5, color));
                ctx.draw(&Line::new(11.5, 7.0, 11.5, 5.0, color));
                ctx.draw(&Line::new(12.0, 7.0, 12.0, 5.0, color));
                ctx.draw(&Line::new(12.5, 6.5, 12.5, 6.0, color));
                ctx.draw(&Line::new(13.0, 6.5, 13.0, 6.0, color));
            });
        icon.render(area, buf);
    }

    //Book Icon
    pub fn timer_work_icon(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let icon = Canvas::default()
            .x_bounds([0.0, 40.0])
            .y_bounds([0.0, 14.0])
            .marker(Marker::Quadrant)
            .block(block)
            .paint(|ctx| {
                //Edges of right page
                ctx.draw(&Line::new(30.0, 3.0, 34.0, 9.5, Color::White));
                ctx.draw(&Line::new(31.0, 2.5, 35.0, 10.0, Color::White));

                //Bottom of right page
                ctx.draw(&Line::new(30.0, 3.0, 29.0, 3.0, Color::White));
                ctx.draw(&Line::new(29.0, 3.5, 21.0, 3.5, Color::White));
                ctx.draw(&Line::new(20.0, 3.0, 18.5, 3.0, Color::White));

                //Top of right page
                ctx.draw(&Line::new(32.5, 10.0, 34.0, 10.0, Color::White));
                ctx.draw(&Line::new(32.0, 10.5, 24.0, 10.5, Color::White));
                ctx.draw(&Line::new(24.0, 10.0, 22.5, 10.0, Color::White));

                //Edges of left page
                ctx.draw(&Line::new(18.5, 3.0, 22.5, 10.0, Color::White));
                ctx.draw(&Line::new(8.0, 3.1, 12.0, 9.9, Color::White));

                //Top of left page
                ctx.draw(&Line::new(12.0, 9.9, 13.0, 9.9, Color::White));
                ctx.draw(&Line::new(13.5, 10.5, 19.5, 10.5, Color::White));
                ctx.draw(&Line::new(20.0, 9.9, 21.0, 9.9, Color::White));

                //Bottom of left page
                ctx.draw(&Line::new(8.0, 3.1, 9.5, 3.1, Color::White));
                ctx.draw(&Line::new(10.0, 3.5, 16.5, 3.5, Color::White));

                //Book Cover
                ctx.draw(&Line::new(22.0, 2.5, 28.0, 2.5, Color::White));
                ctx.draw(&Line::new(10.5, 2.5, 17.5, 2.5, Color::White));

                //Left Page
                ctx.draw(&Line::new(13.0, 8.5, 18.0, 8.5, Color::White));
                ctx.draw(&Line::new(12.5, 7.5, 17.5, 7.5, Color::White));
                ctx.draw(&Line::new(12.0, 6.0, 16.0, 6.0, Color::White));

                //Right Page
                ctx.draw(&Line::new(24.0, 8.5, 30.0, 8.5, Color::White));
                ctx.draw(&Line::new(23.5, 7.5, 29.5, 7.5, Color::White));
                ctx.draw(&Line::new(23.0, 6.8, 28.5, 6.8, Color::White));
                ctx.draw(&Line::new(22.2, 5.2, 27.2, 5.2, Color::White));

            });
        icon.render(area, buf);
    }

    pub fn timer_work_animation_frame_one_icon(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let icon = Canvas::default()
            .x_bounds([0.0, 40.0])
            .y_bounds([0.0, 14.0])
            .marker(Marker::Quadrant)
            .block(block)
            .paint(|ctx| {
                //Edges of right page
                ctx.draw(&Line::new(30.0, 3.0, 34.0, 9.5, Color::White));
                ctx.draw(&Line::new(31.0, 2.5, 35.0, 10.0, Color::White));

                //Bottom of right page
                ctx.draw(&Line::new(30.0, 3.0, 29.0, 3.0, Color::White));
                ctx.draw(&Line::new(29.0, 3.5, 21.0, 3.5, Color::White));
                ctx.draw(&Line::new(20.0, 3.0, 18.5, 3.0, Color::White));

                //Top of right page
                ctx.draw(&Line::new(32.5, 10.0, 34.0, 10.0, Color::White));
                //ctx.draw(&Line::new(32.0, 10.5, 24.0, 10.5, Color::White));
                //ctx.draw(&Line::new(24.0, 10.0, 22.5, 10.0, Color::White));

                //Edges of left page
                ctx.draw(&Line::new(18.5, 3.0, 22.5, 10.0, Color::White));
                ctx.draw(&Line::new(8.0, 3.1, 12.0, 9.9, Color::White));

                //Top of left page
                ctx.draw(&Line::new(12.0, 9.9, 13.0, 9.9, Color::White));
                ctx.draw(&Line::new(13.5, 10.5, 19.5, 10.5, Color::White));
                ctx.draw(&Line::new(20.0, 9.9, 21.0, 9.9, Color::White));

                //Bottom of left page
                ctx.draw(&Line::new(8.0, 3.1, 9.5, 3.1, Color::White));
                ctx.draw(&Line::new(10.0, 3.5, 16.5, 3.5, Color::White));

                //Book Cover
                ctx.draw(&Line::new(22.0, 2.5, 28.0, 2.5, Color::White));
                ctx.draw(&Line::new(10.5, 2.5, 17.5, 2.5, Color::White));

                //Left Page
                ctx.draw(&Line::new(13.0, 8.5, 18.0, 8.5, Color::White));
                ctx.draw(&Line::new(12.5, 7.5, 17.5, 7.5, Color::White));
                ctx.draw(&Line::new(12.0, 6.0, 16.0, 6.0, Color::White));

                //Top Edge of Flipped Page
                ctx.draw(&Line::new(21.7, 10.0, 24.7, 11.5, Color::White));
                ctx.draw(&Line::new(24.7, 11.5, 31.5, 12.2, Color::White));

                //Bottom Edge of Flipped Page
                ctx.draw(&Line::new(18.7, 4.0, 21.7, 5.5, Color::White));
                ctx.draw(&Line::new(21.7, 5.5, 28.5, 6.0, Color::White));


                //Right Edge of flipped page
                ctx.draw(&Line::new(28.2, 5.7, 31.5, 11.7, Color::White));
            });
        icon.render(area, buf);
    }
    pub fn timer_work_animation_frame_two_icon(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let icon = Canvas::default()
            .x_bounds([0.0, 40.0])
            .y_bounds([0.0, 14.0])
            .marker(Marker::Quadrant)
            .block(block)
            .paint(|ctx| {
                //Edges of right page
                ctx.draw(&Line::new(30.0, 3.0, 34.0, 9.5, Color::White));
                ctx.draw(&Line::new(31.0, 2.5, 35.0, 10.0, Color::White));

                //Bottom of right page
                ctx.draw(&Line::new(30.0, 3.0, 29.0, 3.0, Color::White));
                ctx.draw(&Line::new(29.0, 3.5, 21.0, 3.5, Color::White));
                ctx.draw(&Line::new(20.0, 3.0, 18.5, 3.0, Color::White));

                //Top of right page
                ctx.draw(&Line::new(32.5, 10.0, 34.0, 10.0, Color::White));
                ctx.draw(&Line::new(32.0, 10.5, 25.0, 10.5, Color::White));

                //Edges of left page
                ctx.draw(&Line::new(18.5, 3.0, 22.5, 10.0, Color::White));
                ctx.draw(&Line::new(8.0, 3.1, 12.0, 9.9, Color::White));

                //Top of left page
                ctx.draw(&Line::new(12.0, 9.9, 13.0, 9.9, Color::White));
                ctx.draw(&Line::new(13.5, 10.5, 19.5, 10.5, Color::White));
                ctx.draw(&Line::new(20.0, 9.9, 21.0, 9.9, Color::White));

                //Bottom of left page
                ctx.draw(&Line::new(8.0, 3.1, 9.5, 3.1, Color::White));
                ctx.draw(&Line::new(10.0, 3.5, 16.5, 3.5, Color::White));

                //Book Cover
                ctx.draw(&Line::new(22.0, 2.5, 28.0, 2.5, Color::White));
                ctx.draw(&Line::new(10.5, 2.5, 17.5, 2.5, Color::White));

                //Left Page
                ctx.draw(&Line::new(13.0, 8.5, 18.0, 8.5, Color::White));
                ctx.draw(&Line::new(12.5, 7.5, 17.5, 7.5, Color::White));
                ctx.draw(&Line::new(12.0, 6.0, 16.0, 6.0, Color::White));

                //Right Page
                ctx.draw(&Line::new(24.0, 8.5, 30.0, 8.5, Color::White));
                ctx.draw(&Line::new(23.5, 7.5, 29.5, 7.5, Color::White));
                ctx.draw(&Line::new(23.0, 6.8, 28.5, 6.8, Color::White));
                ctx.draw(&Line::new(22.2, 5.2, 27.2, 5.2, Color::White));
                
                //Flipped Page Top Edge
                ctx.draw(&Line::new(22.0, 10.0, 23.5, 12.0, Color::White));
                ctx.draw(&Line::new(23.5, 12.0, 24.5, 12.5, Color::White));

                //Left edge of fhipped page
                ctx.draw(&Line::new(25.0, 12.5, 22.0, 6.0, Color::White));

                //Bottom Edge of flipped page
                ctx.draw(&Line::new(19.0, 3.0, 19.5, 5.0, Color::White));
                ctx.draw(&Line::new(20.5, 5.0, 21.5, 5.5, Color::White));


            });
        icon.render(area, buf);
    }

    pub fn timer_work_animation_frame_three_icon(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let icon = Canvas::default()
            .x_bounds([0.0, 40.0])
            .y_bounds([0.0, 14.0])
            .marker(Marker::Quadrant)
            .block(block)
            .paint(|ctx| {
                //Edges of right page
                ctx.draw(&Line::new(30.0, 3.0, 34.0, 9.5, Color::White));
                ctx.draw(&Line::new(31.0, 2.5, 35.0, 10.0, Color::White));

                //Bottom of right page
                ctx.draw(&Line::new(30.0, 3.0, 29.0, 3.0, Color::White));
                ctx.draw(&Line::new(29.0, 3.5, 21.0, 3.5, Color::White));
                ctx.draw(&Line::new(20.0, 3.0, 18.5, 3.0, Color::White));

                //Top of right page
                ctx.draw(&Line::new(32.5, 10.0, 34.0, 10.0, Color::White));
                ctx.draw(&Line::new(32.0, 10.5, 24.0, 10.5, Color::White));
                ctx.draw(&Line::new(24.0, 10.0, 22.5, 10.0, Color::White));

                //Edges of left page
                ctx.draw(&Line::new(18.5, 3.0, 22.5, 10.0, Color::White));
                ctx.draw(&Line::new(8.0, 3.1, 12.0, 9.9, Color::White));

                //Top of left page
                ctx.draw(&Line::new(12.0, 9.9, 13.0, 9.9, Color::White));

                //Bottom of left page
                ctx.draw(&Line::new(8.0, 3.1, 9.5, 3.1, Color::White));
                ctx.draw(&Line::new(10.0, 3.5, 16.5, 3.5, Color::White));

                //Book Cover
                ctx.draw(&Line::new(22.0, 2.5, 28.0, 2.5, Color::White));
                ctx.draw(&Line::new(10.5, 2.5, 17.5, 2.5, Color::White));

                //Right Page
                ctx.draw(&Line::new(24.0, 8.5, 30.0, 8.5, Color::White));
                ctx.draw(&Line::new(23.5, 7.5, 29.5, 7.5, Color::White));
                ctx.draw(&Line::new(23.0, 6.8, 28.5, 6.8, Color::White));
                ctx.draw(&Line::new(22.2, 5.2, 27.2, 5.2, Color::White));

                //Top Edge of Flipped Page
                ctx.draw(&Line::new(18.0, 11.0, 21.0, 9.9, Color::White));
                ctx.draw(&Line::new(14.0, 11.6, 18.0, 11.0, Color::White));

                //Side of Flipped Page
                ctx.draw(&Line::new(10.1, 5.0, 14.2, 11.5, Color::White));

                //Top Edge of Flipped Page
                ctx.draw(&Line::new(10.0, 5.0, 13.0, 4.6, Color::White));
                ctx.draw(&Line::new(13.5, 4.6, 17.5, 4.0, Color::White));

            });
        icon.render(area, buf);
    }

    //Tea Icon
    pub fn timer_break_icon_frame_one(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let icon = Canvas::default()
            .x_bounds([0.0, 40.0])
            .y_bounds([0.0, 14.0])
            .marker(Marker::Quadrant)
            .block(block)
            .paint(|ctx| {
                //Tea
                ctx.draw(&FilledLine::new(18.5, 6.8, 23.5, 6.8, 7.5, Color::DarkGray));
                ctx.draw(&Line::new(13.5, 7.5, 28.5, 7.5, Color::DarkGray));
                //Base of cup
                ctx.draw(&Line::new(15.0, 3.0, 26.0, 3.0, Color::White));

                //Left side of cup
                ctx.draw(&Line::new(15.0, 3.0, 12.0, 4.0, Color::White));
                ctx.draw(&Line::new(11.5, 4.2, 11.0, 4.2, Color::White));
                ctx.draw(&Line::new(10.5, 4.7, 10.0, 6.2, Color::White));
                ctx.draw(&Line::new(10.0, 6.2, 10.0, 7.5, Color::White));

                //Right side of cup
                ctx.draw(&Line::new(26.0, 3.0, 29.0, 4.0, Color::White));
                ctx.draw(&Line::new(29.5, 4.2, 30.0, 4.2, Color::White));
                ctx.draw(&Line::new(30.5, 4.7, 31.0, 6.2, Color::White));
                ctx.draw(&Line::new(31.0, 6.2, 31.0, 7.5, Color::White));
                
                //Rim
                ctx.draw(&Line::new(10.5, 7.5, 13.0, 7.3, Color::White));
                ctx.draw(&Line::new(14.5, 6.8, 17.5, 6.8, Color::White));

                ctx.draw(&Line::new(12.0, 8.0, 13.0, 8.2, Color::White));
                ctx.draw(&Line::new(14.0, 8.5, 17.5, 8.5, Color::White));

                //Center of rim
                ctx.draw(&Line::new(18.0, 6.3, 23.5, 6.3, Color::White));
                ctx.draw(&Line::new(18.0, 9.0, 23.5, 9.0, Color::White));

                ctx.draw(&Line::new(30.5, 7.5, 28.0, 7.3, Color::White));
                ctx.draw(&Line::new(27.5, 6.8, 24.0, 6.8, Color::White));

                ctx.draw(&Line::new(27.5, 8.5, 24.5, 8.5, Color::White));

                //Steam
                ctx.draw(&Line::new(18.0, 10.0, 18.5, 10.5, Color::White));
                ctx.draw(&Line::new(18.5, 10.5, 18.5, 11.0, Color::White));
                ctx.draw(&Line::new(18.0, 11.5, 18.5, 12.0, Color::White));

                ctx.draw(&Line::new(20.0, 10.0, 20.5, 11.0, Color::White));
                ctx.draw(&Line::new(20.5, 11.0, 20.5, 11.0, Color::White));
                ctx.draw(&Line::new(20.0, 11.5, 20.0, 12.0, Color::White));
                ctx.draw(&Line::new(20.5, 12.5, 20.5, 12.5, Color::White));

                ctx.draw(&Line::new(24.0, 10.3, 23.5, 10.5, Color::White));
                ctx.draw(&Line::new(23.5, 10.5, 23.5, 11.0, Color::White));
                ctx.draw(&Line::new(24.0, 11.5, 23.5, 12.0, Color::White));

                ctx.draw(&Line::new(26.0, 9.7, 26.5, 10.2, Color::White));
                ctx.draw(&Line::new(26.0, 10.5, 26.5, 11.2, Color::White));

                ctx.draw(&Line::new(15.0, 9.7, 15.5, 10.2, Color::White));
                ctx.draw(&Line::new(15.0, 10.5, 15.0, 10.5, Color::White));
            });
        icon.render(area, buf);
    }

    pub fn timer_break_icon_frame_two(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();
        let icon = Canvas::default()
            .x_bounds([0.0, 40.0])
            .y_bounds([0.0, 14.0])
            .marker(Marker::Quadrant)
            .block(block)
            .paint(|ctx| {
                ctx.draw(&FilledLine::new(18.5, 6.8, 23.5, 6.8, 7.5, Color::DarkGray));
                ctx.draw(&Line::new(13.5, 7.5, 28.5, 7.5, Color::DarkGray));
                //Base of cup
                ctx.draw(&Line::new(15.0, 3.0, 26.0, 3.0, Color::White));

                //Left side of cup
                ctx.draw(&Line::new(15.0, 3.0, 12.0, 4.0, Color::White));
                ctx.draw(&Line::new(11.5, 4.2, 11.0, 4.2, Color::White));
                ctx.draw(&Line::new(10.5, 4.7, 10.0, 6.2, Color::White));
                ctx.draw(&Line::new(10.0, 6.2, 10.0, 7.5, Color::White));

                //Right side of cup
                ctx.draw(&Line::new(26.0, 3.0, 29.0, 4.0, Color::White));
                ctx.draw(&Line::new(29.5, 4.2, 30.0, 4.2, Color::White));
                ctx.draw(&Line::new(30.5, 4.7, 31.0, 6.2, Color::White));
                ctx.draw(&Line::new(31.0, 6.2, 31.0, 7.5, Color::White));
                
                //Rim
                ctx.draw(&Line::new(10.5, 7.5, 13.0, 7.3, Color::White));
                ctx.draw(&Line::new(14.5, 6.8, 17.5, 6.8, Color::White));

                ctx.draw(&Line::new(12.0, 8.0, 13.0, 8.2, Color::White));
                ctx.draw(&Line::new(14.0, 8.5, 17.5, 8.5, Color::White));

                //Center of rim
                ctx.draw(&Line::new(18.0, 6.3, 23.5, 6.3, Color::White));
                ctx.draw(&Line::new(18.0, 9.0, 23.5, 9.0, Color::White));

                ctx.draw(&Line::new(30.5, 7.5, 28.0, 7.3, Color::White));
                ctx.draw(&Line::new(27.5, 6.8, 24.0, 6.8, Color::White));

                ctx.draw(&Line::new(27.5, 8.5, 24.5, 8.5, Color::White));

                //Steam
                ctx.draw(&Line::new(18.5, 10.5, 18.0, 11.0, Color::White));
                ctx.draw(&Line::new(18.0, 11.0, 18.0, 11.5, Color::White));
                ctx.draw(&Line::new(18.5, 12.0, 18.0, 12.5, Color::White));

                ctx.draw(&Line::new(20.5, 10.5, 20.0, 11.5, Color::White));
                ctx.draw(&Line::new(20.0, 11.5, 20.0, 11.5, Color::White));
                ctx.draw(&Line::new(20.5, 12.0, 20.5, 12.5, Color::White));
                ctx.draw(&Line::new(20.0, 12.8, 20.0, 13.0, Color::White));

                ctx.draw(&Line::new(23.5, 10.5, 23.5, 10.5, Color::White));
                ctx.draw(&Line::new(24.0, 11.0, 24.0, 11.5, Color::White));
                ctx.draw(&Line::new(23.5, 12.0, 24.0, 12.5, Color::White));

                ctx.draw(&Line::new(26.0, 10.2, 26.5, 10.7, Color::White));
                ctx.draw(&Line::new(26.0, 11.0, 26.5, 12.2, Color::White));

                ctx.draw(&Line::new(14.5, 10.2, 15.0, 10.7, Color::White));
                ctx.draw(&Line::new(14.5, 11.0, 14.5, 11.0, Color::White));
            });
        icon.render(area, buf);

    }

    fn render_loaded_config_info(&self, color: Color, area: Rect, buf: &mut Buffer) {
        match self.config {
            Some(config) => {
                self.render_icon_text(config.to_string_icon_format().as_str(), color, area, buf);
            },
            None => self.render_icon_text("No timer config loaded", color, area, buf)
        }
    }

    pub fn render_icon(&self, icon: MenuIcon, colors: Color, area: Rect, buf: &mut Buffer) {
        let icon_layout = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1)
        ]).split(area);

        match icon {
            MenuIcon::Configs => {
                self.menu_icon_config(colors, icon_layout[0], buf);
                self.render_icon_text("Configs", colors, icon_layout[1], buf);
            },
            MenuIcon::Apple => {
                self.menu_icon_apple(colors, icon_layout[0], buf);
                self.render_icon_text("Start", colors, icon_layout[1], buf);
                self.render_loaded_config_info(colors, icon_layout[3], buf);
            },
            MenuIcon::NewConfig => {
                self.menu_icon_new_config(colors, icon_layout[0], buf);
                self.render_icon_text("New Config", colors, icon_layout[1], buf);
            },
            MenuIcon::Play => {
                self.menu_icon_resume_study(colors, icon_layout[0], buf);
                self.render_icon_text("Resume Session", colors, icon_layout[1], buf);

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
        icon_colors[self.menu.selected() % 3] = Color::White;
        icon_colors
    }
}
