use std::io;

pub fn run() {
    println!("Welcome to the string reverse program");
    let mut continue_loop: bool = true;
    
    while continue_loop  {
        // capture the string to reverse
        println!("Enter the string you want to reverse");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        // Reverse the string
        let reversed_string = reverse(&input);
        println!("The reversed string is {reversed_string}");
        
        // Ask the user if they want to continue
        println!("Do you want to reverse another string? (y/n)");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();
        if choice != "y" {
            continue_loop = false;
        }
    }
    println!("Exiting the string reverse program");
}


fn reverse(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut reversed_string = String::new();
    for i in (0..bytes.len()).rev() {
        reversed_string.push(bytes[i] as char);
    }
    reversed_string
}