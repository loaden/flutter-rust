use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert("Blue", 10);
    m.insert("Red", 30);
    println!("{:?}", m);

    let i = m.get("Gray");
    match i {
        Some(n) => println!("{}", n),
        None => (),
    }

    let text = "hello Red world test Red world Blue";
    for i in text.split_whitespace() {
        let count = m.entry(i).or_insert(0);
        *count += 1;
    }

    for i in &mut m {
        println!("{}, {}", i.0, i.1);
        match i {
            (x, y) => println!("match: {}, {}", x, y),
        }
    }
}
