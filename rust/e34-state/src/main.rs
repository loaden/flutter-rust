use iced::widget::{button, checkbox, column, container, row};
use iced::{Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    Box::run(Settings::default())
}

struct Box {
    debug: bool,
    page: Page,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    NextPage,
    PrevPage,
    DebugMode(bool),
}

#[derive(PartialEq)]
enum Page {
    Welcome,
    CheckBox,
    End,
}

impl Sandbox for Box {
    type Message = Message;

    fn new() -> Self {
        Self {
            debug: false,
            page: Page::Welcome,
        }
    }

    fn title(&self) -> String {
        String::from("State - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DebugMode(b) => self.debug = b,
            Message::NextPage => self.page = Page::CheckBox,
            Message::PrevPage => self.page = Page::Welcome,
        }
    }

    fn view(&self) -> Element<Message> {
        let mut content = match self.page {
            Page::Welcome => column!["Welcome!"],
            Page::CheckBox => column![checkbox("Debug mode", self.debug, Message::DebugMode),],
            Page::End => column!["End"],
        };

        content = match self.page {
            Page::Welcome => content.push(button("Next").on_press(Message::NextPage)),
            Page::End => content.push(button("Prev").on_press(Message::PrevPage)),
            _ => content.push(
                row![
                    button("Prev").on_press(Message::PrevPage),
                    button("Next").on_press(Message::NextPage),
                ]
                .spacing(10),
            ),
        };

        container(content.spacing(20))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
