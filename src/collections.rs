pub fn examples () {
    let a = [1, 2, 3, 4, 5];

    let mut vec: Vec<i32> = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        examples();
    }
}