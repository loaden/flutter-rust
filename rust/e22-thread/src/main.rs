use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..=5 {
            thread::sleep(Duration::from_secs(1));
            println!("thread: {}", i);
        }
    });

    for i in 10..15 {
        thread::sleep(Duration::from_secs(1));
        println!("main: {}", i);
    }
}
