enum Language {
    Rust,
    Go,
    Python,
    JavaScript,
}

pub fn example () {
    let lang = Language::Rust;

    match lang {
        Language::Rust => println!("Rust is awesome!"),
        Language::Go => println!("Go is cool!"),
        Language::Python => println!("Python is great!"),
        Language::JavaScript => println!("JavaScript is fun!"),
        // _ => println!("Some other language!"), // This is not needed because we are matching all the variants. The _ pattern will match all values (catchall pattern).
    }

    let mut stack = vec![];

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

    print_coordinates(º&(x, y));
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
