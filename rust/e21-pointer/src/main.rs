use self::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    let r: List<u32> = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:#?}", r);
    let r: List<f64> = Cons(1.12, Box::new(Cons(2.23, Box::new(Nil))));
    println!("{:#?}", r);

    let r = MyBox::new(String::from("hello"));
    println!("{}", *r);

    hello(&r);
    let r = SmartPointer::new(String::from("happy"));
    println!("{}", *r);
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

struct SmartPointer {
    x: String,
}

impl SmartPointer {
    fn new(x: String) -> SmartPointer {
        SmartPointer { x }
    }
}

impl Deref for SmartPointer {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.x
    }
}

impl Drop for SmartPointer {
    fn drop(&mut self) {
        println!("drop MyBox: {}", &self.x);
    }
}

fn hello(s: &str) {
    println!("{}", s);
}