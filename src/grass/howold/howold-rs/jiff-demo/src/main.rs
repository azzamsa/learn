use humantime::{format_duration, parse_duration};
use jiff::{civil::date, Zoned};

fn main() {
    let now = Zoned::now();
    let birthday = date(1990, 11, 28).intz("Asia/Jakarta").unwrap();
    let age = now.since(&birthday).unwrap();

    println!("You were born at: {}", birthday.strftime("%A, %d %B %Y"));
    let age = parse_duration(&format!("{}hours", age.get_hours())).unwrap();
    println!("You are: {}", format_duration(age));
}
