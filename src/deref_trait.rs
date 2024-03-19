pub fn example() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // that mean dereference y. It will follow the reference to the value it’s pointing to.

    let x = 5;
    let y = Box::new(x); // Box<T> is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references.

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // let x = 5;
    // let y = MyBox::new(x);
    //
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref_trait() {
        example();
    }
}