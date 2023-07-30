fn main() {
    let q = Message::Quit;
    let p = Message::Point { x: 32, y: 66 };
    let t = Message::Title(String::from("title"));
    let c = Message::Color(255, 0, 0);
    print_enum(q);
    print_enum(p);
    print_enum(t);
    print_enum(c);
}

#[derive(Debug)]
enum Message {
    Quit,
    Point {x: i32, y: i32},
    Title(String),
    Color(i32, i32, i32)
}

fn print_enum(m: Message) -> Message {
    println!("{:?}", m);
    m
}