use time::PrimitiveDateTime as DateTime;
use time::Duration;
// Returns a DateTime one billion seconds after start.
const GIGA_SEC: i64 = 1_000_000_000;

pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(GIGA_SEC)
}
