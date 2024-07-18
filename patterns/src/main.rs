fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // IF LET pattern --- Compiler does not check for exhaustiveness

    println!("------------------ IF LET -------------------");

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tues is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // WHILE LET pattern

    println!("------------------ WHILE LET -------------------");

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // FOR pattern

    println!("------------------ FOR -------------------");

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // FUNCTION PARAMETERS pattern

    println!("------------------ FUNCTION PARAMETERS -------------------");

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({x}, {y})");
    }

    let point = (3, 5);
    print_coordinates(&point);

    // Patterns in IF LET and WHILE LET are refutable,
    // means they can fail to match for some possible values.
    // Suppose you are expecting `None` but got `Some(x)`

    // Patterns in FOR, FUNCTION PARAMTERS, and LET VARIBALE DECLARATION
    // are irrefutable.

    // You can't use refutable patterns on the place irrefutable patters are expected.
}
