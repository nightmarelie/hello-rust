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


    let string1 = "abcd";
    let string2 = "xyz";

    let result = longest(string1, string2);
    println!("The longest string is {}", result);
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