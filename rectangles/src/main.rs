#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Self) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }

    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 20,
    };

    println!("Area of rectangle is {} square pixels", area(&rect1));
    println!("use Debug: {:?}", rect1);
    println!("use Debug formatted: {:#?}", rect1);
    dbg!(&rect1);
    println!("Use area method of the instance: {}", rect1.area());

    let rect2 = Rectangle {
        width: 50,
        height: 10,
    };

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));

    let sq1 = Rectangle::square(2);
    println!("sq1: {:?}", sq1);
}

fn area(rect: &Rectangle) -> i32 {
    rect.width * rect.height
}
