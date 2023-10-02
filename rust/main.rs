use std::fs::OpenOptions;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let filename = "test.txt";
    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .create(true)
        .open(filename)?;

    file.write_all(b"test")?;
    file.flush()?;

    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(n) => println!("f.read_to_string: {}, {}", buf, n),
        Err(e) => println!("{}", e),
    };

    Ok(())
}
