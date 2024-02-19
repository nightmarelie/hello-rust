use std::io;
use std::cmp::Ordering;
use rand::{thread_rng, Rng};

pub fn run () {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..102);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess =  String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

    let result = guess.trim().parse::<i32>().expect("Can't parse the guess!").cmp(&secret_number);

    assert_eq!(result, Ordering::Equal);

    println!("The result is: {:?}", result);
}