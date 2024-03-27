use std::thread;
use std::time::Duration;

pub fn example() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // at this point the spawned thread may or may not have finished executing
    // if the spawned thread has not finished executing, the main thread will exit and the spawned thread will be terminated
}