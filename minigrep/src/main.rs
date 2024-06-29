use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Search for: {query}");
    println!("Search in file: {file_path}");
}
