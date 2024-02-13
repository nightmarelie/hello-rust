use std::collections::HashMap;

pub fn example (hash_map: HashMap<i32, i32>) -> HashMap<i32, i32> {
    let mut numbers = hash_map;

    numbers.insert(10, 10);
    numbers.insert(11, 11);
    numbers.insert(12, 12);

    return numbers;
}

pub fn example2 (hash_map: HashMap<&str, i32>) -> HashMap<&str, i32> {
    let mut vikings = hash_map;

    vikings.insert("Einar", 25);

    return vikings;
}

// write test for example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut numbers = HashMap::new();
        numbers.insert(1, 1);
        numbers.insert(2, 2);
        numbers.insert(3, 3);
        numbers.insert(4, 4);
        numbers.insert(5, 5);
        numbers = example(numbers);

        let expected = HashMap::from([
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (10, 10),
            (11, 11),
            (12, 12),
        ]);

        assert_eq!(numbers, expected);
    }

    #[test]
    fn test_example2() {
        let mut vikings = HashMap::new();
        vikings.insert("Einar", 25);
        vikings = example2(vikings);

        let mut expected = HashMap::new();
        expected.insert("Einar", 25);

        assert_eq!(vikings, expected);
    }
}
