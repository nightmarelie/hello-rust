enum IpAddrKind {
    V4,
    V6,
}

pub enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Speak(String),
    ChangeColor(i32, i32, i32),
}

pub fn example(command: Command) {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);

    match command {
        Command::Quit => println!("Quitting"),
        Command::Move { x, y } => println!("Moving to x: {}, y: {}", x, y),
        Command::Speak(s) => println!("Speaking: {}", s),
        Command::ChangeColor(r, g, b) => println!("Changing color to red: {}, green: {}, blue: {}", r, g, b),
    }
}

fn route(ip_type: IpAddrKind) {
    match ip_type {
        IpAddrKind::V4 => println!("V4"),
        IpAddrKind::V6 => println!("V6"),
    }
}

// write test for example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        example(Command::Quit);
        example(Command::Move { x: 10, y: 20 });
        example(Command::Speak("Hello".to_string()));
        example(Command::ChangeColor(255, 255, 255));
    }
}