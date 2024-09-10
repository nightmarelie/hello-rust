mod closures;

use std::thread;
use std::time::Duration;
use closures::{Inventory, ShirtColor};

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);
    
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);
    
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        
        thread::sleep(Duration::from_secs(2));
        num
    };
    
    println!("{}", expensive_closure(13));
}
