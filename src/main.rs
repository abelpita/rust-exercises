pub mod temperature_converter;
pub mod fibonacci_generator;
pub mod reverse_string;
pub mod time_calculator;

use std::io;

fn main() {
    println!("Welcome to the Program Chooser");
    let mut continue_loop: bool = true;
    while continue_loop {
        println!("Choose the program you want to run");
        println!("1. Temperature converter");
        println!("2. Fibonacci generator");
        println!("3. Reverse a string");
        println!("4. Time calculator");
        println!("5. Exit");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u8 = choice.trim().parse().expect("Please type a number!");
        match choice {
            1 => temperature_converter::run(),
            2 => fibonacci_generator::run(),
            3 => reverse_string::run(),
            4 => time_calculator::run(),
            5 => continue_loop = false,
            _ => println!("Invalid choice!"),
        };
    }
    println!("Exiting the Program Chooser");
}