fn main() {
    let mut a: Option<&str> = Some("Hello, world");
    if let Some(a) = a {
        println!("{}", a);
    }
    a = None;
    match a {
        Some(s) => println!("The string is {}", s),
        None => println!("The string is None"),
    }

    let b: Result<u32, _> = "25".parse();
    match b {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e),
    }

    let v = vec![1, 2, 3, 4, 5, 6, 7];
    for ( i, v ) in v.iter().enumerate() {
        println!("{}: {}", i, v);
    }
    println!("{}", v.len());

    let (x, y, z) = (10, 20, 30);
    println!("x = {}, y = {}, z = {}", x, y, z);

    let (x, y, _) = (4, 5, 6);
    println!("x = {}, y = {}, z = {}", x, y, z);

    let (x, ..) = (7, 8, 9, 10);
    println!("x = {}, y = {}, z = {}", x, y, z);
}
