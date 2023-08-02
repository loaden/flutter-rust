fn main() {
    let b = Bird { value: String::from("red"), };
    println!("{}", b.fly());
}

pub trait Gongfu {
    fn hello(&self) -> String {
        String::from("hi")
    }
    fn fly(&self) -> String {
        format!("{}, {}", self.hello(), String::from("I CAN FLY."))
    }
}

struct Bird<T> {
    value: T,
}

impl<T> Gongfu for Bird<T> {
    fn fly(&self) -> String {
        format!("{}, {} it's me.", self.hello(), "self.value")
    }
}
