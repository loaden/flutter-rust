use iced::executor;
use iced::widget::{button, column, container, row};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};

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
    debug: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Next,
    Back,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self { debug: true }, Command::none())
    }

    fn title(&self) -> String {
        String::from("App - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Next => window::close(),
            Message::Back => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        "hi".into()
    }
}
