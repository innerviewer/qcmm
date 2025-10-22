use std::fs::File;
use std::path::Path;

struct Serializer {}

pub fn deserialize_questions(filepath: &std::path::Path) {
    if (!filepath.exists()) {
        todo!();
        return;
    }

    // TODO: better handle an error when opening file?
    let mut file = File::open(filepath).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents);
}
