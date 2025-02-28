// // // fn just_print_it<T: ToString>(x: T) {
// // fn just_print_it<T, U>(x: T, y: U)
// // where
// //     T: ToString,
// //     U: ToString,
// // {
// //     println!("{}", x.to_string());
// // }

// // fn main() {
// //     just_print_it("Hello", 5);
// //     just_print_it(5, 12);
// // }

// #[derive(Debug)]
// struct Degrees(f32);

// #[derive(Debug)]
// struct Radians(f32);

// impl From<Radians> for Degrees {
//     fn from(rad: Radians) -> Self {
//         Degrees(rad.0 * 100.0 / std::f32::consts::PI)
//     }
// }

// impl From<Degrees> for Radians {
//     fn from(deg: Degrees) -> Self {
//         Radians(deg.0 * std::f32::consts::PI / 180.0)
//     }
// }

// fn sin(angle: impl Into<Radians>) -> f32 {
//     let angle: Radians = angle.into();
//     angle.0.sin()
// }

// fn main() {
//     let behind_you = Degrees(180.0);
//     println!("behind_you: {behind_you:?}");

//     let behind_you_radians = Radians::from(behind_you);
//     let behind_you_radians2: Radians = Degrees(180.0).into();

//     println!("behind_you_radians: {behind_you_radians:?}");
//     println!("behind_you_radians2: {behind_you_radians2:?}");

//     let sin_value = sin(Degrees(180.0));
//     println!("sin_value: {sin_value:?}");
// }

use std::collections::HashMap;

#[derive(Debug)]
struct HashMapBucket<K, V> {
    map: HashMap<K, Vec<V>>,
}

impl<K, V> HashMapBucket<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}

fn main() {
    let mut bucket = HashMapBucket::new();
    bucket.insert("hello", 1);
    bucket.insert("hello", 2);
    bucket.insert("goodbye", 3);

    println!("{bucket:?}");
}
