struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Color(i32, i32, i32);

fn main() {
    let user1 = build_user(
        String::from("Vikram Singh Rathore"),
        String::from("chinta_chita_chita@gmail.com"),
    );

    let user2 = User {
        active: user1.active,
        name: user1.name,
        email: String::from("user2@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        name: String::from("User 3"),
        ..user2
    };

    println!("user1 email: {}", user1.email);
    // println!("user2 email: {}", user2.email); // can't user after the move
    println!("user3 email: {}", user3.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black color: {:?}", black);
    println!("origin point: {:?}", origin);
}

fn build_user(name: String, email: String) -> User {
    User {
        active: true,
        name,
        email,
        sign_in_count: 1,
    }
}
