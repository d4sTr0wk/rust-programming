use std::io::{self, Write};

fn main() {
    print!("Introduce the index of the fibonacci number needed: ");
    std::io::stdout().flush().expect("Failed to flush stdout");

    let index: usize = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(num) => break num,
            Err(_) => println!("Invalid input. Please enter a valid number"),
        }
    };

    let mut fn_0 = 0;
    let mut fn_1 = 1;

    if index == 0 {
        fn_1 = 0;
    } else if index != 1 {
        for _ in 2..index {
            let next = fn_0 + fn_1;
            fn_0 = fn_1;
            fn_1 = next;
        }
    }
    println!("The fibonnaci number at index {index} is: {fn_1}");
}
