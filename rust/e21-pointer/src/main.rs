mod r#box;
use crate::r#box::box_pointer_test;

mod rc;
use crate::rc::rc_pointer_test;

fn main() {
    box_pointer_test();
    rc_pointer_test();
}
