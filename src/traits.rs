use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}

pub fn notify(item: &(impl Summary + Debug)) {
    println!("Breaking news! {}", item.summarize());
}

// the impl Trait syntax is syntactic sugar for a longer form:
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn some_function<T: Summary + Debug>(item: &T) {
    println!("Breaking news! {:?}", item);
}

// restriction with return value using impl Trait should be used when you want to return a single type, but don’t want to write out a long type signature.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Using Trait Bounds to Conditionally Implement Methods
// You can also conditionally implement methods on a generic type depending on whether the generic type has a particular trait bound or not.
// For example, the type Tuples that implements the cmp_display method on Pair<T> if T has the Display and PartialOrd traits:
// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }

// Using Trait Objects That Allow for Values of Different Types
// impl <T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         format!("{}", self)
//     }
// }

// where clause
pub fn some_function2<T>(item: &T)
where
    T: Summary + Debug,
{
    println!("Breaking news! {:?}", item);
}

pub fn example() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    notify(&tweet);
    notify(&article);

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", article.summarize());
    
    let human = Human;
    
    // human.fly(); // error: multiple `fly` found
    
    Pilot::fly(&human);
    Wizard::fly(&human);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        example();
    }

    #[test]
    fn test_add() {
        let p1 = Point { x: 1, y: 0 };
        let p2 = Point { x: 2, y: 3 };
        assert_eq!(p1 + p2, Point { x: 3, y: 3 });
    }
}

// Advanced Traits
pub trait Iterator {
    type Item; // associated type. Can be used to define a placeholder type in the definition of a trait or struct. The implementor of a trait will specify the concrete type for the associated type.

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(42)
    }
}

// Default Generic Type Parameters and Operator Overloading
// The Add trait is defined in the standard library. It is used to overload the + operator.

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Call the same method with different types
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// Supertraits
// A trait can require another trait to be implemented. For example, the OutlinePrint trait requires the Display trait to be implemented.
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}



