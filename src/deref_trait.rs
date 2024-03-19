use std::ops::Deref;

pub fn example() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // that mean dereference y. It will follow the reference to the value it’s pointing to.

    let x = 5;
    let y = Box::new(x); // Box<T> is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references.

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // &MyBox<String> -> &String -> &str
    hello(&(*m)[..]); // (*m) dereferences the MyBox<String> into a String, and & and [..] then take a string slice of the String that is equal to the whole string to match the signature of hello.
}

fn hello (name: &str) {
    println!("Hello, {}!", name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref_trait() {
        example();
    }
}