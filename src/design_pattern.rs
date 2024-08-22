pub struct Post {
    state: Option<Box<dyn State>>,
}

trait State {}

struct Draft {}

impl State for Draft {}
