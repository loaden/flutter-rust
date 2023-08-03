fn main() {
    other_test();
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

fn other_test() {
    let a = String::from("123");
    let mut r;
    {
        let b: &str = "4567";
        r = longest(a.as_str(), b);
    }
    println!("{}", r);
    {
        let b = String::from("4567");
        r = longest(a.as_str(), b.as_str());
    }
    println!("{}", r);
}