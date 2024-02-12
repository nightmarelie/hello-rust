use std::collections::HashMap;
mod boolean;
mod hash_map;
mod vec;

fn main() {
    // Bool example
    println!("The result of boolean example(true, true) is: {}", boolean::example(true, true));

    // HashMap example
    let mut numbers: HashMap<i32, i32> = HashMap::from([(1,1)]);
    numbers = hash_map::example(numbers);

    println!("The result of hash_map example is: {:?}", numbers);
    println!("The result of hash_map example2 is: {:?}", hash_map::example2(HashMap::new()));

    // Vec example
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    numbers = vec::example(numbers);
    println!("The result of vec example is: {:?}", numbers)
}
