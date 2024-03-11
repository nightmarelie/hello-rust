pub fn example() {
    let vec = vec![1, 2, 3];
    // iterator is lazy, it does not do anything until it is used
    let vec_iter = vec.iter();

    for val in vec_iter {
        println!("Got: {}", val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        example();
    }
}
