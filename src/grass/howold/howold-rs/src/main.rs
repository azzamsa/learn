use chrono::prelude::*;
use chrono::Date;

fn main() {
    let today = Local::today();
    let birth_date = Local.ymd(2000, 11, 28);

    // let age = today - birth_date;
    // let years = age.num_days() / 365;
    // NOTE sadly chrono rust doesn't have num_years and num_month
    // https://github.com/chronotope/chrono/issues/416
    let (years, months, days) = diff(today, birth_date);

    println!("You were born at: {}", birth_date.format("%A, %d %B %Y"));
    println!("You are: {} years, {} months, {} days", years, months, days);
}

fn diff(date1: Date<Local>, date2: Date<Local>) -> (i32, i32, i32) {
    // ported from icza/gox/timex.go
    let y1 = date1.year();
    let m1 = date1.month();
    let d1 = date1.day();

    let y2 = date2.year();
    let m2 = date2.month();
    let d2 = date2.day();

    let mut year = y1 as i32 - y2 as i32;
    let mut month = m1 as i32 - m2 as i32;
    let mut day = d1 as i32 - d2 as i32;

    // Normalize negative values
    let days_in_m1 = get_days_from_month(y1, m1);
    if day < 0 {
        // days in month:
        let t = Local.ymd(y1, m1, days_in_m1 as u32);
        day += days_in_m1 - t.day() as i32;
        month -= 1;
    }
    if month < 0 {
        month += 12;
        year -= 1;
    }

    return (year, month, day.abs());
}

fn get_days_from_month(year: i32, month: u32) -> i32 {
    Local
        .ymd(
            match month {
                12 => year + 1,
                _ => year,
            },
            match month {
                12 => 1,
                _ => month + 1,
            },
            1,
        )
        .signed_duration_since(Local.ymd(year, month, 1))
        .num_days() as i32
}
