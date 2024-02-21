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

    // 2. Ownership and Functions
}