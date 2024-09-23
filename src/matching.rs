enum Language {
    Rust,
    Go,
    Python,
    JavaScript,
}

pub fn example() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    let lang = Language::Rust;

    match lang {
        Language::Rust => println!("Rust is awesome!"),
        Language::Go => println!("Go is cool!"),
        Language::Python => println!("Python is great!"),
        Language::JavaScript => println!("JavaScript is fun!"),
        // _ => println!("Some other language!"), // This is not needed because we are matching all the variants. The _ pattern will match all values (catchall pattern).
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // condition matching
    let number = 13;

    match number {
        n if n < 5 => println!("{} is less than five", n),
        n if n > 5 => println!("{} is greater than five", n),
        _ => println!("{} is five", number),
    }

    // with some
    let some_number = Some(5);

    if let Some(n) = some_number {
        println!("n is {}", n);
    }

    // let statements
    let (x, y, _) = (1, 2, 3);

    print_coordinates(&(x, y));

    // refutable pattern and irrefutable pattern

    // refutable pattern
    let some_value: Option<i32> = Some(5);

    if let Some(value) = some_value {
        println!("value is {}", value);
    } else {
        println!("value is None");
    }

    // irrefutable pattern

    // let some_value: Option<i32> = Some(5);

    // Can only accept irrefutable patterns
    // function parameters, let statements, and for loops

    // multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    // matching ranges of values with ..=
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // the same but for characters
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn example_of_if_let_else_if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
