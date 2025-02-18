use authentication::{greet_user, login, read_line, LoginAction, LoginRole};

fn main() {
    let mut tries = 0;

    loop {
        println!("Enter your username:");
        let username = read_line();
        println!("Enter your password:");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(role)) => {
                match role {
                    LoginRole::Admin => {
                        println!("Admin: {}!", greet_user(&username))
                    }
                    LoginRole::User => {
                        println!("User: {}!", greet_user(&username))
                    }
                }
                break;
            }
            // LoginAction::Granted(LoginRole::Admin) => println!("Admin"),
            // LoginAction::Granted(LoginRole::User) => println!("User"),
            Some(LoginAction::Denied) => {
                // Do nothing
            }
            None => {
                println!("New user")
            }
        }

        println!("Incorrect username or password");
        tries += 1;

        if tries >= 3 {
            println!("Too many failed login attempts");
            break;
        }
    }
}
