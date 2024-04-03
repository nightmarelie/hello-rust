use std::ops::Deref;

// The Deref trait, provided by the standard library, requires us to implement one method named deref that borrows self and returns a reference to the inner data.
// By implementing the Deref trait on the MyBox type, we can call the dereference operator on an instance of MyBox, which will then in turn call the deref method.
// The reason the deref method returns a reference to a value, and that the standard library provides the Deref trait, is so the dereference operator can be used in an expression that needs a reference.

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
    hello(&(*m)[..]); // (*m) dereferences the MyBox<String> into a String, and & and [..] then take a string
                      // slice of the String that is equal to the whole string to match the signature of hello.

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    drop(c); // Explicitly drop a value early by calling the std::mem::drop function.

    println!("CustomSmartPointers created.");
}

fn hello(name: &str) {
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
