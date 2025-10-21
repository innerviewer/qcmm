use crate::app::{App, CurrentScreen, CurrentlyAnswering};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget, Wrap},
};

pub struct UI<'a> {
    app: &'a App,
}

impl<'a> UI<'a> {
    pub fn new(app: &'a App) -> UI<'a> {
        UI { app }
    }

    fn render_main_screen(&self, area: Rect, buf: &mut Buffer) {}
    fn render_choosing_screen(&self, area: Rect, buf: &mut Buffer) {}
    fn render_creating_screen(&self, area: Rect, buf: &mut Buffer) {}
    fn render_answering_screen(&self, area: Rect, buf: &mut Buffer) {}
    fn render_exiting_screen(&self, area: Rect, buf: &mut Buffer) {}
}

impl<'a> Widget for UI<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.app.current_screen {
            CurrentScreen::Main => self.render_main_screen(area, buf),
            CurrentScreen::Choosing => self.render_choosing_screen(area, buf),
            CurrentScreen::Creating => self.render_creating_screen(area, buf),
            CurrentScreen::Answering => self.render_answering_screen(area, buf),
            CurrentScreen::Exiting => self.render_exiting_screen(area, buf),
        }
    }
}
