use std::cell::RefCell;
pub fn test_return_str() {
    let d = Data { data: RefCell::new(String::from("hello")) };
    println!("{}", d.get1());
    println!("{}", d.get2());
}
struct Data { data: RefCell<String>, }
impl Data {
    fn get1(&self) -> String {
        self.data.borrow().to_string()
    }
    fn get2(&self) -> &String {
        &self.data.borrow().to_string()
    }
}