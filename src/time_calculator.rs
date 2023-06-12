use std::io;
use time::{self, Month};

const SECONDS_IN_A_MINUTE: f64 = 60 as f64;
const SECONDS_IN_A_HOUR: f64 = (60 * 60) as f64;
const SECONDS_IN_A_DAY: f64 = SECONDS_IN_A_HOUR * 24 as f64;
const DAYS_IN_A_MONTH: f64 = 30.4375;
const DAYS_IN_YEAR: f64 = 365.25;


pub fn run() {
    println!("Welcome to the time calculator");
    let mut continue_loop: bool = true;

    while continue_loop {
        println!("Choose the type of operation:");
        println!("1. Add Gigaseconds to a date");
        println!("2. Convert Gigaseconds to years, days, hours, minutes and seconds");
        println!("3. Multiply seconds by 10 raised to the power X and then convert to years, days, hours, minutes and seconds");
        println!("4. Exit");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u8 = choice.trim().parse().expect("Please type a number!");
        match choice {
            1 => add_gigaseconds(),
            2 => convert_gigaseconds(),
            3 => convert_seconds(),
            4 => continue_loop = false,
            _ => println!("Invalid choice!")
        }
        if !continue_loop {
            break;
        }
        println!("Do you want to make another operation? (y/n)");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        if choice.trim() != "y" {
            continue_loop = false;
        }
    }
    println!("Exiting the time calculator");
}

fn string_to_date(string: &str) -> time::Date {
    let date_parts: Vec<&str> = string.trim().split("-").collect();
    let year: i32 = date_parts[0].parse().expect("Please type a valid year!");
    let month: Month = match date_parts[1].parse().expect("Please type a valid month!") {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => panic!("Invalid month!")
    };
    let day: u8 = date_parts[2].parse().expect("Please type a valid day!");
    time::Date::from_calendar_date(year, month, day).expect("Please type a valid date!")
}

fn add_gigaseconds() {
    println!("Capture the initial date in the following format: YYYY-MM-DD");
    let mut initial_date = String::new();
    io::stdin()
        .read_line(&mut initial_date)
        .expect("Failed to read line");
    let initial_date = string_to_date(&initial_date);
    
    println!("Capture the gigaseconds to add:");
    let mut gigaseconds = String::new();
    io::stdin()
        .read_line(&mut gigaseconds)
        .expect("Failed to read line");
    let gigaseconds: i64 = gigaseconds.trim().parse().expect("Please type a valid number!");
    let seconds: i64 = gigaseconds * 1_000_000_000;
    let duration: time::Duration = time::Duration::seconds(seconds);
    let final_date = initial_date + duration;
    println!("Final date: {}", final_date);
}

fn gigaseconds_to_seconds(gigaseconds: u64) -> u64 {
    (gigaseconds * 1_000_000_000) as u64
}

fn seconds_to_years_months_days_hours_minutes_seconds(mut seconds: u64) -> (u64, u64, u64, u64, u64, u64) {
    let years: u64 = ((seconds as f64) / SECONDS_IN_A_DAY / DAYS_IN_YEAR) as u64;
    seconds = seconds - (years as f64 * SECONDS_IN_A_DAY * DAYS_IN_YEAR) as u64;
    
    let months: u64 = (seconds as f64 / SECONDS_IN_A_DAY / DAYS_IN_A_MONTH) as u64;
    seconds = seconds - (months as f64 * SECONDS_IN_A_DAY * DAYS_IN_A_MONTH) as u64;
    
    let days: u64 = (seconds as f64 / SECONDS_IN_A_DAY) as u64;
    seconds = seconds - (days as f64 * SECONDS_IN_A_DAY) as u64;
    
    let hours: u64 = (seconds as f64 / SECONDS_IN_A_HOUR) as u64;
    seconds = seconds - (hours as f64  * SECONDS_IN_A_HOUR) as u64;
    
    let minutes: u64 = (seconds as f64 / SECONDS_IN_A_MINUTE) as u64;
    seconds = seconds - (minutes as f64 * SECONDS_IN_A_MINUTE) as u64;
    
    (years, months, days, hours, minutes, seconds)
}

fn print_duration(duration: (u64, u64, u64, u64, u64, u64)) {
    println!("Years: {}\nMonths: {}\nDays: {}\nHours: {}\nMinutes: {}\nSeconds: {}",
    duration.0, duration.1, duration.2, duration.3, duration.4, duration.5);
}

fn convert_gigaseconds() {
println!("Capture the gigaseconds to convert:");

    let mut gigaseconds = String::new();
    io::stdin()
        .read_line(&mut gigaseconds)
        .expect("Failed to read line");
    let gigaseconds: u64 = gigaseconds.trim().parse().expect("Please type a valid number!");
    
    print_duration(seconds_to_years_months_days_hours_minutes_seconds(gigaseconds_to_seconds(gigaseconds)));
}

fn convert_seconds() {
    println!("First, let's determine how many seconds. We'll multiply a number of seconds by 10 raised to the power X");
    println!("What is the value of X?");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
    let x: u32 = x.trim().parse().expect("Please type a valid number!");
    println!("Alright. We'll calculate 10^{}, which equals to {}", x, 10u128.pow(x));
    println!("Now, capture the number of seconds:");
    let mut seconds = String::new();
    io::stdin()
        .read_line(&mut seconds)
        .expect("Failed to read line");
    let mut seconds: u64 = seconds.trim().parse().expect("Please type a valid number!");
    println!("{} seconds times 10^{} equals to:", seconds, x);
    seconds = seconds * 10u128.pow(x) as u64;
    print_duration(seconds_to_years_months_days_hours_minutes_seconds(seconds));
}
