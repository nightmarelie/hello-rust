pub fn example(tuple: (i32, i32)) -> i32 {
    // example of destructuring a tuple
    let (x, y) = tuple;

    // Tuple can be used to return multiple values from a function
    // return ("The result of tuple example is: ", x + y);

    return x + y;
}

// write test for example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(example((1, 2)), 3);
        assert_eq!(example((3, 4)), 7);
        assert_eq!(example((5, 6)), 11);
    }
}
