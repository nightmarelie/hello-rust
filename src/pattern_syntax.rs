struct Point {
    x: i32,
    y: i32,
}

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
    
    let x = 'c';
    
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; // a = 0, b = 7
    
    assert_eq!(0, a);
    assert_eq!(7, b);
}
