pub fn test() {
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
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn failing_test() {
        panic!("Make this test fail");
    }
}