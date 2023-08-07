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

    let mut r = Cacher::new(|i| {
        println!("Slowly3...");
        thread::sleep(Duration::from_secs(2));
        i
    });
    r.value(8);
    r.value(9);

    let mut mut_value = String::from("mut string");
    let f = |mut i: u32| -> String {
        i += 1;
        mut_value.push_str(&i.to_string());
        mut_value
    };
    // println!("{}", mut_value);
    let ret = f(8);
    println!("{}", ret);
}

fn simulated_calc(intensity: u32) -> u32 {
    println!("Slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    r: Option<u32>,
    closure_inside_struct: T,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(closure: T) -> Cacher<T> {
        Cacher { r: None, closure_inside_struct: closure }
    }

    fn value(&mut self, closure_arg: u32) -> u32 {
        match self.r {
            Some(v) => v,
            None => {
                let v = (self.closure_inside_struct)(closure_arg);
                self.r = Some(v);
                v
            },
        }
    }
}
