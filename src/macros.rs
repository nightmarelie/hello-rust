pub fn example() {
    // 1. Macros
    // 1.1 Macros are a way to generate code using metaprogramming
    // 1.2 Macros are invoked using the ! operator
    // 1.3 Macros can take a variable number of arguments
    // 1.4 Macros can be overloaded
    // 1.5 Macros can be used to define new syntax
    // 1.6 Macros can be used to reduce boilerplate code
    // 1.7 Macros can be used to generate tests
    // 1.8 Macros can be used to generate code at compile time
    // 1.9 Macros can be used to generate code at runtime
    // 1.10 Macros can be used to generate code for different platforms
    // 1.11 There are two types of macros: declarative macros and procedural macros
    // 1.12 Declarative macros are also known as macros by example
    // 1.13 Procedural macros are also known as custom derive macros
    // 1.14 Macros can be used to generate code for different architectures
    
    let v = vec![1, 2, 3];
    println!("v: {:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    println!("v: {:?}", v);
}

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub trait HelloMacro {
    fn hello_macro();
}

use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}
