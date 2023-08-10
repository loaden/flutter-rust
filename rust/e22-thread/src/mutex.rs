use std::thread;
use std::time::Duration;
use std::sync::Mutex;

pub fn thread_mutex() {
    let m = Mutex::new(5);
    {
        println!("m: {:?}", m);
        let mut num = m.lock().unwrap();
        *num += 1;
        println!("m: {:?}", m);
        drop(num);
        println!("m: {:?}", m);
    }

    println!("m: {:?}", m);
}