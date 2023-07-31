fn main() {
    let o = Some(6);
    let p: Option<String> = None;
    println!("{:?}, {:?}", o, p);

    match o {
        Option::None => {
            println!("None");
        },
        Option::Some(x) => {
            println!("{}", x);
        }
    };

    if let None = p {
        println!("if let");
    }
}
