// use std::collections::HashMap;
// use std::collections::HashSet;
// mod boolean;
// mod hash_map;
// mod vec;
//
// mod hash_set;
// mod structure;
// mod guessing_game;
mod variable;

fn main() {
    // // Bool example
    // println!("The result of boolean example(true, true) is: {}", boolean::example(true, true));
    //
    // // HashMap example
    // let mut numbers: HashMap<i32, i32> = HashMap::from([(1,1)]);
    // numbers = hash_map::example(numbers);
    //
    // println!("The result of hash_map example is: {:?}", numbers);
    // println!("The result of hash_map example2 is: {:?}", hash_map::example2(HashMap::new()));
    //
    // // Vec example
    // let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // numbers = vec::example(numbers);
    // println!("The result of vec example is: {:?}", numbers);
    //
    // // HashSet example
    // let mut numbers = HashSet::new();
    // numbers.insert(1);
    // numbers.insert(2);
    // println!("The result of hash_set example is: {:?}", hash_set::example(numbers));
    //
    // // Structure example
    // let game = structure::example();
    // println!("The result of structure example is: {:?}", game);
    // // read all properties
    // for player in game.players {
    //     println!("Player name: {}", player.name);
    //     println!("Player score: {}", player.score);
    // }

    // Guessing game
    // guessing_game::run();

    // Variable example
    let result = variable::example();
    println!("The result of variable example is: {}", result);
}
