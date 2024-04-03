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
    
    let (a, b) = split_at_mut(r, 3);
    
    println!("{:?}", a);
    println!("{:?}", b);

    // 4. Implement an unsafe trait

    // 5. Access fields of unions
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    
    assert!(mid <= len);
    
    unsafe {
        (
            from_raw_parts_mut(&ptr, &mid),
            from_raw_parts_mut(&ptr.add(mid), &(len - &mid)),
        )
    }
}

fn from_raw_parts_mut<T>(ptr: &*mut T, len: &usize) -> &'static mut [T] {
    unsafe { std::slice::from_raw_parts_mut(*ptr, *len) }
}
