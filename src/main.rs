use std::io;

fn main() {
    let mut s = String::new();
    println!("请输入一个字符串：");
    io::stdin().read_line(&mut s).expect("failed");
    println!("输出：{}", s.to_uppercase());
}
