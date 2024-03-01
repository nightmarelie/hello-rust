pub fn run (number_list: Vec<i32>) -> i32 {
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
    fn test_run() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = run(number_list);

        assert_eq!(result, 100);
    }
}
