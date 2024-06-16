use std::collections::HashMap;

fn main() {
    // Vector with type annotation
    let v1: Vec<i32> = Vec::new();
    println!("Vection v1: {:?}", v1);

    // Vector using macro
    let v2 = vec![1, 2, 3];
    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third (Option) element is {third}"),
        None => println!("There is no third element."),
    }

    // Fill vector values using push
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);

    for i in &v3 {
        println!("v3 element: {i}")
    }

    for i in &mut v3 {
        *i += 50;
    }

    for i in &v3 {
        println!("v3 element (after mutation): {i}")
    }

    // Vector with different element types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for row_cell in &row {
        match row_cell {
            SpreadsheetCell::Int(integer) => println!("Integer row cell: {integer}"),
            SpreadsheetCell::Float(float) => println!("Float row cell: {float}"),
            SpreadsheetCell::Text(string) => println!("String row cell: {string}"),
        }
    }

    // String
    let s = String::new();
    println!("String using new: {s}");

    // String literal
    let data = "initial contents";
    let literal_to_string = data.to_string();
    println!("Literal to string: {literal_to_string}");

    // String using from
    let string_using_from = String::from("initial contents");
    println!("String using from associated function: {string_using_from}");

    let mut foo_string = String::from("foo");
    println!("foo_string before push_str: {foo_string}");
    let bar = "bar";
    foo_string.push_str(bar);
    println!("foo_string after push_str: {foo_string}");

    let mut lo = String::from("lo");
    println!("lo before push: {lo}");
    lo.push('l');
    println!("lo after push: {lo}");

    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let hello_world = hello + &world;
    println!("hello_world: {hello_world}");

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let format_concat = format!("{tic}-{tac}-{toe}");
    let plus_concat = tic + "-" + &tac + "-" + &toe;

    println!("format_concat: {format_concat}");
    println!("plus_concat: {plus_concat}");

    let two_bytes_char_string = String::from("Зд");
    for char in two_bytes_char_string.chars() {
        println!("char of two_bytes_char_string: {char}");
    }

    for byte in two_bytes_char_string.bytes() {
        println!("byte of two_bytes_char_string: {byte}");
    }

    // Create empty HashMap
    let mut scores = HashMap::new();

    // Insert values in HashMap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Get value from HashMap
    let team_name = String::from("Blue");
    let blue_team_score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue team score: {blue_team_score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let mut blue_scores = HashMap::new();
    blue_scores.insert(String::from("Blue"), 10);
    blue_scores.insert(String::from("Blue"), 25);

    println!("Blue Scores: {blue_scores:?}");

    blue_scores.entry(String::from("Yellow")).or_insert(50);
    blue_scores.entry(String::from("Blue")).or_insert(50);

    println!("Blue Scores after entry: {blue_scores:?}");

    let text = "hello world wonderfull world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
