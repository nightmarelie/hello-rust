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
}