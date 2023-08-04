pub mod path {
    use crate::io::types::types::TargetFile;

    pub fn get_files(path: &String) {
        let t = TargetFile {
            old_name: String::from("old"),
            new_name: String::from("new"),
            renamed: false,
        };
    }

    pub fn rename(old: &String, new: &String) {
        println!("{} - {}", old, new);
    }
}
