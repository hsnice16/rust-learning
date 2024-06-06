// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum IpAddrKindWithData {
//     V4(String),
//     V6(String),
// }

// enum IpAddrKindWithDifferentTypes {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loop_back = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let enum_home = IpAddrKindWithData::V4(String::from("127.0.0.1"));
    // let different_type_enum_home = IpAddrKindWithDifferentTypes::V4(127, 0, 0, 1);

    let five = Some(5);
    let five_plus_one = plus_one(five);
    let none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
