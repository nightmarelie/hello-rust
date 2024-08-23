pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

trait State {}

struct Draft {}

impl State for Draft {}

impl Post {
    fn new() -> Post {
        Post { 
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

pub fn example() {
    let mut post = Post::new();
}
