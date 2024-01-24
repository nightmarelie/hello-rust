fn main() {
    println!("Hello, world!");

    // wait for 3 sec
    std::thread::sleep(std::time::Duration::from_secs(3));

    let mut x:i32 = 5;
    println!("The value of x is: {} (first time)", x);

    x = 10;
    println!("The value of x is: {} (second time)", x);

    // shadowing
    let x = 11;
    println!("The value of x is: {} (third time, after shadowing)", x);

    const MAX_POINTER: u32 = 200_000_000;
    println!("The value of MAX_POINTER is {}", MAX_POINTER);

    // define a var and assign
    let x;
    x = 1;
    println!("The value of x is: {})", x);

    // DATA TYPES
    // Scalar types
    // - Integers
    // - Floating-point numbers
    // - Booleans
    // - Characters
    // Compound types
    // - Tuples
    // - Arrays
    // Custom types
    // - Structs
    // - Enums


}
