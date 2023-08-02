use std::{fs::File, io::ErrorKind};

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
}
