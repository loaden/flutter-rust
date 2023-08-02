use std::cmp::PartialOrd;

fn main() {
    let a = 32;
    let b = 35;
    println!("{}", bigger_test(&a, &b));
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 1.0, y: 3.5 };
}

fn bigger_test<T>(x: &T, y: &T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

impl<T> PartialOrd for bigger_test<T> {}

struct Point<T> {
    x: T,
    y: T,
}
