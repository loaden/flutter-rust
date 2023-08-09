use self::List::{Cons, Nil};

fn main() {
    let r: List<u32> = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:#?}", r);
    let r: List<f64> = Cons(1.12, Box::new(Cons(2.23, Box::new(Nil))));
    println!("{:#?}", r);
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
