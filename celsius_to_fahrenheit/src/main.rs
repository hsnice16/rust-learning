use std::io;

fn main() {
    println!("----------Convert temperature from Celsius to Fahrenheit----------");

    loop {
        let mut celsius_temp = String::new();

        println!("Enter temperature in Celsius:");
        read_user_input(&mut celsius_temp);

        if celsius_temp.trim() == "exit" {
            break;
        }

        let celsius_temp: f32 = match celsius_temp.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let fahrenheit_temp = celsius_temp * 33.8;
        println!("{celsius_temp} °C is equivalent to {fahrenheit_temp} °F");
        println!("Tip: type `exit` to move out from this loop.\n");
    }
}

fn read_user_input(celsius_temp: &mut String) -> () {
    io::stdin()
        .read_line(celsius_temp)
        .expect("Program was not able to read the input");
}
