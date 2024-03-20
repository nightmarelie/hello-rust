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

    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));

    let b = Rc::new(Cons(3, Rc::clone(&a)));
    let c = Rc::new(Cons(4, Rc::clone(&a)));

    println!("list = {:?}", a);


}
