fn main() {
    // let n = 5;

    // let n = {
    //     let n = 6;
    //     // println!("{n}")
    //     // 6 // return -- without semicolon
    // }; // implicit return -- unit type

    // // println!("{n}");
    // println!("{n:?}")

    // let double_of_n = double(n);
    // println!("n: {n}");
    // println!("double of n: {double_of_n}");

    // println!("{n}: {}", double(n));
    // println!("-1: {}", double_or_nothing(-1));

    // let i = 5;
    // let a = if i == 5 { 6 } else { 7 };

    // println!("a: {a}");

    let name = "John".to_string();
    let name = greet(name);
    // greet(name);
    // greet(name);

    greet(name.clone()); // deep clone -- deep clone can be slow
    greet(name);

    // instead of clone, we should borrow (often called referencing/pointer)
    let ref_name = "Wick".to_string();
    greet_borrow(&ref_name);

    let mut ref_mut_name = "Didi".to_string();
    greet_borrow_mut(&mut ref_mut_name);
    println!("{}", ref_mut_name)
}

fn double(n: i32) -> i32 {
    n * 2
}

fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
        return n * 2;
    }
    // else {
    //     return 0;
    // }

    0
}

fn greet(s: String) -> String {
    println!("Hello {s}");
    s // return back the Ownership
}

fn greet_borrow(s: &String) {
    println!("Hello {s}");
}

fn greet_borrow_mut(s: &mut String) {
    // dereference s
    *s = format!("Hello {s}");
}
