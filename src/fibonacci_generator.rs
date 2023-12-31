use std::io;

pub fn run() {
    println!("Welcome to the Fibonacci generator");
    let mut continue_loop: bool = true;
    while continue_loop {
        // Get the number to generate the nth Fibonacci number
        println!("Give a number to generate the nth Fibonacci number or type a letter to exit");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        // Validate if the user typed a letter
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue_loop = false;
                continue;
            }
        };
        // Generate the nth Fibonacci number
        let f_n: u128 = get_nth_fibonacci(number);
        println!("Result");
        println!("F{} -> {}", number, f_n);
    }
    println!("Exiting the Fibonacci generator");
}

fn get_nth_fibonacci(n: u32) -> u128 {
    let mut next_fib: u128 = 0;
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut prev_fib = 0;
            let mut current_fib = 1;
            for _ in 2..=n {
                next_fib = current_fib + prev_fib;
                prev_fib = current_fib;
                current_fib = next_fib;
            }
            next_fib
        }
    }
}
