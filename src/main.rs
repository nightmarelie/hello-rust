fn main() {
    println!("Hello, world!");

    // wait for 3 sec
    std::thread::sleep(std::time::Duration::from_secs(3));

    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 10;
    println!("The value of x is: {}", x);
}
