use humantime::{format_duration, parse_duration};
use time::{format_description, macros::date, Duration, OffsetDateTime};

fn main() {
    let birthday = date!(1990 - 11 - 28);
    let now = OffsetDateTime::now_local().unwrap().date();
    let age: Duration = now - birthday;

    let format =
        format_description::parse("[weekday repr:short], [day] [month repr:short] [year]").unwrap();
    let birthday = birthday.format(&format).unwrap();
    println!("You were born at: {}", birthday);

    let age = parse_duration(format!("{}", age).as_ref()).unwrap();
    println!("You are: {}", format_duration(age));
}
