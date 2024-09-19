use std::sync::mpsc; // Stands for multiple producer, single consumer
use std::thread;
use std::time::Duration;

pub fn example() -> () {
    // introduce channel
    // tx stands for transmitter
    // rx stands for receiver
    let (tx, _rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hi");
        
        tx.send(val).unwrap();
    });
    
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
}
