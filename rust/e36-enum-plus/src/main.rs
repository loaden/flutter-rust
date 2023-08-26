fn main() {
    let mut sts = Steps::new();
    sts.update(StepMessage::SliderChanged(19));
    sts.next();
    sts.update(StepMessage::LanguageSelected(Language::Rust));
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
            current: 1,
        }
    }

    fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }

    fn next(&mut self) {
        self.current += 1;
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

impl Step {
    fn update(&mut self, msg: StepMessage) {
        match msg {
            StepMessage::SliderChanged(val) => {
                println!("Slider New Value: {}", val);
                if let Self::Slider { value } = self {
                    println!("Slider Old Value: {}", value);
                    *value = val;
                }
                match self {
                    Self::Slider { value } => println!("Update Slider: {}", value),
                    _ => ()
                }
            },
            StepMessage::LanguageSelected(lng) => {
                println!("Language New Value: {:?}", lng);
                if let Step::Radio { selection } = self {
                    println!("Language Old Value: {:?}", selection);
                    *selection = Some(lng);
                }
                if let Step::Radio { selection } = self {
                    println!("Update Language: {:?}", selection);
                }
            }
            _ => ()
        }
    }
}

#[derive(Debug)]
enum Language {
    Rust,
    C,
    Other,
}
