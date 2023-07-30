use std::io;

fn main() {
    let mut s = String::new();
    println!("请输入一个字符串：");
    io::stdin().read_line(&mut s).expect("failed");
    s += "test";
    println!("输出：{}", s.to_uppercase());
    let 中文变量 = String::from("我操");
    println!("{}", 中文变量);
}
