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
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi1"),
            String::from("from1"),
            String::from("the1"),
            String::from("spawned1"),
            String::from("thread1"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}