// // // // use std::rc::Rc;
// // // use std::sync::Arc;

// // // #[derive(Debug)]
// // // struct Droppable(i32);

// // // impl Droppable {
// // //     fn new(n: i32) -> Self {
// // //         println!("Constructing {n}");
// // //         Self(n)
// // //     }
// // // }

// // // impl Drop for Droppable {
// // //     fn drop(&mut self) {
// // //         println!("Dropping {}", self.0);
// // //     }
// // // }

// // // // fn move_me(x: Rc<Droppable>) {
// // // fn move_me(x: Arc<Droppable>) {
// // //     println!("Moved {}", x.0);
// // // }

// // // fn main() {
// // //     // let my_shared = Rc::new(Droppable::new(1));
// // //     let my_shared = Arc::new(Droppable::new(1));
// // //     {
// // //         let _x = my_shared.clone();
// // //         let _y = my_shared.clone();
// // //         let _z = my_shared.clone();
// // //     }
// // //     move_me(my_shared.clone());

// // //     // For Arc
// // //     let mut threads = Vec::new();
// // //     for _ in 0..10 {
// // //         let my_clone = my_shared.clone();
// // //         threads.push(std::thread::spawn(move || {
// // //             println!("{my_clone:?}");
// // //         }))
// // //     }
// // //     for t in threads {
// // //         t.join().unwrap();
// // //     }

// // //     println!("{my_shared:?}");
// // //     println!("Application exit");
// // // }

// // // Atomic Reference Count (Arc)
// // use std::sync::{Arc, Mutex};

// // struct SharedData(String);

// // fn main() {
// //     let my_shared = Arc::new(Mutex::new(SharedData("Hello".to_string())));

// //     let mut threads = Vec::new();
// //     for i in 0..10 {
// //         let my_shared = my_shared.clone();
// //         threads.push(std::thread::spawn(move || {
// //             let mut data = my_shared.lock().unwrap();
// //             data.0.push_str(&format!(" {i}"));
// //         }));
// //     }
// //     for t in threads {
// //         t.join().unwrap();
// //     }
// //     let data = my_shared.lock().unwrap();
// //     println!("{}", data.0);
// // }

// use std::sync::{Arc, Mutex};

// struct SharedData {
//     data: Mutex<String>,
// }

// impl SharedData {
//     fn new(s: &str) -> Self {
//         Self {
//             data: Mutex::new(s.to_string()),
//         }
//     }
// }

// fn main() {
//     let my_shared = Arc::new(SharedData::new("Hello"));
//     let mut threads = Vec::new();

//     for i in 0..10 {
//         let my_shared = my_shared.clone();
//         threads.push(std::thread::spawn(move || {
//             let mut data = my_shared.data.lock().unwrap();
//             data.push_str(&format!(" {i}"));
//         }))
//     }

//     for t in threads {
//         t.join().unwrap();
//     }

//     let data = my_shared.data.lock().unwrap();
//     println!("{data}");
// }

use std::{cell::RefCell, sync::Arc};

struct MyData {
    data: RefCell<String>,
}

impl MyData {
    fn new() -> Self {
        Self {
            data: RefCell::new("Hello".to_string()),
        }
    }
}

fn move_data(data: Arc<MyData>) {
    let mut data = data.data.borrow_mut();
    data.push_str(" World");
}

fn main() {
    let shared_data = Arc::new(MyData::new());
    move_data(shared_data.clone());
    let data = shared_data.data.borrow();
    println!("{data}");
}
