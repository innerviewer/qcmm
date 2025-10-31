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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct QCM {
    pub resource: String,
    pub title: String,
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceQuestion {
    pub prompt: String,
    pub options: Answer,
    pub correct_answer: Answer,
    pub user_answer: Option<Answer>,
    pub points: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FillInTheBlanksQuestion {
    pub segments: Vec<Segment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Segment {
    Text(String),
    Blank {
        correct_answers: Answer,
        user_answer: Option<Answer>,
        points: f32,
    },
}

impl Answerable for ChoiceQuestion {
    fn is_correct(&self) -> bool {
        match (&self.correct_answer, &self.user_answer) {
            (a, Some(u)) => a == u,
            _ => false,
        }
    }

    fn answer(&mut self, ans: Answer) {
        if let Answer::Single(i) = ans {
            self.user_answer = Some(i);
        } else {
            // TODO: don't panic
            panic!("Invalid answer type for SingleChoiceQuestion");
        }
    }
}

impl Answerable for FillInTheBlanksQuestion {
    fn is_correct(&self) -> bool {
        for segmen in
        match (&self.correct_answer, &self.user_answer) {
            (a, Some(u)) => a == u,
            _ => false,
        }
    }

    fn answer(&mut self, ans: Answer) {
        if let Answer::Single(i) = ans {
            self.user_answer = Some(i);
        } else {
            // TODO: don't panic
            panic!("Invalid answer type for SingleChoiceQuestion");
        }
    }
}
