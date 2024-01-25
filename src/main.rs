fn main() {
    println!("Hello, world!");

    // wait for 3 sec
    // std::thread::sleep(std::time::Duration::from_secs(3));

    let mut x: i32 = 5;
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
    // let i8: i8 = 127;
    // let i16: i16 = 32767;
    // let i32: i32 = 2147483647;
    // let i64: i64 = 9223372036854775807;
    // let i128: i128 = 170141183460469231731687303715884105727;
    // let isize: isize = 9223372036854775807;
    // let min_i8: i8 = -128;
    // let min_i16: i16 = -32768;
    // let min_i32: i32 = -2147483648;
    // let min_i64: i64 = -9223372036854775808;
    // let min_i128: i128 = -170141183460469231731687303715884105728;
    // let min_isize: isize = -9223372036854775808;
    // let max_u8: u8 = 255;
    // let max_u16: u16 = 65535;
    // let max_u32: u32 = 4294967295;
    // let max_u64: u64 = 18446744073709551615;
    // let max_u128: u128 = 340282366920938463463374607431768211455;
    // let max_usize: usize = 18446744073709551615;
    //
    // let u8: u8 = 255;
    // let u16: u16 = 65535;
    // let u32: u32 = 4294967295;
    // let u64: u64 = 18446744073709551615;
    // let u128: u128 = 340282366920938463463374607431768211455;
    // let usize: usize = 18446744073709551615;
    // let min_u8: u8 = 0;
    // let min_u16: u16 = 0;
    // let min_u32: u32 = 0;
    // let min_u64: u64 = 0;
    // let min_u128: u128 = 0;
    // let min_usize: usize = 0;
    // let max_i8: i8 = 127;
    // let max_i16: i16 = 32767;
    // let max_i32: i32 = 2147483647;
    // let max_i64: i64 = 9223372036854775807;
    // let max_i128: i128 = 170141183460469231731687303715884105727;
    // let max_isize: isize = 9223372036854775807;

    // Numeral System Description	Example Literal
    // Decimal Base-10 common form   98_222
    // Hexadecimal Base-16 0x deadbeef
    // Octal Base-8 0o77
    // Binary Base-2 0b1111_0000
    // Byte (u8 only) Base-2  b'A'

    // - Floating-point numbers
    // let f32: f32 = 3.14;
    // let f64: f64 = std::f64::consts::PI;
    // let min_f32: f32 = std::f32::MIN;
    // let min_f64: f64 = std::f64::MIN;
    // let max_f32: f32 = std::f32::MAX;
    // let max_f64: f64 = std::f64::MAX;

    // - Numeric operations
    // let sum = 5 + 10;
    // let difference = 95.5 - 4.3;
    // let product = 4 * 30;
    // let quotient = 56.7 / 32.2;
    // let remainder = 43 % 5;

    // - Booleans
    let t = true;
    let f: bool = false;

    // let not_true = !t;
    // let true_and_false = t && f;

    if t {
        println!("t is {}", t);
    } else {
        println!("t is {}", f);
    }

    // - Characters
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // print all of them at once
    println!("{} {} {}", c, z, heart_eyed_cat);

    // iterate over the string
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Compound types
    // - Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("The value of tup is: {:?}", tup);
    // destructuring
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {}, {}, {}", x, y, z);
    // access by index
    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {}", tup.1);
    println!("The value of tup.2 is: {}", tup.2);

    // - Arrays
    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);
    // access by index
    println!("The value of a[0] is: {}", a[0]);
    println!("The value of a[1] is: {}", a[1]);
    println!("The value of a[2] is: {}", a[2]);
    println!("The value of a[3] is: {}", a[3]);
    println!("The value of a[4] is: {}", a[4]);

    // iterate over the array
    for element in a.iter() {
        println!("The value of element is: {}", element);
    }

    // - Slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("The value of slice is: {:?}", slice);

    // - Strings
    let s = String::from("hello");
    println!("The value of s is: {}", s);
    // - String literals
    // let s = "hello";

    // Vector
    let v = vec![1, 2, 3, 4, 5];
    println!("The value of v is: {:?}", v);

    // Custom types
    // - Structs
    // struct User {
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    //     active: bool,
    // }
    // // print all User fields
    // println!("User username is: {}", User::username);

    // - Enums
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four: {:?}", four);
    println!("six: {:?}", six);

    // match
    let four = IpAddrKind::V4;
    match four {
        IpAddrKind::V4 => println!("four: {:?}", four),
        IpAddrKind::V6 => println!("six: {:?}", six),
    }
}
