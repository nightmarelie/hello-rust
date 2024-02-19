// 1. Arrays
// Arrays are useful when you want your data allocated on the stack rather than the heap. They are useful when you want to ensure you always have a fixed number of elements.
// 1.1 Arrays have a fixed length
// 1.2 Once declared, they cannot grow or shrink in size
pub fn example(numbers: [i32; 5]) -> i32 {
    let mut sum = 0;

    // 1.3 Declare array with default value and langth
    // let a = [3; 5];

    // 1.4 Accessing array elements
    // let first = a[0];

    for number in numbers.iter() {
        sum += number;
    }

    return sum;
}

// write test for example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(example(), 15);
    }
}