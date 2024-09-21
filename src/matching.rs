enum Language {
    Rust,
    Go,
    Python,
    JavaScript,
}

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
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

    // destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
    }

    // short cut
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // enum matching
    // let msg = Message::ChangeColor(0, 160, 255);
    // 
    // match msg {
    //     Message::Quit => {
    //         println!("The Quit variant has no data to destructure.");
    //     }
    //     Message::Move { x, y } => {
    //         println!("Move in the x direction {x} and in the y direction {y}");
    //     }
    //     Message::Write(text) => {
    //         println!("Text message: {text}");
    //     }
    //     Message::ChangeColor(r, g, b) => {
    //         println!("Change the color to red {r}, green {g}, and blue {b}")
    //     }
    // }

    // nested structs
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    // destructuring structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // ignoring a few values
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // ignoring remaining parts of values with ..
    struct Point1 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point1 { x: 0, y: 0, z: 0 };

    match origin {
        Point1 { x, .. } => println!("x is {x}"),
    }

    // extra conditionals with match guards
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    // bindings
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}

// ignoring entire value
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn main() {
    foo(3, 4);
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
