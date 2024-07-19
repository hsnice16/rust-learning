fn main() {
    // type alias - main use case is to reduce repetition for longer types in the code
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}
