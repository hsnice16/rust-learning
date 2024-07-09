use add_one;
use add_two;

fn main() {
    let num = 10;
    println!("{num} plus one is {}!", add_one::add_one(num));
    println!("{num} plus two is {}!", add_two::add_two(num));
}
