fn main() {
    let p = Message::Point { x: 32, y: 66 };
    p.print();
    match p {
        Message::Point { x, y } => println!("{}, {}", x, y),
        _ => (),
    }
}

#[derive(Debug)]
enum Message {
    Point { x: i32, y: i32 },
    _Title(String),
}

impl Message {
    fn print(&self) {
        println!("{:#?}", self);
    }
}
