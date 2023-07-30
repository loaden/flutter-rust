fn main() {
    let mut s = String::from("hello");
    println!("{}", &s[1..]);
    println!("{}", test_slice(&s));

    // s.clear();
    let r = string_borrow(&s);
    println!("main: {}", r);

    let r = string_borrow_mut(&mut s);
    // s.clear();
    println!("main: {}", r);

    s.clear();
}

fn string_borrow_mut(s: &mut String) -> &str {
    *s += " world";
    println!("string_borrow_mut: {}", s);
    &s[1..]
}

fn string_borrow(s: &String) -> &str {
    println!("string_borrow: {}", *s);
    &s[2..4]
}

fn test_slice(s: &String) -> String {
    s[..3].to_string()
}
