use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    path: PathBuf,
    old_str: String,
    new_str: String,
}

impl Config {
    pub fn new(path: String, old_str: String, new_str: String) -> Config {
        Config {
            path: PathBuf::from(path),
            old_str: old_str,
            new_str: new_str,
        }
    }
}

pub fn process_cmdline() -> Result<Config, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        Err("process_cmdline")
    } else {
        println!("{:?}", args);
        let r = Config::new(
            args[1].clone(),
            args[2].clone(),
            args[3].clone()
        );
        Ok(r)
    }
}
