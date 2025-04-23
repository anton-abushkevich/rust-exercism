use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let rolled = Clock::roll(hours, minutes);
        Self {
            hours: rolled.0,
            minutes: rolled.1
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    fn roll(hours: i32, minutes: i32) -> (i32, i32) {
        let mut hours = (hours + (minutes / 60)) % 24;
        if hours < 0 {
            hours += 24;
        }
        let mut minutes = minutes % 60;
        if minutes < 0 {
            hours = if hours == 0 { 23 } else { hours - 1 };
            minutes += 60
        }
        (hours, minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(Clock::new(8, 0).to_string(), "08:00");
        assert_eq!(Clock::new(8, 30).to_string(), "08:30");
        assert_eq!(Clock::new(8, -30).to_string(), "07:30");
        assert_eq!(Clock::new(24, 0).to_string(), "00:00");
        assert_eq!(Clock::new(25, 0).to_string(), "01:00");
        assert_eq!(Clock::new(100, 0).to_string(), "04:00");
        assert_eq!(Clock::new(1, 60).to_string(), "02:00");
    }

    #[test]
    fn check2() {
        assert_eq!(Clock::new(0, 45).add_minutes(40).to_string(), "01:25");
        assert_eq!(Clock::new(6, 15), Clock::new(6, -4305));
        assert_eq!(Clock::new(7, 32), Clock::new(-12, -268));
        assert_eq!(Clock::new(18, 7), Clock::new(-54, -11513));
    }
}
