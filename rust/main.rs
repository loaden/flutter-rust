fn main() {
    let p = Post::new();
    p.add_text(" world");
    println!("{}", p.content());
}

trait State {
    fn add_text<'a>(&self, post: &'a mut Post, text: &'a str) {}
    fn content<'a>(&self, post: &'a Post) -> &'a str;
}
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::from("hello"),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.state.as_mut().unwrap().add_text(self, text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
}

struct Draft {}
impl State for Draft {
    fn add_text<'a>(&self, post: &'a mut Post, text: &'a str) {
        post.content.push_str(text);
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
