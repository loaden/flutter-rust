fn main() {
    let st = Steps::new();
}

struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Self {
        Self {
            steps: vec![
                Step::Welcome,
                Step::Slider { value: 36 },
                Step::Radio { selection: None },
                Step::TextInput {
                    value: String::new(),
                    is_secure: false,
                },
                Step::End,
            ],
            current: 0,
        }
    }
}

enum Step {
    Welcome,
    Slider { value: u8 },
    Radio { selection: Option<Language> },
    TextInput { value: String, is_secure: bool },
    End,
}

enum StepMessage {
    SliderChanged(u8),
    LanguageSelected(Language),
    InputChanged(String),
}

enum Language {
    Rust,
    C,
    Other,
}
