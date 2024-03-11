pub fn example() {
    let vec = vec![1, 2, 3];
    // iterator is lazy, it does not do anything until it is used
    let vec_iter = vec.iter();

    for val in vec_iter {
        println!("Got: {}", val);
    }
}

// all iterators implement the Iterator trait
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // default implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        example();
    }
}
