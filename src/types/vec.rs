pub fn example(vector: Vec<i32>) -> Vec<i32> {
    let mut numbers = vector;

    numbers.push(10);
    numbers.push(11);
    numbers.push(12);

    return numbers;
}

// write test for example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut numbers = vec![1, 2, 3, 4, 5];
        numbers = example(numbers);

        assert_eq!(numbers, vec![1, 2, 3, 4, 5, 10, 11, 12]);
    }
}
