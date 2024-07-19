fn main() {
    let mut num = 5;

    let r1 = &num as *const i32; // Valid -- Immutable Raw Pointer

    // Starting with _ to avoid warning for unused variable
    let _r2 = &mut num as &mut i32; // Valid -- Mutable Raw Pointer

    let address = 0x298260usize;
    let r = address as *const i32; // Can't certain of validity

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r is: {}", *r);
    }
}
