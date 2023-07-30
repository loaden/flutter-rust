fn main() {
    let s1 = String::from("value");
    let s2 = s1;
    println!("{}", takes_ownership(s2));
    // println!("{}", s1);
    println!("{}", s2);
}

fn takes_ownership(mut s: String) ->usize {
    s.push('A');
    println!("takes_ownership: {}", s);
    s.capacity()
}
