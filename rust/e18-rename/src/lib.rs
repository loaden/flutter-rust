pub mod io;
pub mod args;

use io::types;
use io::path::path;

pub fn test_mod() {
    path::get_files(&"path".to_string());
    let tf = types::TargetFile {
        new_name: String::new(),
        old_name: String::new(),
        renamed: false,
    };
}

#[cfg(test)]
mod tests {
    use args::Config;
    use super::*;

    #[test]
    fn tst_rename() {
        let cfg = Config::new(
            String::from("."),
            String::from("old"),
            String::from("new"),
        );

        let tf = types::TargetFile {
            new_name: String::new(),
            old_name: String::new(),
            renamed: false,
        };
    }
}
