use std::io::Read;
use std::process::Command;
use std::process::Stdio;

fn main() {
    if let Ok(mut child) = Command::new("clang")
        .arg("-v")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        let mut stderr = child.stderr.take().unwrap();
        let mut str = String::from("");
        let _ = stderr.read_to_string(&mut str);
        println!("ERROR: {:#?}", str);

        let output = child.wait_with_output().expect("failed to wait on child");
        if output.status.success() {
            let raw_output = String::from_utf8(output.stdout).unwrap();
            println!("OUTPUT: {:#?}", raw_output);
        }
    }

    if let Ok(output) = Command::new("git").arg("-v").output() {
        if output.status.success() {
            let raw_output = String::from_utf8(output.stdout).unwrap();
            println!("OUTPUT: {:#?}", raw_output);
        }
    }
}
