// Read variable file to get more information about variables in Rust
pub fn example () {
    // write examples for this type integers, floating-point numbers
    // 1. Integers
    // 1.1 Rust has a number of integer types of different sizes
    // 1.2 Signed and unsigned
    // 1.3 Signed: i8, i16, i32, i64, i128
    // 1.4 Unsigned: u8, u16, u32, u64, u128
    // 1.5 isize and usize depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture
    // 1.6 Integer literals: 98_222, 0xff, 0o77, 0b1111_0000, b'A'
    // 1.7 Integer overflow: Rust has a number of checks in place to prevent this from happening
    // 1.8 If you want to explicitly handle the overflow, you can use the standard library type: Wrapping

    let x: i8 = 127;
    let y: u8 = 255;
    let z: isize = 9223372036854775807;
    let w: usize = 18446744073709551615;

    // 2. Floating-Point Numbers
    // 2.1 Rust’s floating-point types are f32 and f64
    // 2.2 f64 is the default type
    // 2.3 Rust’s floating-point types follow the IEEE-754 standard
    // 2.4 Floating-point literals: 3.14, 0.1, 1.0e-6, 2.0E5
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}