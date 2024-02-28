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

    // Accessing elements
    let third: &i32 = &vec[2];
    let third: Option<&i32> = vec.get(2);

    match third {
        Some(value) => println!("The third element is {}", value),
        None => println!("There is no third element"),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        examples();
    }
}