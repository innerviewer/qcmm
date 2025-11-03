use crate::qcm::{Answer, ChoiceQuestion, FillInTheBlanksQuestion, QCM, Question, Segment};
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
use std::fmt::{Display, Formatter};
use std::io;
use strum_macros::AsRefStr;

#[derive(Debug, AsRefStr)]
pub enum CurrentScreen {
    Main,
    Choosing,
    Creating,
    #[strum(to_string = "Answering")]
    Answering(AnsweringState),
    Exiting,
}

impl std::fmt::Display for CurrentScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnsweringMode {
    Navigating, // just scrolling/selecting
    Answering,  // actively answering one question
}

#[derive(Debug)]
pub struct AnsweringState {
    pub mode: AnsweringMode,
    pub current_question: usize,
    pub scroll_offset: u16,
    pub qcm: QCM,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub exit: bool,
}

impl App {
    pub fn default() -> App {
        App {
            current_screen: CurrentScreen::Main,
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

    pub fn test_qcm(&mut self) {
        let test_single_choice = Question::SingleChoice(ChoiceQuestion {
            prompt: "What is the Capital of France?".to_string(),
            options: vec![
                "Paris".to_string(),
                "Berlin".to_string(),
                "Kiev".to_string(),
                "Washington".to_string(),
            ],
            correct_answer: Answer::Single(0),
            user_answer: None,
            points: 1.0,
        });

        let test_multiple_choice = Question::MultipleChoice(ChoiceQuestion {
            prompt: "Select even numbers.".to_string(),
            options: vec![
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
                "6".to_string(),
            ],
            correct_answer: Answer::Multiple(vec![0, 2, 4]),
            user_answer: None,
            points: 1.0,
        });

        let test_fill_blanks = Question::FillInTheBlanks(FillInTheBlanksQuestion {
            segments: vec![
                Segment::Text("The capital of Germany is ".to_string()),
                Segment::Blank {
                    correct_answers: vec!["Berlin".to_string(), "berlin".to_string()],
                    user_answer: None,
                    points: 1.0,
                },
                Segment::Text(". ".to_string()),
                Segment::Text("The capital of Italy is ".to_string()),
                Segment::Blank {
                    correct_answers: vec!["Rome".to_string(), "rome".to_string()],
                    user_answer: None,
                    points: 1.0,
                },
                Segment::Text(".".to_string()),
            ],
        });

        let test_qcm = QCM {
            title: "Partiel 2025-10-22".to_string(),
            resource: "R1.03".to_string(),
            questions: vec![test_single_choice, test_multiple_choice, test_fill_blanks],
        };

        self.current_screen = CurrentScreen::Answering(AnsweringState {
            current_question: 0,
            qcm: test_qcm,
            scroll_offset: 0,
            mode: AnsweringMode::Navigating,
        });
    }

    pub fn increment_counter(&mut self) {}

    pub fn decrement_counter(&mut self) {}
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match &self.current_screen {
            // TODO: pass self to each?
            CurrentScreen::Main => ui::render_main_screen(area, buf, self),
            CurrentScreen::Choosing => ui::render_choosing_screen(area, buf, self),
            CurrentScreen::Creating => ui::render_creating_screen(area, buf, self),
            CurrentScreen::Answering(q) => ui::render_answering_screen(area, buf, self),
            CurrentScreen::Exiting => ui::render_exiting_screen(area, buf, self),
        }
    }
}
