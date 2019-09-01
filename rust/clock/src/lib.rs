#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    fn normalise(minutes: i32) -> i32 {
        const MINUTES_IN_DAY: i32 = 60 * 24;

        let minutes = minutes % MINUTES_IN_DAY;

        if minutes >= 0 {
            minutes
        } else {
            MINUTES_IN_DAY + minutes
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: Clock::normalise(hours * 60 + minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: Clock::normalise(self.minutes + minutes),
        }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
