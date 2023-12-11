use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("Welcome to the Guessing game");
    let mut secret_number: u8;
    let mut continue_loop: bool = true;
    while continue_loop {
        secret_number = rand::thread_rng().gen_range(1..=10);
        loop {
            println!("Please input your guess number or type a letter to stop guessing");
            let mut input: String = String::new();
            // Read the user guess
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            // Validate the user typed a number
            if input.trim().chars().all(|c| c.is_alphabetic()) {
                println!("Exiting the guessing round.");
                break;
            }

            let input: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number");
                    continue;
                }
            };

            println!("You guessed: {}", input);
            match input.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
        println!("Do you want to play again (Y/N)?");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().eq_ignore_ascii_case("n") {
            continue_loop = false;
        }
    }
}
