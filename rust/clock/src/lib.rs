const MINUTES_PER_HOUR:i32 = 60;
const MINUTES_PER_DAY:i32 = 24 * MINUTES_PER_HOUR;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let fixed_minutes = (((hours * MINUTES_PER_HOUR + minutes) % MINUTES_PER_DAY)
                            + MINUTES_PER_DAY) % MINUTES_PER_DAY;

        Clock {
            minutes: fixed_minutes,
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.hours(), self.minutes() + minutes)
    }

    pub fn hours(&self) -> i32 {
        self.minutes / MINUTES_PER_HOUR
    }

    pub fn minutes(&self) -> i32 {
        self.minutes % MINUTES_PER_HOUR
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours(), self.minutes())
    }
}
