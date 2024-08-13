use std::thread;
use std::time::Duration;

pub fn example() -> () {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));    
        }
    });
    
    handle.join().unwrap();
    
    for i in 1..5 {
        println!("hi number {i} from main thread!");
        thread::sleep(Duration::from_millis(1));
    } 
}