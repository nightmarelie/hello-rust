use unicode_segmentation::UnicodeSegmentation;

pub fn examples() {
    // let a = [1, 2, 3, 4, 5];

    let mut vec: Vec<i32> = Vec::new(); // type required because of empty vec

    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    // let vec2 = Vec::from([1, 2, 3, 4, 5]); // type inferred from array
    // let vec3 = vec![1, 2, 3, 4, 5]; // type inferred from values

    // Accessing elements
    // let third: &i32 = &vec[2];
    let third: Option<&i32> = vec.get(2);

    match third {
        Some(value) => println!("The third element is {}", value),
        None => println!("There is no third element"),
    }

    // Iterating over elements
    for i in &mut vec {
        *i += 50;
    }

    for i in &vec {
        println!("{}", i);
    }

    // String are UTF-8 encoded
    // let s = String::new();
    // let data = "initial contents";
    // let s = data.to_string();
    // let s = String::from("hello");

    // Updating a string
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('l');

    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    let s4 = format!("{}-{}", s2, s3);
    // println!("{}", s1); // error here because s1 has been moved
    println!("{}", s3);

    // Iterating over strings
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
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