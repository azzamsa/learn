use humantime::{format_duration, parse_duration};
use time::{format_description, Date, Month, OffsetDateTime, UtcOffset};

fn main() {
    let birthday = Date::from_calendar_date(1990, Month::November, 28).unwrap();
    let birthday = birthday.with_hms(7, 0, 0).unwrap();
    let birthday = birthday.assume_offset(UtcOffset::from_hms(7, 0, 0).unwrap());
    let now = OffsetDateTime::now_local().unwrap();
    let age = now - birthday;

    let format =
        format_description::parse("[weekday repr:short], [day] [month repr:short] [year]").unwrap();
    let birthday = birthday.format(&format).unwrap();
    println!("You were born at: {}", birthday);

    let age = parse_duration(format!("{}hours", age.whole_hours()).as_ref()).unwrap();
    println!("You are: {}", format_duration(age));
}
