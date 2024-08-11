use std::cell::RefCell;
use std::rc::{Rc, Weak};
use List::{Cons, Nil};
use BoxList::{Cons, Nil};

// Smart pointers are data structures that not only act like a pointer but also have additional metadata and capabilities.
// The smart pointer pattern is a general programming pattern that you can implement in Rust using trait implementations.
// Smart pointers are usually implemented using structs. The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the Deref and Drop traits.
// The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers.
// The Drop trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope.
// Smart pointers are usually used for implementing reference counting or interior mutability patterns.
// The Rc<T> type keeps track of the number of references to a value which determines whether or not a value is still in use.
// The RefCell<T> type represents single ownership over the data it holds, but it allows mutable borrows checked at runtime.

#[derive(Debug)]
enum List {
    // reference counting
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

enum BoxList {
    Cons(i32, Box<BoxList>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// space is allocated on the heap for the values
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn example() {
    let list = Cons(1, Box::new(Const(2, Box::new(Nil))));

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // example of coercion

    let b = Box::new(5);

    println!("b = {}", b);

    let a = Rc::new(Cons(1, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    println!("list = {:?}", a);

    let b = Rc::new(Cons(2, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&Rc::clone(&branch));

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    #[test]
    fn it_sends_an_over_90_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(91);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // Associated type

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drom for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox with data {}", self.0);
    }
}

fn hello(name: &str) -> () {
    println!("Hello, {name}");
}
