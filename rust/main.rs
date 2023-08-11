use std::cell::RefCell;
use std::cell::Ref;

fn main() {
    let mut p = Post::new();
    p.add_text(" world");
    println!("{}", p.content());
}

trait State {
    fn add_text<'a>(&self, post: &'a Post, text: &'a str) {}
    fn content<'a>(&self, post: &'a Post) -> Ref<'a, String>;
}
pub struct Post {
    state: Option<Box<dyn State>>,
    content: RefCell<String>,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: RefCell::new(String::from("hello")),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.state.as_ref().unwrap().add_text(self, text);
    }

    pub fn content(&self) -> Ref<'_, String> {
        self.state.as_ref().unwrap().content(&self)
    }
}

struct Draft {}
impl State for Draft {
    fn add_text<'a>(&self, post: &'a Post, text: &'a str) {
        post.content.borrow_mut().push_str(text);
    }
    fn content<'a>(&self, post: &'a Post) -> Ref<'a, String> {
        post.content.borrow()
    }
}
