#![allow(dead_code, unused_variables, unused_imports, unused_imports_braces)]

use color_eyre::Result;
use std::env;

mod app;
mod events;
mod qcm;
mod serialization;
mod ui;

use crate::app::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    // TODO: remove when serialization is tested enough
    let test_single_choice = qcm::Question::SingleChoice(qcm::SingleChoiceQuestion {
        prompt: "What is the Capital of France?".to_string(),
        options: vec![
            "Paris".to_string(),
            "Berlin".to_string(),
            "Kiev".to_string(),
            "Washington".to_string(),
        ],
        correct_index: 0,
    });

    let test_multiple_choice = qcm::Question::MultipleChoice(qcm::MultipleChoiceQuestion {
        prompt: "Select even numbers.".to_string(),
        options: vec![
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
        ],
        correct_indices: vec![0, 2, 4],
    });

    let test_fill_blanks = qcm::Question::FillInTheBlanks(qcm::FillInTheBlanksQuestion {
        segments: vec![
            qcm::Segment::Text("The capital of Germany is ".to_string()),
            qcm::Segment::Blank {
                acceptable_answers: vec!["Berlin".to_string(), "berlin".to_string()],
            },
            qcm::Segment::Text(". ".to_string()),
            qcm::Segment::Text("The capital of Italy is ".to_string()),
            qcm::Segment::Blank {
                acceptable_answers: vec!["Rome".to_string(), "rome".to_string()],
            },
            qcm::Segment::Text(".".to_string()),
        ],
    });

    let test_qcm = qcm::QCM {
        title: "Partiel 2025-10-22".to_string(),
        resource: "R1.03".to_string(),
        questions: vec![test_single_choice, test_multiple_choice, test_fill_blanks],
    };

    let metadata = qcm::Metadata {
        created_at: "2025-10-22".to_string().into(),
        author: "innerviewer".to_string().into(),
    };

    let qcm_file = qcm::QCMFile {
        version: 1,
        metadata: Some(metadata),
        qcm: test_qcm,
    };

    serialization::serialize_qcm(
        std::env::current_dir().unwrap().join("test.toml").as_path(),
        &qcm_file,
    );

    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);

    ratatui::restore();
    Ok(app_result?)
}
