pub fn example () -> i32 {
    // 1. Variables and Mutability
    // Define a variable
    // 1.1 This variable is immutable du default
    let x = 5;

    // 1.2 This is the way to make it mutable
    let mut y = 10;

    // in this way we can change the value of y
    y = 11;

    // 2. Constants
    // This is the way to define a constant
    // 2.1 Constants are always immutable, and you can't use mut with them
    // 2.2 Constants can be declared in any scope, including the global scope
    // 2.3 Constants mast be annotated with a type
    // 2.4 Constants can only be set to a constant expression, not the result of a function call or any other value that could only be computed at runtime
    const MAX_POINTS: u32 = 100_000;

    // 3. Shadowing
    // 3.1 We can declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable
    // 3.2 We can change the type of the value but reuse the same name
    let y = x + 1;
    let y = "hello!";

    assert_eq!(y, "hello!");

    // 4. Data Types
    // 4.1 Rust is a statically typed language, which means that it must know the types of all variables at compile time
    // 4.2 Two categories of data types: scalar and compound
    // 4.3 Scalar types represent a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters
    // 4.4 Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays

    return x;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(example(), 5);
    }
}
