use time::ext::NumericalDuration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // todo!("What time is a gigasecond later than {start}");
    // https://docs.rs/time/latest/time/struct.PrimitiveDateTime.html#method.checked_add
    // https://docs.rs/time/latest/time/struct.Duration.html
    start.checked_add(1_000_000_000_i64.seconds()).unwrap()
}
