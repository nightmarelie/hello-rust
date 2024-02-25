enum IpAddrKind {
    V4(String),
    V6(String),
}

// Option enum as defined in the standard library. This is a simplified version.
// The purpose of this enum is to show how to define an enum with a generic type.
enum Option<T> {
    Some(T),
    None,
}

pub enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Speak(String),
    ChangeColor(i32, i32, i32),
}

pub fn example(command: Command) {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("127.0.0.1"));

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
        IpAddrKind::V4(ip) => println!("Routing to IPv4 address: {}", ip),
        IpAddrKind::V6(ip) => println!("Routing to IPv6 address: {}", ip),
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