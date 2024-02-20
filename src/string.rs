pub fn example () {
    // 1. Strings
    // 1.1.1 Owned String
    // 1.1 Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str
    // 1.2 The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type
    // 1.3 Rust’s standard library also includes a number of other string types, such as OsString, OsStr, CString, and CStr
    // 1.4 Rust strings are UTF-8 encoded
    // 1.5 Rust strings are immutable
    // 1.6 Rust strings are not null-terminated
    let mut s = String::new();
    // 1.1.2 String literals | String slice
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    // 1.7 Updating a string
    // 1.8 Appending to a string with push_str and push
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("The result of str example is: {}", s);

    let mut s = String::from("lo");
    s.push('l');
    println!("The result of str example is: {}", s);

    // 1.7 Characters
    // 1.7.1 Char - Unicode Scalar Value
    // 1.8 Accessing characters in a string
    let s = String::from("hello");
    let h = s.chars().nth(0);
    println!("The result of str example is: {:?}", h);

    let characters: char = 'a';
    let characters: char = 'b';
    let characters: char = 'c';

    println!("The result of str example is: {}", characters);

    // 1.9 Slicing
    // 1.10 Slicing a string
    let hello = "Hello, world!";
    let s = &hello[0..4];
    println!("The result of str example is: {}", s);


}