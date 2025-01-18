use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Customized greetings");

    println!("Please input your name");

    let mut guess = String::from("Hello,");


    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("{}",guess);
    print_time();
    let (hour, _minute) = get_current_time();
    if  hour < 12 {
        println!("Good morning to you! {:?}", print_time())
    }
    if hour >= 12 && hour <= 16 {
        println!("Good afternoon to you! {:?}", print_time())
    }
    if hour > 16 && hour <= 23 {
        println!("Good evening to you! {:?}", print_time())
    }
}

fn print_time() -> String {
    let (hour, minute) = get_current_time();
    return format!("It's {}-{} in UTC", hour, minute)
}

fn get_current_time() -> (u64, u64) {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let total_seconds = since_epoch.as_secs();
    let hours = (total_seconds / 3600) % 24; // Hours in UTC
    let minutes = (total_seconds / 60) % 60;

    return (hours, minutes);
}