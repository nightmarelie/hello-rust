use std::rc::Rc;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    // reference counting
    Cons(i32, Rc<List>),
    Nil,
}

// space is allocated on the heap for the values
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn example () {
    let b = Box::new(5);

    println!("b = {}", b);

    let a = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));

    let b = Cons(3, Rc::new(a));
    let c = Cons(4, Rc::new(a));

    println!("list = {:?}", a);


}
