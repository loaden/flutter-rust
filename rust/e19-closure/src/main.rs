use std::thread;
use std::time::Duration;

fn main() {
    let i = 3;
    simulated_calc(i);
    println!("Hello, world!");

    let closure_simulated_calc = |i| {
        println!("Slowly2...");
        thread::sleep(Duration::from_secs(2));
        String::from(i)
    };
    closure_simulated_calc("haha");
}

fn simulated_calc(intensity: u32) -> u32 {
    println!("Slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
