struct Config {}
fn main() {
    let _cfg = test().unwrap_or_else(|err| {
        println!("{}", err);
        never()
    });
}
fn test() -> Result<Config, &'static str> {
    Err("error")
}
fn never() -> ! {
    panic!("panic")
}
