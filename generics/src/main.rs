use generics::{NewsArticle, Summary, Tweet};

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Coordinate<T, U> {
    latitude: T,
    longitude: U,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest::<i32>(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 25, 100, 65];
    let result = largest::<i32>(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest::<char>(&char_list);
    println!("The largest char is {result}");

    let point = Point { x: 1, y: 1 };
    println!("Point: {point:?}");

    let coordinate = Coordinate {
        latitude: 178,
        longitude: 23.3423444,
    };
    println!("Coordinate: {coordinate:?}");

    let tweet = Tweet {
        username: String::from("hsnice16"),
        content: String::from("changed the bio"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };
    println!("New arctile available! {}", article.summarize());
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
