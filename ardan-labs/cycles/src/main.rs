// use std::{cell::RefCell, rc::Rc};

// #[derive(Debug)]
// struct Node {
//     value: i32,
//     next: RefCell<Option<Rc<Node>>>,
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         println!("Dropping node with value {}", self.value)
//     }
// }

// fn main() {
//     let tail = Rc::new(Node {
//         value: 1,
//         next: RefCell::new(None),
//     });
//     let head = Rc::new(Node {
//         value: 2,
//         next: RefCell::new(Some(tail.clone())),
//     });

//     // Let's break things
//     *tail.next.borrow_mut() = Some(head.clone());

//     // println!("head: {head:?}");
// }

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<NextNode>,
}

#[derive(Debug)]
enum NextNode {
    None,
    Strong(Rc<Node>),
    Weak(Weak<Node>),
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping node with value {}", self.value);
    }
}

fn main() {
    let tail = Rc::new(Node {
        value: 1,
        next: RefCell::new(NextNode::None),
    });
    let head = Rc::new(Node {
        value: 2,
        next: RefCell::new(NextNode::Strong(tail.clone())),
    });

    *tail.next.borrow_mut() = NextNode::Weak(Rc::downgrade(&head));

    println!("head: {head:?}");
}
