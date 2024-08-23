pub struct Post {
    state: Option<Box<dyn State>>,
}

trait State {}

struct Draft {}

impl State for Draft {}

impl Post {
    fn new() -> Post {
        Post { state: None }
    }
}

pub fn example() {
    let mut post = Post::new();
}
