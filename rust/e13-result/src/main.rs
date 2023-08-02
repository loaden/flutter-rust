use std::{fs::File, io::ErrorKind};
use std::io;
use std::io::Read;
use std::fs;

fn main() {
    let filename = "test.txt";
    let _f = match File::open(filename) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(file) => file,
                Err(err) => panic!("create file failed: {:?}", err),
            },
            ErrorKind::Other => panic!("what happened"),
            _ => panic!("??"),
        },
    };

    let _f = File::open(filename).unwrap();
    let _f = File::open(filename).expect("msg");

    match read_file() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Err: {}", e),
    }

    match test_report() {
        Ok(_f) => (),
        Err(e) => println!("Err: {}", e),
    }

    match fs::read_to_string(filename) {
        Ok(s) => println!("read: {}", s),
        Err(e) => println!("read error: {}", e),
    }

    let _ = fs::remove_file(filename);
}

fn read_file() -> Result<String, io::Error> {
    let mut f = File::open("non-exist")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn test_report() -> Result<File, io::Error> {
    let f = File::open("report.txt")?;
    Ok(f)
}