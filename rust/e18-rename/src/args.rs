use std::path::PathBuf;
use std::env;

#[derive(Debug)]
pub struct CliArgs {
    pub path: PathBuf,
    pub old_str: String,
    pub new_str: String,
}

pub fn process_cmdline() -> Result<CliArgs, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        Err(String::from("process_cmdline"))
    } else {
        println!("{:?}", args);
        let r = CliArgs {
            path: PathBuf::from(args[1].as_str()),
            old_str: String::from(args[2].as_str()),
            new_str: String::from(args[3].as_str()),
        };
        Ok(r)
    }
}