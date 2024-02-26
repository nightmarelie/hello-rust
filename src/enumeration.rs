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
    // let x = 15;
    // let y: Option<i8> = Some(5); // If it is None, it will use default value 0
    //
    // let sum =  x + y.unwrap_or(0); // this will not work because y is an Option
    // println!("The result of sum is: {}", sum);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("The result of six is: {:?}", six);
    println!("The result of none is: {:?}", none);


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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(some_value: Option<i32>) -> Option<i32> {
    match some_value {
        Some(3) => println!("three"),
        Some(i) => Some(i + 1),
        _ => None,
    }

    if let Some(i) = some_value {
        Some(i + 1)
    } else {
        None
    }
}