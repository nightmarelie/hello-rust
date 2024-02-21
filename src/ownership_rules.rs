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

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid. It's called a move, not a copy. It's done to avoid double free error

    println!("{}", s2); // s2 is valid
    // println!("{}", s1); // s1 is not valid

    let s1 = s2.clone(); // s2 is cloned to s1, both are valid
    println!("{}", s1); // s1 is valid

    // 2. Ownership and Functions
}