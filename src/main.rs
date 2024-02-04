fn main() {
    let number = 3;

    // if-else in rust is expression
    let condition  = if number < 5 {
        "Number is less than 3";
    } else {
        "Number is greater than or equal to 3";
    };

    println!("The number is: {}", number);
}
