fn main() {
    let number1 = 15;
    let number2= 4;

    if number1 > number2 && number1 % number2 == 0 {
        println!("{} is greater than {} and divisible by {}", number1, number2, number2);
    } else if number1 > number2 {
        println!("{} is greater than {}", number1, number2);
    } else if number1 == number2 {
        println!("{} is equal to {}", number1, number2);
    } else {
        println!("{} is less than {}", number1, number2);
    }
}
