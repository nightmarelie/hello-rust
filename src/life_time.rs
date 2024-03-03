// Struct cann't live longer than the reference it holds
struct ImportantExcerpt<'a> {
    part: &'a str,
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

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    //
    let i = ImportantExcerpt { part: first_sentence };

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