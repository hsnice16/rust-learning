use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

// // return static array
// pub fn get_users() -> Vec<User> {
//     vec![
//         User::new("admin", "password", LoginRole::Admin),
//         User::new("bob", "password", LoginRole::User),
//     ]
// }

fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();

    users.insert(
        "admin".to_string(),
        User::new("admin", "password", LoginRole::Admin),
    );
    users.insert(
        "bob".to_string(),
        User::new("bob", "password", LoginRole::User),
    );

    users
}

fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load the file!
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        // Create a file and return it
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

// fn get_admin_users() {
//     let users: Vec<String> = get_users()
//         .into_iter()
//         .filter(|u| u.role == LoginRole::Admin)
//         .map(|u| u.username)
//         .collect();
// }

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();
    let username = username.to_lowercase();
    let password = hash_password(password);

    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }

    // if let Some(user) = users.iter().find(|user| user.username == username) {
    //     if user.password == password {
    //         return Some(LoginAction::Granted(user.role.clone()));
    //     } else {
    //         return Some(LoginAction::Denied);
    //     }
    // }

    None

    // if username != "admin" && username != "bob" {
    //     return None;
    // }

    // if username == "admin" && password == "password" {
    //     Some(LoginAction::Granted(LoginRole::Admin))
    // } else if username == "bob" && password == "password" {
    //     Some(LoginAction::Granted(LoginRole::User))
    // } else {
    //     Some(LoginAction::Denied)
    // }
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Stdin not working");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!(greet_user("John"), "Hello John");
    }

    #[test]
    fn test_login() {
        assert_eq!(
            login("ADMIN", "password"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("bob", "password"),
            Some(LoginAction::Granted(LoginRole::User))
        );
        assert_eq!(login("admin-not", "password"), None);
    }
}
