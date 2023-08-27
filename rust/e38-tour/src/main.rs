use iced::alignment::Horizontal;
use iced::executor;
use iced::widget::{button, checkbox, column, horizontal_space, row, slider, text, Column};
use iced::window;
use iced::{Application, Command, Element, Length, Settings, Theme};

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (800, 450),
            ..Default::default()
        },
        ..Default::default()
    };
    App::run(settings)
}

struct App {
    steps: Steps,
    debug: bool,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                steps: Steps::new(),
                debug: true,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("App - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let Self { steps, debug } = self;
        match message {
            Message::PageNext => {
                steps.go_next();
                Command::none()
            }
            Message::PageBack => {
                steps.go_back();
                Command::none()
            }
            Message::StepMessage(msg) => {
                steps.update(msg, debug);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let Self { steps, .. } = self;

        let mut controls = row![];
        if steps.can_back() {
            controls = controls.push(button("Back").on_press(Message::PageBack));
        }

        controls = controls.push(horizontal_space(Length::Fill));
        if steps.has_next() {
            controls = controls.push(button("Next").on_press(Message::PageNext));
        }

        column![steps.view(self.debug).map(Message::StepMessage), controls,]
            .spacing(20)
            .padding(20)
            .into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    PageNext,
    PageBack,
    StepMessage(StepMessage),
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
                Step::Debugger,
                Step::Slider { value: 36 },
                Step::End,
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage, debug: &mut bool) {
        self.steps[self.current].update(msg, debug);
    }

    fn view(&self, debug: bool) -> Element<StepMessage> {
        self.steps[self.current].view(debug)
    }

    fn go_next(&mut self) {
        if self.has_next() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.can_back() {
            self.current -= 1;
        }
    }

    fn has_next(&self) -> bool {
        self.current + 1 < self.steps.len()
    }

    fn can_back(&self) -> bool {
        self.current > 0
    }
}

enum Step {
    Welcome,
    Debugger,
    Slider { value: u8 },
    End,
}

#[derive(Debug, Clone)]
enum StepMessage {
    DebugToggled(bool),
    SliderChanged(u8),
}

impl<'a> Step {
    fn update(&mut self, msg: StepMessage, debug: &mut bool) {
        match msg {
            StepMessage::SliderChanged(val) => {
                if let Self::Slider { value } = self {
                    *value = val;
                }
            }
            StepMessage::DebugToggled(dbg) => {
                if let Self::Debugger = self {
                    *debug = dbg;
                }
            }
        }
    }

    fn view(&self, debug: bool) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome(),
            Step::Debugger => Self::debugger(debug),
            Step::Slider { value } => Self::slider(*value),
            Step::End => Self::end(),
        }
        .into()
    }

    fn container(title: &str) -> Column<'a, StepMessage> {
        column![text(title).size(50)].spacing(20)
    }

    fn welcome() -> Column<'a, StepMessage> {
        Self::container("Welcome!").push(
            "This is a simple tour meant to showcase a bunch of widgets \
                that can be easily implemented on top of Iced.",
        )
    }

    fn debugger(debug: bool) -> Column<'a, StepMessage> {
        Self::container("Debugger")
            .push(
                "You can ask Iced to visually explain the layouting of the \
                 different elements comprising your UI!",
            )
            .push(checkbox("Explain layout", debug, StepMessage::DebugToggled))
    }

    fn slider(value: u8) -> Column<'a, StepMessage> {
        Self::container("Slider")
            .push(
                "A slider allows you to smoothly select a value from a range \
                 of values.",
            )
            .push(slider(0..=100, value, StepMessage::SliderChanged))
            .push(
                text(value.to_string())
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
            )
    }

    fn end() -> Column<'a, StepMessage> {
        Self::container("You reached the end!")
            .push("This tour will be updated as more features are added.")
            .push("Make sure to keep an eye on it!")
    }
}
