pub fn example() {
    // Unsafe code ability:

    // 1. Dereferencing a raw pointer
    let mut num = 5;
    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer
    
    let address = 0x012345usize;
    let r = address as *const i32;

    // 2. Call an unsafe function or method

    // 3. Access or modify a mutable static variable

    // 4. Implement an unsafe trait

    // 5. Access fields of unions
}
