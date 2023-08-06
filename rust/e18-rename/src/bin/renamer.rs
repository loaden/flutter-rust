use renamer as app;

fn main() {
    let a = app::add(1, 2);
    println!("add: {}", a);

    let r = app::args::process_cmdline();
    match r {
        Ok(args) => println!("args: {:?}", args),
        Err(err) => println!("args: {}", err),
    }

    // 需要返回CliArgs
    // let r = app::args::process_cmdline().unwrap_or_else(|err| {
    //     println!("{}", err);
    // });
}
