pub fn example() {
    // 1. Control Flow
    // 1.1 If expression
    // 1.2 If-else expression
    // 1.3 If-else-if expression
    let y = 10;
    let x = if (y < 10) {
        5
    } else {
        6
    };

    println!("The result of control_flow example is: {}", x);

    // 1.4 Loop
    // 1.5 While
    // 1.6 For
    // 1.7 Match
    let mut number = 0;
    loop {
        number += 1;

        if number == 10 {
            break;
        }
    }
}