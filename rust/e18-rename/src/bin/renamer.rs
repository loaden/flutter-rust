use renamer as app;

fn main() {
    let a = app::add(1, 2);
    println!("add: {}", a);

    let r = app::args::process_cmdline();
    match r {
        Ok(_args) => (),
        Err(err) => println!("args: {}", err),
    }
}