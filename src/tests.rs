// All tests run in parallel by default, so make sure your tests don’t depend on each other or on any shared state, including a shared environment, such as the current working directory or environment variables. If your tests do depend on shared state, you can serialize them by specifying the --test-threads=1 flag to the test runner. For example, to run the tests in tests.rs one at a time:
// All tests run in a separate thread, so they can’t share mutable state. This is why the tests in tests.rs don’t need to be in a module called integration_tests. Cargo treats the tests in tests.rs as integration tests regardless of whether they’re in a module.
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn test() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let rect1 = Rectangle { width: 8, height: 7 };
        let rect2 = Rectangle { width: 5, height: 1 };

        assert!(rect1.can_hold(&rect2));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let rect1 = Rectangle { width: 8, height: 7 };
        let rect2 = Rectangle { width: 5, height: 1 };

        assert!(!rect2.can_hold(&rect1));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_two_2() {
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Cargo");

        assert!(result.contains("Cargo"), "Greeting did not contain name, value was `{}`", result);
    }

    #[test]
    #[should_panic(expected = "test")]
    fn test_run() {
        panic!("test")
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
