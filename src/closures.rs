use std::thread;
use std::time::Duration;

// Closures are anonymous functions you can save in a variable or pass as arguments to other functions.
// Closures can capture values from the scope in which they’re defined. The syntax and capabilities of closures make them very convenient for on-the-fly usage.
pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    // The closure is stored in the calculation field of the Cacher instance. The value field is an Option<u32> because the Cacher can store the result of the closure if it has been run with a value. The value field is None at the start.
    let mut expensive_result = Cacher {
        calculation: |num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        },
        value: None,
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value.unwrap_or_else(|| {
                let v = expensive_result.calculation(intensity);
                expensive_result.value = Some(v);
                v
            })
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value.unwrap_or_else(|| {
                let v = expensive_result.calculation(intensity);
                expensive_result.value = Some(v);
                v
            })
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value.unwrap_or_else(|| {
                    let v = expensive_result.calculation(intensity);
                    expensive_result.value = Some(v);
                    v
                })
            );
        }
    }
}