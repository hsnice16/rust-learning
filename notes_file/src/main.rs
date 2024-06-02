#![deny(clippy::all)]

use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        Err("Usage: notes your_note_goes_here")?;
        std::process::exit(1);
    }

    let note = args[1].clone();
    println!("Note: {}", note);

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("notes.txt")
        .unwrap();

    file.write_all(b"<!-- ")?;
    file.write_all(now.as_bytes())?;
    file.write_all(b" -->\n")?;

    file.write_all(note.as_bytes())?;
    file.write_all(b"\n\n")?;

    Ok(())
}
