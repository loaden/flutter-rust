use std::cell::RefCell;
use std::rc::Rc;

pub fn weak_pointer_test() {
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
