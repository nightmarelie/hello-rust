use std::thread;
use std::time::Duration;

// Closures are anonymous functions you can save in a variable or pass as arguments to other functions.
// Closures can capture values from the scope in which they’re defined. The syntax and capabilities of closures make them very convenient for on-the-fly usage.
// pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[allow(dead_code)]
pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} push-ups!", cached_result.value(intensity));
        println!("Next, do {} sit-ups!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cached_result.value(intensity));
        }
    }
}

#[allow(dead_code)]
pub fn capturing() {
    let x = 4;

    // The closure captures the value of x from its environment and stores it in the closure. The closure then returns the value of x + 1, which is 5.
    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
    println!("The result of equal_to_x is: {}", equal_to_x(y));

    // FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
    // FnMut can change the environment because it mutably borrows values.
    // Fn borrows values from the environment immutably.
}

// There are three ways to define closures:
// 1. Using the Fn trait. Means the closure captures values from the environment immutably.
// 2. Using the FnMut trait. Means the closure can change the environment because it mutably borrows values.
// 3. Using the FnOnce trait. Means the closure takes ownership of the variables it captures from its enclosing scope.

#[allow(dead_code)]
pub fn example() {
    // similarity between fn and closure  
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    let list_of_numbers = vec![1, 2, 3];

    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    println!("list_of_strings: {:?}", list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("list_of_statuses: {:?}", list_of_statuses);
}

#[allow(dead_code)]
fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
