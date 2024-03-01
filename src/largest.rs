pub fn run<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_largest_number() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = run(number_list);

        assert_eq!(result, 100);
    }

    #[test]
    fn find_largest_characters() {
        let character_list = vec!["a", "b", "c", "d", "e"];

        let result = run(character_list);

        assert_eq!(result, "e");
    }
}
