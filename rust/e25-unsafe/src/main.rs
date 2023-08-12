mod raw_pointer;
use raw_pointer::raw_pointer_test;

mod state;
use crate::state::Post;

mod r#trait;
use crate::r#trait::{Point, Human, Pilot, Wizard, Dog, Animal, OutlinePrint};
use crate::r#trait::{Millimeters, Meters};

fn main() {
    let mut p = Post::new();
    p.add_text(" world");
    println!("{}", p.content());

    raw_pointer_test();

    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
    p3.outline_print();

    let mm = Millimeters(1);
    let m = Meters(1);
    let nm = mm + m;
    println!("{}", nm.0);

    let h = Human;
    h.fly();
    Pilot::fly(&h);
    Wizard::fly(&h);

    println!("{}", Dog::name());
    println!("{}", <Dog as Animal>::name());
}
