fn main() {
    let b = Bird {
        value: String::from("red"),
    };
    println!("{}", b.fly());
    println!("{}", b.get());
    notify(&b);

    let p = Plane {};
    println!("{}", p.fly());
    notify(&p);
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

struct Plane {}

impl<T> Gongfu for Bird<T> {
    fn fly(&self) -> String {
        format!("{}, {} it's me.", self.hello(), "Bird")
    }
}

impl Gongfu for Plane {
    fn fly(&self) -> String {
        format!("{}, {} is cool.", self.hello(), "Plane")
    }
}

impl<T> Bird<T> {
    fn get(&self) -> &T {
        &self.value
    }
}

fn notify(item: &impl Gongfu) {
    println!("Notify: {}", item.fly());
}
