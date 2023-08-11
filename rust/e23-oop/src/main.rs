mod oop;
use oop::AveragedCollection;


fn main() {
    let mut a = AveragedCollection::new();
    a.add(3);
    a.add(7);
    a.add(2);
    match a.remove(3) {
        Ok(v) => println!("Value: {:?}", v),
        Err(e) => println!("Error: {:?}", e),
    }
    println!("Average: {}", a.average());
}
