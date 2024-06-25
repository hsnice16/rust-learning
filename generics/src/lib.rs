use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item1: &impl Summary, item2: &(impl Summary + Display)) {
    println!("Breaking news! {}", item1.summarize());
}

// different implementation
// drawback:- restricting both types to be of type T
// pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {
//     println!("Breaking news! {}", item1.summarize());
// }

// pub fn notify_next<T>(item1: &T, item2: &T)
// where
//     T: Summary + Display,
// {
//     println!("Breaking news! {}", item1.summarize());
// }

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("hsnice16"),
        content: String::from("changed the bio"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}
