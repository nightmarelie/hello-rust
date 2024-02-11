use std::collections::HashMap;
mod boolean;
mod hash_map;

fn main() {
    println!("The result of boolean example(true, true) is: {}", boolean::example(true, true));

    let mut numbers: HashMap<i32, i32> = HashMap::from([(1,1)]);
    numbers = hash_map::example(numbers);

    println!("The result of hash_map example is: {:?}", numbers);
}
