use std::sync::{mpsc, Mutex}; // Stands for multiple producer, single consumer
use std::thread;
use std::time::Duration;

pub fn example() -> () {
    
    let counter = Mutex::new(0);
    let mut handles = vec![];
    
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            
            *num += 1;
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());


    // introduce channel
    // tx stands for transmitter
    // rx stands for receiver
    // let (tx, rx) = mpsc::channel();
    // 
    // thread::spawn(move || {
    //     let val = String::from("hi");
    // 
    //     tx.send(val).unwrap();
    // });
    // 
    // let received = rx.recv().unwrap();
    // println!("Got: {received}");
    // 
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // 
    // handle.join().unwrap();
    // 
    // for i in 1..5 {
    //     println!("hi number {i} from main thread!");
    //     thread::sleep(Duration::from_millis(1));
    // }
    // 
    // let v = vec![1, 2, 3];
    // 
    // let handle = thread::spawn(move || {
    //     println!("here's a vector: {v:?}");
    // });
    // 
    // handle.join().unwrap();
    // 
    // let (tx, rx) = mpsc::channel();
    // 
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    // 
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // 
    // for received in rx {
    //     println!("Got: {received}");
    // }
    // let (tx, rx) = mpsc::channel();
    // 
    // let tx1 = tx.clone();
    // 
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    // 
    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // 
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];
    // 
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // 
    // for received in rx {
    //     println!("Got: {received}");
    // }
    // 
    // // Mutex
    // 
    // let m = Mutex:: new(4);
    // 
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // 
    // println!("m = {m:?}");
}
