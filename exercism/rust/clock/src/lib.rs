use std::fmt;

#[derive(Debug,PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { hours: hours, minutes : 0 }.add_minutes(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut hours = self.hours;
        let mut minutes = self.minutes + minutes;

        while minutes < 0 { minutes += 60; hours -= 1; };
        while hours < 0 { hours += 24; };
        
        hours += (minutes - minutes % 60) / 60;
        minutes = minutes % 60;
        hours = hours % 24;

        Clock { hours: hours, minutes: minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
