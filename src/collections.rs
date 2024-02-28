pub fn examples() {
    let a = [1, 2, 3, 4, 5];

    let mut vec: Vec<i32> = Vec::new(); // type required because of empty vec

    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    let vec2 = Vec::from([1, 2, 3, 4, 5]); // type inferred from array
    let vec3 = vec![1, 2, 3, 4, 5]; // type inferred from values
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        examples();
    }
}