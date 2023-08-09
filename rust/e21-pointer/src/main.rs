mod r#box;
use crate::r#box::box_pointer_test;

mod rc;
use crate::rc::rc_pointer_test;

mod refcell;
mod mix;
use crate::mix::mixed::test_rc_refcell_pointer;

fn main() {
    box_pointer_test();
    rc_pointer_test();
    test_rc_refcell_pointer();
}
