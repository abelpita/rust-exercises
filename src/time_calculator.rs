use std::io;
use time::{self, Month};

fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn run() {
    println!("Welcome to the time calculator");
    let mut continue_loop: bool = true;

    while continue_loop {
        println!("Choose the type of operation:");
        println!("1. Add Gigaseconds to a date");
        println!("2. Convert Gigaseconds to years, days, hours, minutes and seconds");
        println!("3. Multiply seconds by 10 raised to the power X and then convert to years, days, hours, minutes and seconds");
        println!("4. Exit");
        match read_user_input().parse().expect("Please type a number!") {
            1 => add_gigaseconds(),
            2 => convert_gigaseconds(),
            3 => convert_seconds(),
            4 => continue_loop = false,
            _ => println!("Invalid choice!"),
        }
        if !continue_loop {
            break;
        }
        println!("Do you want to make another operation? (y/n)");
        if read_user_input() != "y" {
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
        _ => panic!("Invalid month!"),
    };
    let day: u8 = date_parts[2].parse().expect("Please type a valid day!");
    time::Date::from_calendar_date(year, month, day).expect("Please type a valid date!")
}

fn add_gigaseconds() {
    println!("Capture the initial date in the following format: YYYY-MM-DD");
    let initial_date = string_to_date(&(read_user_input()));

    println!("Capture the gigaseconds to add:");
    let seconds: i64 = gigaseconds_to_seconds(
        read_user_input()
            .parse()
            .expect("Please type a valid number!"),
    ) as i64;
    let duration: time::Duration = time::Duration::seconds(seconds);
    let final_date = initial_date + duration;
    println!("Final date: {}", final_date);
}

fn gigaseconds_to_seconds(gigaseconds: u64) -> u64 {
    gigaseconds * 1_000_000_000
}

fn seconds_to_years_months_days_hours_minutes_seconds(
    mut seconds: f64,
) -> (u64, u64, u64, u64, u64, u64) {
    const SECONDS_IN_A_MINUTE: f64 = 60.0;
    const SECONDS_IN_A_HOUR: f64 = 60.0 * 60.0;
    const SECONDS_IN_A_DAY: f64 = SECONDS_IN_A_HOUR * 24.0;
    const DAYS_IN_A_MONTH: f64 = 30.4375;
    const DAYS_IN_YEAR: f64 = 365.25;

    let years = seconds / (SECONDS_IN_A_DAY * DAYS_IN_YEAR);
    seconds %= SECONDS_IN_A_DAY * DAYS_IN_YEAR;

    let months = seconds / (SECONDS_IN_A_DAY * DAYS_IN_A_MONTH);
    seconds %= SECONDS_IN_A_DAY * DAYS_IN_A_MONTH;

    let days = seconds / SECONDS_IN_A_DAY;
    seconds %= SECONDS_IN_A_DAY;

    let hours = seconds / SECONDS_IN_A_HOUR;
    seconds %= SECONDS_IN_A_HOUR;

    let minutes = seconds / SECONDS_IN_A_MINUTE;
    seconds %= SECONDS_IN_A_MINUTE;

    (
        years as u64,
        months as u64,
        days as u64,
        hours as u64,
        minutes as u64,
        seconds as u64,
    )
}

fn print_duration(duration: (u64, u64, u64, u64, u64, u64)) {
    println!(
        "Years: {}\nMonths: {}\nDays: {}\nHours: {}\nMinutes: {}\nSeconds: {}",
        duration.0, duration.1, duration.2, duration.3, duration.4, duration.5
    );
}

fn convert_gigaseconds() {
    println!("Capture the gigaseconds to convert:");

    let gigaseconds = read_user_input();
    let gigaseconds: u64 = gigaseconds
        .trim()
        .parse()
        .expect("Please type a valid number!");

    print_duration(seconds_to_years_months_days_hours_minutes_seconds(
        gigaseconds_to_seconds(gigaseconds) as f64,
    ));
}

fn convert_seconds() {
    println!("First, let's determine how many seconds. We'll multiply a number of seconds by 10 raised to the power X");
    println!("To what value do you want to raise 10^X?");
    let power: u32 = read_user_input()
        .parse()
        .expect("Please type a valid number!");
    let mut value: String = String::new();
    let value_string: String = 10u128.pow(power).to_string();
    for i in 0..value_string.len() {
        if (value_string.len() - i) % 3 == 0 && i != 0 {
            value.push_str(",");
        }
        value.push(value_string.chars().nth(i).unwrap());
    }
    println!(
        "Alright. We'll calculate 10^{}, which equals to {}",
        power, value
    );
    println!("Now, capture the number of seconds:");
    let seconds: u64 = read_user_input()
        .parse()
        .expect("Please type a valid number!");
    println!("{} seconds times 10^{} equals to:", seconds, power);
    let seconds = seconds * 10u128.pow(power) as u64;
    print_duration(seconds_to_years_months_days_hours_minutes_seconds(
        seconds as f64,
    ));
}
