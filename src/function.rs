fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { // usinf fn pointer as argument
    f(arg) + f(arg)
}

pub fn example() {
    // 1. Function
    // 1.1 Functions are declared using the fn keyword
    // 1.2 The return type of the function is declared after an arrow ->
    // 1.3 The return type can be omitted, in which case the return type is assumed to be ()
    // 1.4 The body of the function is enclosed in curly braces
    // 1.5 The last expression in the function is considered as return value
    // 1.6 The return keyword can be used to return early from a function
    // 1.7 The return value of the function is the value of the final expression in the block of the body of the function

    let answer = do_twice(add_one, 5);
    
    assert_eq!(answer, 12);
    
    println!("answer: {}", answer);
}
