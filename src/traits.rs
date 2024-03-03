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
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    notify(&tweet);
    notify(&article);

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", article.summarize());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        example();
    }
}