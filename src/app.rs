use crate::{events, ui};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::collections::HashMap;
use std::io;

pub enum CurrentScreen {
    Main,
    Choosing,
    Creating,
    Answering,
    Exiting,
}

pub enum CurrentlyAnswering {
    SingleChoice,
    MultipleChoice,
    FillInTheBlanks,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub currently_answering: Option<CurrentlyAnswering>,
    pub exit: bool,
}

impl App {
    pub fn default() -> App {
        App {
            current_screen: CurrentScreen::Main,
            currently_answering: None,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            events::handle_events(self)?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn increment_counter(&mut self) {}

    pub fn decrement_counter(&mut self) {}
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.current_screen {
            CurrentScreen::Main => ui::render_main_screen(area, buf),
            CurrentScreen::Choosing => ui::render_choosing_screen(area, buf),
            CurrentScreen::Creating => ui::render_creating_screen(area, buf),
            CurrentScreen::Answering => ui::render_answering_screen(area, buf),
            CurrentScreen::Exiting => ui::render_exiting_screen(area, buf),
        }
    }
}
