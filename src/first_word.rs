pub fn example() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// write tests for the function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        let my_string = String::from("hello world");
        assert_eq!(first_word(&my_string[..]), "hello");
    }
}
