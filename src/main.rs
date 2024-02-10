fn main() {
    let hello = "Hello, world!";

    for c in hello.chars() {
        println!("{}", c);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }
}
