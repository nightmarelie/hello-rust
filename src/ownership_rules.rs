pub fn example () {
    // 1. Ownership rules
    // 1.1. Each value in Rust has a variable that’s called its owner.
    // 1.2. There can only be one owner at a time.
    // 1.3. When the owner goes out of scope, the value will be dropped.

    // example
    { // s is not valid here, it’s not yet declared
        let s = String::from("hello"); // s is valid from this point forward. This variable is the owner of the String. It will be stored in the heap, literal &str "hello" will be stored in the stack
        // for types stored in the heap, the variable stores the pointer to the heap, the length and the capacity. Allocation and deallocation is done automatically
        // do stuff with s
        println!("{}", s);
    } // this scope is now over, and s is no longer valid

    // example with variables
    let x = 5;
    let y = x; // x is copied to y, both are valid
    // all types stored in the stack are copied, because they are stored in the stack
    // these types are: integers, floats, booleans, characters, tuples (if they contain only types stored in the stack)
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid. It's called a move, not a copy. It's done to avoid double free error

    println!("{}", s2); // s2 is valid
    // println!("{}", s1); // s1 is not valid

    let s1 = s2.clone(); // s2 is cloned to s1, both are valid
    println!("{}", s1); // s1 is valid

    // 2. Ownership and Functions
    // 2.1 example
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function and so is no longer valid here
    // println!("{}", s); // s is not valid here

    // 2.2 example
    // gives_ownership will move its return value into s1
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    println!("{}", s1); // s1 is valid

    // 2.3 example
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back and moves its return value into s3
    println!("{}", s3); // s3 is valid
    // println!("{}", s2); // s2 is not valid

    // 3. Return Values and Scope
    // 3.1 example
    let s = String::from("hello");
    let len = calculate_length(&s); // s is borrowed, so it's still valid
    println!("The length of '{}' is {}.", s, len); // s is still valid

    // 3.2 example
    let mut s = String::from("hello");
    change(&mut s); // s is borrowed, so it's still valid
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
} // Here, a_string goes out of scope and `drop` is called. The backing memory is freed.

fn calculate_length(s: &String) -> usize { // s comes into scope and is borrowed. It's immutable by default
    let length = s.len(); // length is the length of the string
    length // length is returned and moves out to the calling function
} // Here, s goes out of scope and `drop` is called. The backing memory is freed.

fn change(s: &mut String)  { // s comes into scope and is borrowed. It's mutable
    s.push_str(", world"); // s is changed
} // Here, s goes out of scope and `drop` is called. The backing memory is freed.