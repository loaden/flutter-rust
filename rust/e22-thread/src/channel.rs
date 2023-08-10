use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn thread_channel() {
    let (tx, rx) = mpsc::channel();

    let h = thread::spawn(move || {
        let msg = "hi".to_string();
        thread::sleep(Duration::from_secs(1));
        tx.send(msg).unwrap();
        thread::sleep(Duration::from_secs(2));
    });

    println!("begin receiving");
    let msg = rx.recv().unwrap();
    println!("end receiving");
    println!("Got: {}", msg);

    h.join().unwrap();
    println!("done");
}