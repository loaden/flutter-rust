fn main() {
    other_test();
    let a = String::from("abcd");   // ---------+- 'a
    let b = String::from("xyz");    // -+- 'b   |
                                            //  |       |
    let r = longest(&a, &b);  // -|-------|-------+- 'r
    println!("{}", r);                      //  |       |       |
}                                           // -+-------+-------+-

fn longest<'d>(x: &'d String, y: &'d String) -> &'d String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'c>(x: &'c str, y: &'c str) -> &'c str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn other_test() {
    let a = String::from("123");
    let r: &str;
    {
        let b = "4567";
        r = longest2(a.as_str(), b);
    }
    println!("{}", r);
    // {
    //     let b = String::from("4567");
    //     r = longest2(a.as_str(), b.as_str());
    // }
    // println!("{}", r);
}