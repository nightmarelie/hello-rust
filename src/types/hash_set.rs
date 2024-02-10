pub fn example(hash_set: HashSet<i32>) -> HashSet<i32> {
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

        let mut expected = HashSet::from([
            1,
            2,
            3,
            4,
            5,
            10,
            11,
            12,
        ]);

        assert_eq!(numbers, expected);
    }
}