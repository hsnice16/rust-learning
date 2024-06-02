use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("####### Start the Number Guessing Game ########");

    loop {
        let mut player_guessed = String::new();

        println!("----> Enter your guess: ");
        stdin()
            .read_line(&mut player_guessed)
            .expect("Game was not able to read your input.");

        // Shadowing
        let player_guessed: u32 = match player_guessed.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut rand_generator = rand::thread_rng();
        let rand_number = rand_generator.gen_range(1..=100);

        match player_guessed.cmp(&rand_number) {
            Ordering::Less => println!("Too small 😭😭!"),
            Ordering::Greater => println!("Too big 😭😭!"),
            Ordering::Equal => {
                println!("You win 🎉🎉!");
                break;
            }
        }
    }
}
