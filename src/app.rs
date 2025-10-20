use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Choosing,
    Creating,
    Answering,
    Exiting,
}

pub enum CurrentlyAnswering {
    TextField,
    MultipleChoice,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub currently_answering: Option<CurrentlyAnswering>,
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
}
