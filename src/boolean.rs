pub fn example(test: bool, test2: bool) -> bool {
    return test && test2;
}

// write test for example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(example(true, true), true);
        assert_eq!(example(true, false), false);
        assert_eq!(example(false, true), false);
        assert_eq!(example(false, false), false);
    }
}
