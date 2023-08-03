fn main() {
    let a = String::from("abcd");      // ---------+- 'a
    let b = "xyz";                       // -+- 'b   |
                                               //  |       |
    let r = longest(a.as_str(), b); // -|-------|-------+- 'r
    println!("{}", r);                         //  |       |       |
}                                              // -+-------+-------+-

fn longest<'c>(x: &'c str, y: &'c str) -> &'c str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}