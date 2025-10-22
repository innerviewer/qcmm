use crate::qcm;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

struct Header {
    pub version: u16,
}

impl Header {
    pub fn default() -> Self {
        Header { version: 1 }
    }
}

pub fn deserialize_questions(filepath: &std::path::Path) -> qcm::QCMFile {
    if !filepath.exists() {
        todo!();
    }

    // TODO: better handle an error when opening file?
    let mut file = File::open(filepath).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let qcm_file: qcm::QCMFile =
        toml::from_str(&contents).expect("Failed to deserialize QCM .toml file!");

    qcm_file
}

pub fn serialize_qcm(filepath: &std::path::Path, qcm_file: &qcm::QCMFile) {
    // TODO: better handling of  errors
    if !filepath.parent().unwrap().exists() {
        todo!();
    }

    let mut file = File::create(filepath).unwrap();
    let toml_str = toml::to_string_pretty(&qcm_file).unwrap();

    file.write_all(toml_str.as_bytes()).unwrap();
}
