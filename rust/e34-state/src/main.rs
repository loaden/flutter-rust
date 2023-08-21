use iced::widget::{button, checkbox, column, container, row};
use iced::{Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    Box::run(Settings::default())
}

struct Box {
    debug: bool,
}

#[derive(Debug, Clone)]
enum Message {
    NextPage,
    PrevPage,
    DebugMode(bool),
}

impl Sandbox for Box {
    type Message = Message;

    fn new() -> Self {
        Self { debug: false }
    }

    fn title(&self) -> String {
        String::from("State - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DebugMode(b) => self.debug = b,
            _ => (),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            checkbox("Debug mode", self.debug, Message::DebugMode),
            row![
                button("Prev").on_press(Message::PrevPage),
                button("Next").on_press(Message::NextPage),
            ]
            .spacing(10)
        ]
        .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
