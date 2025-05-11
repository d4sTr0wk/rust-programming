use std::io::{self, Write};

fn main() {
    print!("Introduce temperature in Fahrenheit: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let fahrenheit: f64 = input
        .trim()
        .parse()
        .expect("Please enter a valid temperature");
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("{fahrenheit}ºF is equal to {celsius:.2}ºC");
}
