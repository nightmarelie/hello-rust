use std::thread;
use std::time::Duration;
use std::sync::mpsc; // multiple producer, single consumer

pub fn example() {
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handler.join().unwrap(); // wait for the spawned thread to finish executing

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // at this point the spawned thread may or may not have finished executing
    // if the spawned thread has not finished executing, the main thread will exit and the spawned thread will be terminated

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector {:?}", v)
    });

    handle.join().unwrap();
}

pub fn example2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hi");

        tx.send(msg).unwrap();
        // println!("val is {}", val); // this will not compile because val has been moved to the spawned thread
    });

    let received = rx.recv().unwrap();
    println!("Got message {}", received);
}