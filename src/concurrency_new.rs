use std::sync::mpsc;
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

    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        println!("here's a vector: {v:?}");
    });
    
    handle.join().unwrap();
    
    // introduce channel
    let (tx, rx) = mpsc::channel();
}
