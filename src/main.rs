fn main() {
    let number = 15;

    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);

        if number > 10 {
            println!("{} is greater than 10", number);
        } else {
            println!("{} is less than 10", number);
        }
    }
}
