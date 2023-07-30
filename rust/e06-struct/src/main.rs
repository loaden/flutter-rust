fn main() {
    let u = User {
        username: String::from("Name"),
        email: String::from("test@mail.com"),
        age: 9,
    };
    println!("{}, {}, {}", u.username, u.email, u.age);
}

struct User {
    username: String,
    email: String,
    age: u8,
}