pub fn example (hash_set: HashSet<i32>) -> HashSet<i32> {
    let mut numbers = hash_set;

    numbers.insert(10);
    numbers.insert(11);
    numbers.insert(12);

    return numbers;
}

// write test for example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut numbers = HashSet::new();
        numbers.insert(1);
        numbers.insert(2);
        numbers.insert(3);
        numbers.insert(4);
        numbers.insert(5);
        numbers = example(numbers);

        let mut expected = HashSet::new();
        expected.insert(1);
        expected.insert(2);
        expected.insert(3);
        expected.insert(4);
        expected.insert(5);
        expected.insert(10);
        expected.insert(11);
        expected.insert(12);

        assert_eq!(numbers, expected);
    }
}