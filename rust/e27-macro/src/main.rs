fn main() {
    let v = ve![1, 2];
    println!("{:?}", v);
    let v = ve!(3, 4);
    println!("{:?}", v);
}

#[macro_export]
macro_rules! ve {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}