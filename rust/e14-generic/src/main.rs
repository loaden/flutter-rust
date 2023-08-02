use std::{cmp::PartialOrd, fmt::Display};

fn main() {
    let a = 32;
    let b = 35;
    println!("{}", bigger_test(a, b));
    let c = String::from("cccc");
    let d = String::from("dddd");
    println!("{}", bigger_test(c, d));

    let _p1 = Point { x: 10, y: 20 };
    let _p2 = Point { x: 1.0, y: 3.5 };
}

fn bigger_test<T: PartialOrd + Display>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

struct Point<T> {
    x: T,
    y: T,
}
