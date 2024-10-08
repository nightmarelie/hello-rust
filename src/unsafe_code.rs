static HELLO_WORLD: &str = "Hello, world!"; // global variable

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn example() {
    // Unsafe code ability:

    // 0. Reading from or writing to a mutable static variable is unsafe
    unsafe {
        println!("COUNTER: {COUNTER}");
    }

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
    
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3) );
    }
    
    println!("name is : {}", HELLO_WORLD);

    // 4. Implement an unsafe trait 
    
    // see above code

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

#[no_mangle] // don't mangle the name of this function. witch is mean that the name of this function will be the same as the name of the function in the C code
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
