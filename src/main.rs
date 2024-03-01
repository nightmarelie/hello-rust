mod largest;
mod generics;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest::run(number_list);

    println!("The largest number is {}", result);

    generics::run();
}
