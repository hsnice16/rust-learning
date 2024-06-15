use std::collections::HashMap;
use std::fmt;
use std::io;

use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

fn function1(result: fmt::Result, result1: Result) -> () {
    // --snip--
}

fn function2(result: io::Result<()>, result1: IoResult<()>) -> () {
    // --snip--
}
