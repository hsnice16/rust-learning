use std::cell::RefCell;

use smart_pointers::{CustomSmartPointer, List, MyBox, Rc, RcList, RcRefCellList};

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let s = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *s);

    let d = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(d);
    let c = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let name = MyBox::new(String::from("Rust"));
    hello(&name);

    let rc_a = Rc::new(RcList::Cons(
        5,
        Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
    ));
    println!("Count after creating a = {}", Rc::strong_count(&rc_a));

    let rc_b = RcList::Cons(3, Rc::clone(&rc_a));
    println!("Count after creating b = {}", Rc::strong_count(&rc_a));

    {
        let rc_c = RcList::Cons(4, Rc::clone(&rc_a));
        println!("Count after creating c = {}", Rc::strong_count(&rc_a));
    }
    println!(
        "Count after c goes out of scope = {}",
        Rc::strong_count(&rc_a)
    );

    let value = Rc::new(RefCell::new(5));
    let value_a = Rc::new(RcRefCellList::Cons(
        Rc::clone(&value),
        Rc::new(RcRefCellList::Nil),
    ));

    let value_b = RcRefCellList::Cons(Rc::new(RefCell::new(3)), Rc::clone(&value_a));
    let value_c = RcRefCellList::Cons(Rc::new(RefCell::new(4)), Rc::clone(&value_a));

    println!("before value_a = {value_a:?}");
    println!("before value_b = {value_b:?}");
    println!("before value_c = {value_c:?}");

    *value.borrow_mut() += 10;

    println!("after value_a = {value_a:?}");
    println!("after value_b = {value_b:?}");
    println!("after value_c = {value_c:?}");
}

fn hello(name: &str) -> () {
    println!("Hello, {name}!");
}
