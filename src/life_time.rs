// Struct cann't live longer than the reference it holds
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // method
    fn level(&'a self) -> &'a str {
        "test"
    }
}

pub fn example() {
    // dangling reference
    // let r;
    //
    // {
    //     let x = 5;
    //
    //     // r is a reference to x
    //     r = &x; // error here because x is dropped after this block
    // }
    //
    // println!("r: {}", r);


    // let string1 = "abcd";
    // let string2 = "xyz";
    //
    // let result = longest(string1, string2);
    // println!("The longest string is {}", result);

    // dangling reference
    // let string1 = "abcd";
    // let result;
    //
    // {
    //     let string2 = "xyz";
    //     result = longest(string1, string2);
    // }
    //
    // println!("r: {}", result);

    // let novel = String::from("Call me Ishmael. Some years ago...");
    // let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //
    // //
    // let i = ImportantExcerpt { part: first_sentence };

    let s = String::from("hello world");
    let word = first_word(&s);

    println!("The first word is: {}", word);

    // exa,ple of static life-time
    // The text of this string is stored directly in the program’s binary, which is always available.
    // Therefore, the lifetime of all string literals is 'static.
    let s: &'static str = "I have a";
    println!("{}", s);
}

// using generic life-time annotation
// &i32       // a reference
// &'a i32    // a reference with an explicit lifetime
// &'a mut i32// a mutable reference with an explicit lifetime
fn longest <'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// illusion rules of life-time
// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: std::fmt::Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
