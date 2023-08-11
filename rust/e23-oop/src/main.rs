mod oop;
use oop::AveragedCollection;

mod gui;
use gui::{Screen, Button, SelectBox};

fn main() {
    let mut a = AveragedCollection::new();
    a.add(3);
    a.add(7);
    a.add(2);
    match a.remove(3) {
        Ok(v) => println!("Value: {:?}", v),
        Err(e) => println!("Error: {:?}", e),
    }
    println!("Average: {}", a.average());

    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 100,
                height: 100,
                label: "&Save".to_string(),
            }),
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec!["Yes".to_string(), "No".to_string()],
            }),
        ],
    };

    screen.run();
}
