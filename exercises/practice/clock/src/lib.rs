#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { minutes: (((hours * 60 + minutes) % 1440) + 1440) % 1440 }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

use std::fmt::{Display, Formatter, Result};

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

#[test]
fn test_fmt() {
    assert_eq!("03:20", Clock{ minutes: 200 }.to_string());
}
