use chrono::offset::Local;
use chrono::{TimeZone, Duration};
use humantime::{parse_duration, format_duration};

fn main() {
    let birthday = Local.with_ymd_and_hms(1990, 11, 28, 0, 0, 0).unwrap();
    let now = Local::now();
    let age: Duration = now - birthday;

    println!("You were born at: {}", birthday.format("%A, %d %B %Y"));

    let age = parse_duration(format!("{}days", age.num_days()).as_ref()).unwrap();
    println!("You are: {}", format_duration(age));
}
