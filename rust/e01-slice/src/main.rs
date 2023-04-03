fn main() {
    let mut s = String::from("hello");

    // println!("fn main: {}", mut_string_borrow(&mut s));
    let r = mut_string_borrow(&mut s);
    println!("fn main: {}", r);

    let r = string_borrow(&s);
    // s.clear();
    println!("fn main: {}", r);
}

fn mut_string_borrow(s: &mut String) -> &str {
    *s += " world";
    println!("fn mut_string_borrow: {}", s);
    &s[1..]
}

fn string_borrow(s: &String) -> &str {
    println!("fn string_borrow: {}", *s);
    &s[2..]
}