mod example;
mod foo;

use foo::{hello::module_hello, test};
use example::module_example;

fn main() {
    module_hello::call_module_hello();
    test::call_module_test();
    module_example::call_module_example();
}
