enum IpAddrKind {
    V4(String),
    V6(String),
}

pub enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Speak(String),
    ChangeColor(i32, i32, i32),
}

pub fn example(command: Command) {
    // let four = IpAddrKind::V4(String::from("127.0.0.1"));
    // let six = IpAddrKind::V6(String::from("127.0.0.1"));

    // route(four);

    // example of option
    let x = 15;
    let y: Option<i8> = Some(5); // If it is None, it will use default value 0

    let sum =  x + y.unwrap_or(0); // this will not work because y is an Option
    println!("The result of sum is: {}", sum);

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