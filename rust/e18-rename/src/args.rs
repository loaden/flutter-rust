use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliArgs {
    path: PathBuf,
    old_str: String,
    new_str: String,
}

impl CliArgs {
    pub fn new(path: String, old_str: String, new_str: String) -> CliArgs {
        CliArgs {
            path: PathBuf::from(path),
            old_str: old_str,
            new_str: new_str,
        }
    }
}

pub fn process_cmdline() -> Result<CliArgs, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        Err("process_cmdline")
    } else {
        println!("{:?}", args);
        let r = CliArgs::new(
            args[1].clone(),
            args[2].clone(),
            args[3].clone()
        );
        Ok(r)
    }
}
