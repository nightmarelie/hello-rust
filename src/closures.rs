use std::thread;
use std::time::Duration;

// Closures are anonymous functions you can save in a variable or pass as arguments to other functions.
// Closures can capture values from the scope in which they’re defined. The syntax and capabilities of closures make them very convenient for on-the-fly usage.
pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
