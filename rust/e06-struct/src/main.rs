fn main() {
    let u = User {
        username: String::from("Name"),
        email: String::from("test@mail.com"),
        age: 9,
    };
    println!("{}, {}, {}", u.username, u.email, u.age);
    let u2 = User {
        username: String::from("Name2"),
        email: u.email,
        age: 10,
    };
    println!("{:#?}", u2);

    // 元组定义风格
    #[derive(Debug)]
    struct Point(i32, i32, f64);
    let p = Point(12, 12, 15.8);
    println!("{:?}", p);
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: u8,
}