pub fn example() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // that mean dereference y

    let x = 5;
    let y = Box::new(x);

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