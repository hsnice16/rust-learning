use std::io;

fn main() {
    println!("---------Generate nth Fibonacci number---------");

    loop {
        let mut nth_number = String::new();

        println!("Enter the nth Fibonacci to find:");
        read_user_input(&mut nth_number);

        if nth_number.trim() == "exit" {
            break;
        };

        let nth_number: u32 = match nth_number.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        if nth_number == 0 {
            println!("The count starts from 1. Please enter a number greater than 0.\n");
            continue;
        }

        let mut nth_fibonacci: u32 = 0;

        if nth_number >= 2 {
            let mut nth_minus_one_fibonacci: u32 = 1;

            for _ in 0..(nth_number - 1) {
                let temp_value = nth_fibonacci;
                nth_fibonacci += nth_minus_one_fibonacci;
                nth_minus_one_fibonacci = temp_value;
            }
        }

        println!("Fibonacci number at position {nth_number} is: {nth_fibonacci}");
        println!("Tip: type `exit` to move out from this loop.\n");
    }
}

fn read_user_input(nth_number: &mut String) -> () {
    io::stdin()
        .read_line(nth_number)
        .expect("Program was not able to read the input.");
}
