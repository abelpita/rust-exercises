use std::io;

pub fn run() {
    println!("Welcome to the temperature converter");
    let mut continue_loop: bool = true;

    while continue_loop  {
        println!("Choose the type of conversion you want to do");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Exit");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u8 = choice.trim().parse().expect("Please type a number!");
        match choice {
            1 => fahrenheit_to_celsius(),
            2 => celsius_to_fahrenheit(),
            3 => continue_loop = false,
            _ => println!("Invalid choice!"),
        };
        if !continue_loop {
            break;
        }
        println!("Do you want to make another conversion? (y/n)");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();
        if choice != "y" {
            continue_loop = false;
        }
    }
    println!("Exiting the temperature converter");
}


fn fahrenheit_to_celsius() {
    println!("Give me the temperature in Fahrenheit");
    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");
    let fahrenheit: f32 = fahrenheit.trim().parse().expect("Please type a number!");
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("The temperature in Celsius is: {}", celsius);
}

fn celsius_to_fahrenheit() {
    println!("Give me the temperature in Celsius");
    let mut celsius = String::new();
    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");
    let celsius: f32 = celsius.trim().parse().expect("Please type a number!");
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("The temperature in Fahrenheit is: {}", fahrenheit);
}
