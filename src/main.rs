fn main() {
    println!("Hello, world!");
    let result: i8 = another(1);
    println!("Result: {}", result);
}

fn another(num: i8)  -> i8 {
    println!("Another function. {}", num);

    return num + 1;
}


