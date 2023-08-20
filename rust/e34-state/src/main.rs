use iced::{Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Box::run(Settings::default())
}

struct Box;

#[derive(Debug)]
enum Message {}

impl Sandbox for Box {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("State - Iced")
    }

    fn update(&mut self, _message: Message) {
        todo!()
    }

    fn view(&self) -> Element<Message> {
        "".into()
    }
}
