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
    pub qcm: Qcm,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub created_at: Option<String>,
    pub author: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Qcm {
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
    fn is_correct_index(&self, index: usize) -> bool;
    fn incorrect_answers(&self) -> Vec<usize>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceQuestion {
    pub prompt: String,
    pub options: Vec<String>,
    pub correct_answer: Answer,
    pub user_answer: Answer,
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
        user_answer: String,
        points: f32,
    },
}

//
// Implementations
//

impl Qcm {
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
                let incorrect = q.incorrect_answers();
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
        self.user_answer = ans;
    }

    fn is_correct(&self) -> bool {
        self.incorrect_answers().is_empty()
    }

    fn is_correct_index(&self, option_index: usize) -> bool {
        match (&self.correct_answer, &self.user_answer) {
            (Answer::Single(a), Answer::Single(u)) => a == u && *u == option_index,
            (Answer::Multiple(a), Answer::Multiple(u)) => {
                a.contains(&option_index) && u.contains(&option_index)
            }
            _ => false,
        }
    }

    fn incorrect_answers(&self) -> Vec<usize> {
        let mut incorrect: Vec<usize> = vec![];
        match (&self.correct_answer, &self.user_answer) {
            (Answer::Single(a), Answer::Single(u)) => {
                if a != u {
                    incorrect.push(*u);
                }
            }
            (Answer::Multiple(a), Answer::Multiple(u)) => {
                // TODO: should I leave this?
                assert_eq!(a.len(), u.len());

                for (i, _) in a.iter().enumerate() {
                    if a[i] != u[i] {
                        incorrect.push(i);
                    }
                }
            }
            _ => {}
        }

        incorrect
    }
}

impl Answerable for FillInTheBlanksQuestion {
    fn answer(&mut self, ans: Answer) {
        if let Answer::Fill(user_fills) = ans {
            let mut fill_iter = user_fills.into_iter();
            for segment in &mut self.segments {
                if let Segment::Blank { user_answer, .. } = segment {
                    *user_answer = fill_iter.next().unwrap();
                }
            }
        } else {
            panic!("Invalid answer type for FillInTheBlanksQuestion");
        }
    }

    fn is_correct(&self) -> bool {
        self.incorrect_answers().is_empty()
    }

    fn is_correct_index(&self, blank_index: usize) -> bool {
        if let Segment::Blank {
            correct_answers,
            user_answer,
            ..
        } = &self.segments[blank_index]
        {
            correct_answers
                .iter()
                .any(|ca| ca.eq_ignore_ascii_case(user_answer))
        } else {
            false
        }
    }

    fn incorrect_answers(&self) -> Vec<usize> {
        let mut incorrect = Vec::new();
        for (i, segment) in self.segments.iter().enumerate() {
            if let Segment::Blank {
                correct_answers,
                user_answer,
                ..
            } = segment
            {
                match user_answer {
                    ua if correct_answers.iter().any(|ca| ca.eq_ignore_ascii_case(ua)) => {
                        // correct
                    }
                    _ => incorrect.push(i),
                }
            }
        }
        incorrect
    }
}
