pub fn example() {
    // 1. Control Flow
    // 1.1 If expression
    // 1.2 If-else expression
    // 1.3 If-else-if expression
    let y = 10;
    let x = if (y < 10) { 5 } else { 6 };

    println!("The result of control_flow example is: {}", x);

    // 1.4 Loop
    // 1.5 While
    // 1.6 For
    for n in 1..10 {
        println!("The result of control_flow example is: {}", n);
    }

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        println!("The result of control_flow example is: {}", name);
    }

    let mut number = 0;
    loop {
        number += 1;

        if number == 10 {
            break;
        }
    }

    // 1.8 Nested loops & labels
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }

        println!("This point will never be reached");
    }

    // 1.9 Some
    let some_number = Some(5);

    // 1.10 Match pattern and assign variable
    if let Some(5) = some_number {
        println!("The number is 5");
    }

    // 1.11 If is some
    if some_number.is_some() {
        println!("The number is {}", some_number.unwrap());
    }

    // 1.12 Match
    match some_number {
        Some(i) => println!("The number is {}", i),
        _ => println!("No value."),
    }
}
