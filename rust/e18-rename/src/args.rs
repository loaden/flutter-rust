use std::path::PathBuf;

pub struct CliArgs {
    pub cfg: PathBuf,
}

pub fn process_cmdline() -> Result<CliArgs, String> {
    Err(String::from("process_cmdline"))
}