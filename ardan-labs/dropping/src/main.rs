// struct MyStruct {
//     n: i32,
// }

// impl MyStruct {
//     fn new(n: i32) -> Self {
//         println!("Constructing {n}");
//         Self { n }
//     }
// }

// impl Drop for MyStruct {
//     fn drop(&mut self) {
//         println!("Dropping {}", self.n);
//     }
// }

// struct HasDroppables {
//     x: MyStruct,
// }

// fn move_me(x: MyStruct) {
//     // Do nothing
// }

// fn main() {
//     let x = MyStruct::new(3);
//     {
//         let y = MyStruct::new(2);
//     }
//     move_me(x);
//     println!("Back from function");

//     let has_drop = HasDroppables {
//         x: MyStruct::new(4),
//     };

//     println!("Ending the main function");
// }

use std::alloc::{Layout, alloc, dealloc};

struct SmartPointer<T> {
    ptr: *mut u8,
    data: *mut T,
    layout: Layout,
}

impl<T> SmartPointer<T> {
    fn new() -> SmartPointer<T> {
        println!("Allocating memory for SmartPointer");

        unsafe {
            let layout = Layout::new::<T>();
            let ptr = alloc(layout);

            SmartPointer {
                ptr,
                data: ptr as *mut T,
                layout,
            }
        }
    }

    fn set(&mut self, val: T) {
        unsafe {
            *self.data = val;
        }
    }

    fn get(&self) -> &T {
        unsafe { self.data.as_ref().unwrap() }
    }
}

impl<T> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        print!("Deallocating memory from SmartPointer");
        unsafe {
            dealloc(self.ptr, self.layout);
        }
    }
}

fn main() {
    let mut my_num = SmartPointer::<i32>::new();
    my_num.set(12);
    println!("my_num = {}", my_num.get());

    let my_num = Box::new(12);
}
 