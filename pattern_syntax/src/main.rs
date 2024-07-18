struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum NewMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum HelloMessage {
    Hello { id: i32 },
}

fn main() {
    println!("-------------------- Matching Literals --------------------");

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    println!("-------------------- Matching Named Variables --------------------");

    let x = Some(5); // --- Both are Irrefutable patterns
    let y = 10; // ---

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    println!("-------------------- Multiple Patterns --------------------");

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    };

    println!("-------------------- Matching Ranges of Values --------------------");

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    println!("-------------------- Destructuring to Break Apart Values --------------------");

    println!("-------------------- ##1 Destructing Structs --------------------");

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    println!("Point -- x -- a: {}", a);
    println!("Point -- y -- b: {}", b);

    let Point { x, y } = p;
    println!("Point -- x: {}", x);
    println!("Point -- y: {}", y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})")
        }
    }

    println!("-------------------- ##2 Destructing Enums --------------------");

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has not data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
    }

    println!("-------------------- ##3 Destructing Nested Structs and Enums --------------------");

    let msg = NewMessage::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        NewMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        NewMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }

    // -------------------- Destructuring Structs and Tuples --------------------

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!("-------------------- Ingnore Entire value with _ --------------------");

    foo(3, 4);

    println!("-------------------- Ingnore Parts of a value with _ --------------------");

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    println!("-------------------- Ingnore Remaining parts with .. --------------------");

    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"),
    }

    println!("-------------------- Extra Conditionals with Match Guards --------------------");

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    println!("-------------------- @ Bindings --------------------");

    let msg = HelloMessage::Hello { id: 5 };

    match msg {
        HelloMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        HelloMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        HelloMessage::Hello { id } => println!("Found some other id: {id}"),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}
