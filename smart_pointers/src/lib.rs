pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
