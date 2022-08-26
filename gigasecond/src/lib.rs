use time::{PrimitiveDateTime as DateTime, Duration, ext::NumericalDuration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.checked_add(1000000000.seconds()).unwrap()
}
