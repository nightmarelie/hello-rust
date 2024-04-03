pub fn example() {
    // Unsafe code ability:

    // 1. Dereferencing a raw pointer
    let mut num = 5;
    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer
    
    unsafe {
        println!("r1 is: {}", *r1); // dereference raw pointer
        println!("r2 is: {}", *r2); // dereference raw pointer
    
    }

    // 2. Call an unsafe function or method
    unsafe fn dangerous() {}
    
    unsafe {
        dangerous();
    }

    // 3. Access or modify a mutable static variable
    let mut v = vec![1, 2, 3, 4];
    let r = &mut v[..];
    
    let (a, b) = r.split_at_mut(1);
    
    println!("{:?}", a);
    println!("{:?}", b);

    // 4. Implement an unsafe trait

    // 5. Access fields of unions
}
