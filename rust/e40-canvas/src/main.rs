use iced::{Element, Sandbox, Settings};

fn main() -> iced::Result {
    Ewl::run(Settings::default())
}

struct Ewl;

impl Sandbox for Ewl {
    type Message = ();

    fn new() -> Self {
        Ewl
    }

    fn title(&self) -> String {
        String::from("Ewl - Iced")
    }

    fn update(&mut self, _: Self::Message) {}

    fn view(&self) -> Element<Self::Message> {
        "".into()
    }
}
