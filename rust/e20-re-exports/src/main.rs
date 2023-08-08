use art::PrimaryColor;
use art::SecondaryColor;
use art::mix;

mod quux {
    pub use self::foo::{bar, baz};
    // pub use foo::{bar, baz};

    pub mod foo {
        pub fn bar() {}
        pub fn baz() {}
    }
}

fn main() {
    quux::bar();
    quux::baz();
    let r = mix(PrimaryColor::Blue, PrimaryColor::Red);
    println!("{:#?}", r);
    assert_eq!(SecondaryColor::Green, r);
}