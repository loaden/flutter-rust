fn main() {
    let mut s = String::from("hello");

    println!("fn main: {}", mut_string_borrow(&mut s));
    // let r = mut_string_param(&mut s);
    s.clear();
    // println!("fn main: {}", r);
}

fn mut_string_borrow(s: &mut String) -> &str {
    *s += " world";
    println!("fn mut_string_borrow: {}", s);
    &s[1..]
}