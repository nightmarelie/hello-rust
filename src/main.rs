fn main() {
    println!("Hello, world!");

    // wait for 3 sec
    std::thread::sleep(std::time::Duration::from_secs(3));

    let mut x:i32 = 5;
    println!("The value of x is: {} (first time)", x);

    x = 10;
    println!("The value of x is: {} (second time)", x);

    // shadowing
    let x = 11;
    println!("The value of x is: {} (third time, after shadowing)", x);

    const MAX_POINTER: u32 = 200_000_000;
    println!("The value of MAX_POINTER is {}", MAX_POINTER);

    // define a var and assign
    let x;
    x = 1;
    println!("The value of x is: {})", x);

    // DATA TYPES
    // Scalar types
    // - Integers
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // create all integer types
    let i8: i8 = 127;
    let i16: i16 = 32767;
    let i32: i32 = 2147483647;
    let i64: i64 = 9223372036854775807;
    let i128: i128 = 170141183460469231731687303715884105727;
    let isize: isize = 9223372036854775807;
    let min_i8: i8 = -128;
    let min_i16: i16 = -32768;
    let min_i32: i32 = -2147483648;
    let min_i64: i64 = -9223372036854775808;
    let min_i128: i128 = -170141183460469231731687303715884105728;
    let min_isize: isize = -9223372036854775808;
    let max_u8: u8 = 255;
    let max_u16: u16 = 65535;
    let max_u32: u32 = 4294967295;
    let max_u64: u64 = 18446744073709551615;
    let max_u128: u128 = 340282366920938463463374607431768211455;
    let max_usize: usize = 18446744073709551615;

    let u8: u8 = 255;
    let u16: u16 = 65535;
    let u32: u32 = 4294967295;
    let u64: u64 = 18446744073709551615;
    let u128: u128 = 340282366920938463463374607431768211455;
    let usize: usize = 18446744073709551615;
    let min_u8: u8 = 0;
    let min_u16: u16 = 0;
    let min_u32: u32 = 0;
    let min_u64: u64 = 0;
    let min_u128: u128 = 0;
    let min_usize: usize = 0;
    let max_i8: i8 = 127;
    let max_i16: i16 = 32767;
    let max_i32: i32 = 2147483647;
    let max_i64: i64 = 9223372036854775807;
    let max_i128: i128 = 170141183460469231731687303715884105727;
    let max_isize: isize = 9223372036854775807;

    // Numeral System Description	Example Literal
    // Decimal Base-10 common form   98_222
    // Hexadecimal Base-16 0x deadbeef
    // Octal Base-8 0o77
    // Binary Base-2 0b1111_0000
    // Byte (u8 only) Base-2  b'A'

    // - Floating-point numbers
    let f32: f32 = 3.14;
    let f64: f64 = std::f64::consts::PI;
    let min_f32: f32 = std::f32::MIN;
    let min_f64: f64 = std::f64::MIN;
    let max_f32: f32 = std::f32::MAX;
    let max_f64: f64 = std::f64::MAX;

    // - Booleans
    // - Characters
    // Compound types
    // - Tuples
    // - Arrays
    // Custom types
    // - Structs
    // - Enums


}
