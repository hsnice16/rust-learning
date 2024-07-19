use std::slice;

// C programming language function in Rust
extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32; // Valid -- Immutable Raw Pointer

    // Starting with _ to avoid warning for unused variable
    let _r2 = &mut num as &mut i32; // Valid -- Mutable Raw Pointer

    let address = 0x298260usize;
    let _r = address as *const i32; // Can't certain of validity

    unsafe {
        println!("r1 is: {}", *r1);
        // println!("r is: {}", *r); // Segmentation fault

        dangerous()
    }

    // dangerous() // can't call unsafe function outside unsafe block

    let mut v = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(v, [1, 2, 3, 4, 5, 6]);

    let r = &mut v[..];
    assert_eq!(r, &mut [1, 2, 3, 4, 5, 6]);

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // let (a, b) = split_in_mut(r, 3);

    // assert_eq!(a, &mut [1, 2, 3]);
    // assert_eq!(b, &mut [4, 5, 6]);

    let (a, b) = split_in_mut_unsafe(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {HELLO_WORLD}");
    add_to_count(3);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}

unsafe fn dangerous() {}

// throws error
// fn split_in_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();

//     assert!(mid <= len);

//     (&mut values[..mid], &mut values[mid..])
// }

fn split_in_mut_unsafe(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // mutable raw pointer

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid), // unsafe because takes a raw pointer and must trust that the pointer is valid
            slice::from_raw_parts_mut(ptr.add(mid), len - mid), // unsafe because takes a raw pointer and must truct that the offset location is valid
        )
    }
}
