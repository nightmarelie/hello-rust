struct Point {
    x: i32,
    y: i32,
}

// Enums

enum Color {
    Rgb(u8, u8, u8), // tuple
    Cmyk {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeColor2(Color),
}

enum Message2 {
    Hello { id: i32 },
}

pub fn example() {
    // Patterns

    // match arms
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let _y = 5;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // y is shadowed
        _ => println!("Default case, x = {:?}", x),
    }

    // Multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Range patterns
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; // a = 0, b = 7

    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // destructing enums
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor2(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor2(Color::Cmyk {
            cyan,
            magenta,
            yellow,
            black,
        }) => {
            println!(
                "Change the color to cyan {}, magenta {}, yellow {}, and black {}",
                cyan, magenta, yellow, black
            )
        }
    }

    // Ignoring values in a pattern
    let _numbers = (2, 4, 8, 16, 32);

    match _numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    let mut _settings_value = Some(5);
    let new_settings_value = Some(10);

    match (_settings_value, new_settings_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            _settings_value = new_settings_value;
        }
    }

    println!("new settings value is {:?}", _settings_value);

    // using range syntax for ignoring remaining parts of a value
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // match guards
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    
    // shadowing in match guards
    let x = Some(5);
    
    match x {
        Some(5) => println!("five"),
        Some(_) => println!("some other number"),
        None => (),
    }
    
    // assigning a value to a variable in a match guard
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // assigning a value using @ at operator
    
    let msg = Message2::Hello { id: 5 };
    
    match msg {
        Message2::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message2::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }   
}
