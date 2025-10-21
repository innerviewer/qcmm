use crate::events;
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
        let title = Line::from(" QCM Manager ".bold());
        let instructions = Line::from(vec![
            " Create Test ".into(),
            "<C>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title_bottom(instructions.centered())
            .title_top(title)
            .border_set(border::ROUNDED);

        let counter_text = Text::from(vec![Line::from(vec!["Main Menu".into()])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
