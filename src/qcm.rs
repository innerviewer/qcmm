use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QCMFile {
    pub version: u32,
    pub metadata: Option<Metadata>,
    pub qcm: QCM,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub created_at: Option<String>,
    pub author: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct QCM {
    pub resource: String,
    pub title: String,
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Question {
    #[serde(rename = "single")]
    SingleChoice(ChoiceQuestion),
    #[serde(rename = "multiple")]
    MultipleChoice(ChoiceQuestion),
    #[serde(rename = "blanks")]
    FillInTheBlanks(FillInTheBlanksQuestion),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Answer {
    Single(usize),
    Multiple(Vec<usize>),
    Fill(Vec<String>),
}

pub trait Answerable {
    fn answer(&mut self, answer: Answer);
    fn is_correct(&self) -> bool;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceQuestion {
    pub prompt: String,
    pub options: Vec<String>,
    pub correct_answer: Answer,
    pub user_answer: Option<Answer>,
    pub points: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FillInTheBlanksQuestion {
    pub segments: Vec<Segment>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Segment {
    Text(String),
    Blank {
        correct_answers: Vec<String>,
        user_answer: Option<String>,
        points: f32,
    },
}

//
// Implementations
//

impl QCM {
    /// Compute the total possible points in this QCM.
    pub fn total_points(&self) -> f32 {
        self.questions.iter().map(|q| q.total_points()).sum()
    }

    /// Compute the total points earned by the user for correct answers.
    pub fn earned_points(&self) -> f32 {
        self.questions.iter().map(|q| q.earned_points()).sum()
    }
}

impl Question {
    /// Return total points for a single question.
    fn total_points(&self) -> f32 {
        match self {
            Question::SingleChoice(q) | Question::MultipleChoice(q) => q.points,
            Question::FillInTheBlanks(q) => q
                .segments
                .iter()
                .map(|s| match s {
                    Segment::Blank { points, .. } => *points,
                    Segment::Text(_) => 0.0,
                })
                .sum(),
        }
    }

    /// Return earned points depending on correctness.
    fn earned_points(&self) -> f32 {
        match self {
            Question::SingleChoice(q) | Question::MultipleChoice(q) => {
                if q.is_correct() {
                    q.points
                } else {
                    0.0
                }
            }
            Question::FillInTheBlanks(q) => {
                let incorrect = q.incorrect_blanks();
                q.segments
                    .iter()
                    .enumerate()
                    .map(|(i, seg)| match seg {
                        Segment::Blank { points, .. } if !incorrect.contains(&i) => *points,
                        _ => 0.0,
                    })
                    .sum()
            }
        }
    }
}

impl Answerable for ChoiceQuestion {
    fn answer(&mut self, ans: Answer) {
        self.user_answer = Some(ans);
    }

    fn is_correct(&self) -> bool {
        match (&self.correct_answer, &self.user_answer) {
            (Answer::Single(a), Some(Answer::Single(u))) => a == u,
            (Answer::Multiple(a), Some(Answer::Multiple(u))) => {
                // TODO: is this the way?
                let mut a_sorted = a.clone();
                let mut u_sorted = u.clone();
                a_sorted.sort_unstable();
                u_sorted.sort_unstable();
                a_sorted == u_sorted
            }
            _ => false,
        }
    }
}

impl Answerable for FillInTheBlanksQuestion {
    fn answer(&mut self, ans: Answer) {
        if let Answer::Fill(user_fills) = ans {
            let mut fill_iter = user_fills.into_iter();
            for segment in &mut self.segments {
                if let Segment::Blank { user_answer, .. } = segment {
                    *user_answer = fill_iter.next();
                }
            }
        } else {
            panic!("Invalid answer type for FillInTheBlanksQuestion");
        }
    }

    fn is_correct(&self) -> bool {
        self.incorrect_blanks().is_empty()
    }
}

impl FillInTheBlanksQuestion {
    /// Returns the indices of blanks that are incorrect or unanswered.
    pub fn incorrect_blanks(&self) -> Vec<usize> {
        let mut incorrect = Vec::new();
        for (i, segment) in self.segments.iter().enumerate() {
            if let Segment::Blank {
                correct_answers,
                user_answer,
                ..
            } = segment
            {
                match user_answer {
                    Some(ua) if correct_answers.iter().any(|ca| ca.eq_ignore_ascii_case(ua)) => {
                        // correct
                    }
                    _ => incorrect.push(i),
                }
            }
        }
        incorrect
    }
}
