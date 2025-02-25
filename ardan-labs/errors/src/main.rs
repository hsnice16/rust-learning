use serde::Deserialize;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
enum UsersError {
    #[error("No users found")]
    NoUsers,
    #[error("Too many users were found")]
    TooManyUsers,
}

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("myfile.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file()?;
    Ok(contents.to_uppercase())
}

#[derive(Deserialize)]
struct User {
    user: String,
}

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// fn load_users() -> GenericResult<Vec<User>> {
// fn load_users() -> anyhow::Result<Vec<User>> {
fn load_users() -> Result<Vec<User>, UsersError> {
    let my_path = Path::new("users.json");

    // let raw_text = std::fs::read_to_string(my_path)?;
    // let users: Vec<User> = serde_json::from_str(&raw_text)?;

    let raw_text = std::fs::read_to_string(my_path).map_err(|_| UsersError::NoUsers)?;
    let users: Vec<User> = serde_json::from_str(&raw_text).map_err(|_| UsersError::NoUsers)?;

    // anyhow::bail!("Oh no! We can't go on!");

    Ok(users)
}

fn main() {
    // let my_file = Path::new("myfile.txt");
    // let content = std::fs::read_to_string(my_file);

    // let s = file_to_uppercase();
    // println!("{s:#?}");

    if let Ok(content) = file_to_uppercase() {
        println!("Contents: {content}");
    }

    let content = maybe_read_a_file();

    match content {
        Ok(contents) => println!("{contents}"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => println!("File not found - myfile.txt"),
            _ => println!("Error: {e:#?}"),
        },
    }
}
