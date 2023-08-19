use iced::executor;
use iced::widget::{button, column, container};
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

#[derive(Default)]
struct App {
    exit_confirm: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Confirm,
    Exit,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("App - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Confirm => window::close(),
            Message::Exit => {
                self.exit_confirm = true;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = if self.exit_confirm {
            column![
                "Are you sure you want to exit?",
                button("Yes, exit now")
                    .padding([10, 20])
                    .on_press(Message::Confirm),
            ]
        } else {
            column![
                "Click the button to exit",
                button("Exit").padding([10, 20]).on_press(Message::Exit),
            ]
        }
        .spacing(10)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}
