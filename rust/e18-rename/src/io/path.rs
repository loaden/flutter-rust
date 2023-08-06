pub mod path {
    use crate::types;
    use crate::args;

    pub fn get_files(path: &String) {
        let t = types::TargetFile {
            old_name: String::from("old"),
            new_name: String::from("new"),
            renamed: false,
        };
    }

    pub fn rename<'a>(cfg: &args::Config, tf: &'a mut types::TargetFile) -> &'a types::TargetFile {
        println!("{:?}", cfg);
        tf.new_name = tf.old_name.replace(&cfg.old_str, &cfg.new_str);
        tf.renamed = true;
        tf
    }
}
