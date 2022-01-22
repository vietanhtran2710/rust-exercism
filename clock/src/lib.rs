use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: hours,
            minutes: minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut timestamp = (self.hours * 60 + self.minutes + minutes) % (24 * 60);
        if timestamp < 0 {
            timestamp = 24 * 60 - timestamp.abs() % (24 * 60);
        }
        Clock {
            minutes: timestamp % 60,
            hours: timestamp / 60,
        }
    }

    pub fn get_timestamp(&self) -> i32 {
        let mut timestamp: i32 = self.hours * 60 + self.minutes;
        if timestamp < 0 {
            timestamp = 24 * 60 - timestamp.abs() % (24 * 60);
        }
        timestamp %= 24 * 60;
        timestamp
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stamp = self.get_timestamp();
        write!(f, "{:0>2}:{:0>2}", stamp / 60, stamp % 60)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.get_timestamp() == other.get_timestamp()
    }
}