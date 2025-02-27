// // // // fn borrow<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
// // // //     i
// // // // }

// // // // fn main() {
// // // //     let n = 12;
// // // //     let borrowed_n = &n;
// // // //     borrow(&n, borrowed_n);
// // // // }

// // // struct Cat(String);

// // // impl Cat {
// // //     fn feed(&mut self) {
// // //         self.0 = format!("{} (purrring)", self.0);
// // //     }
// // // }

// // // struct CatFeeder<'a> {
// // //     cat: &'a mut Cat,
// // // }

// // // impl<'a> CatFeeder<'a> {
// // //     fn feed(&mut self) {
// // //         self.cat.feed();
// // //     }
// // // }

// // // fn main() {
// // //     let mut cats = vec![Cat("Froda".to_string()), Cat("Bilbo".to_string())];

// // //     let mut feeders = Vec::new();
// // //     for cat in cats.iter_mut() {
// // //         feeders.push(CatFeeder { cat });
// // //     }

// // //     feeders.iter_mut().for_each(|f| f.feed());
// // // }

// // use std::fmt;

// // struct Point {
// //     x: i32,
// //     y: i32,
// // }

// // impl fmt::Debug for Point {
// //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// //         f.debug_struct("Point")
// //             .field("x", &self.x)
// //             .field("y", &self.y)
// //             .finish()
// //     }
// // }

// use std::any::Any;

// trait Animal: std::fmt::Debug {
//     fn speak(&self);
// }

// #[derive(Debug)]
// struct Cat;

// impl Animal for Cat {
//     fn speak(&self) {
//         println!("Meow");
//     }
// }

// #[derive(Debug)]
// struct Dog;

// impl Animal for Dog {
//     fn speak(&self) {
//         println!("Woof");
//     }
// }

// fn speak_twice(animal: &impl Animal) {
//     animal.speak();
//     animal.speak();

//     println!("{animal:?}");
// }

// fn make_animal() -> impl Animal {
//     Cat
// }

// trait DowncastableAnimal {
//     fn speak(&self) {
//         println!("No idea")
//     }

//     fn as_any(&self) -> &dyn Any;
// }

// struct Tortoise;

// impl DowncastableAnimal for Tortoise {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// fn main() {
//     let cat = Cat;
//     cat.speak();

//     let dog = Dog;
//     dog.speak();

//     speak_twice(&cat);

//     let animal = make_animal();
//     animal.speak();

//     let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];
//     animals.iter().for_each(|animal| animal.speak());

//     let more_animals: Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];
//     for animal in more_animals.iter() {
//         if let Some(t) = animal.as_any().downcast_ref::<Tortoise>() {
//             println!("I'm a tortoise");
//         }
//     }
// }

use std::ops::Add;

struct Point {
    x: f32,
    y: f32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 3.0, y: 4.0 };
    let c = a + b;
    println!("c.x = {}, c.y = {}", c.x, c.y);
}
