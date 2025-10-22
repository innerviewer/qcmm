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
    SingleChoice(SingleChoiceQuestion),
    #[serde(rename = "multiple")]
    MultipleChoice(MultipleChoiceQuestion),
    #[serde(rename = "blanks")]
    FillInTheBlanks(FillInTheBlanksQuestion),
}

pub trait Validatable {
    type UserAnswer;

    fn validate(&self, answer: &Self::UserAnswer) -> bool;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleChoiceQuestion {
    pub prompt: String,
    pub options: Vec<String>,
    pub correct_index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultipleChoiceQuestion {
    pub prompt: String,
    pub options: Vec<String>,
    pub correct_indices: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FillInTheBlanksQuestion {
    pub segments: Vec<Segment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Segment {
    Text(String),
    Blank { acceptable_answers: Vec<String> },
}

impl Validatable for SingleChoiceQuestion {
    type UserAnswer = usize;

    fn validate(&self, answer: &usize) -> bool {
        *answer == self.correct_index
    }
}

impl Validatable for MultipleChoiceQuestion {
    type UserAnswer = Vec<usize>;

    fn validate(&self, answer: &Vec<usize>) -> bool {
        let mut sorted_user = answer.clone();
        let mut sorted_correct = self.correct_indices.clone();
        sorted_user.sort();
        sorted_correct.sort();
        sorted_user == sorted_correct
    }
}

impl Validatable for FillInTheBlanksQuestion {
    type UserAnswer = Vec<String>;

    fn validate(&self, answers: &Vec<String>) -> bool {
        let mut idx = 0;
        for segment in &self.segments {
            if let Segment::Blank { acceptable_answers } = segment {
                if let Some(user_input) = answers.get(idx) {
                    if !acceptable_answers.iter().any(|ans| ans == user_input)
                    //.any(|ans| ans.eq_ignore_ascii_case(user_input))
                    {
                        return false;
                    }
                } else {
                    return false;
                }
                idx += 1;
            }
        }
        true
    }
}
