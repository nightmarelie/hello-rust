pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

trait State {}

struct Draft {}

impl State for Draft {}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    pub fn content(&self) -> &String {
        &self.content
    }
}

pub fn example() {
    let mut post = Post::new();
    
    post.add_text("I ate a salad for lunch today");
    
    assert_eq!("I ate a salad for lunch today", post.content())
}
