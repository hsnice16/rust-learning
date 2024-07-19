pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// // Possible Generic way could be
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        //
    }
}
