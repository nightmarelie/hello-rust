pub fn example() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let _y = 5;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // y is shadowed
        _ => println!("Default case, x = {:?}", x),
    }

    // Multiple patterns
    let x = 1;
    
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    
    // Range patterns
    let x = 5;
    
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something"),
    }
}
