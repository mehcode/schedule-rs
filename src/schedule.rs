use std::time;
use cron;
use error::*;
use chrono::{DateTime, UTC, Duration};

pub enum Schedule {
    /// Set to execute on set time periods
    Periodic(cron::Schedule),

    /// Set to execute exactly `duration` away from the previous execution
    Interval(time::Duration),
}

impl Schedule {
    // Determine the next time we should execute (from a reference point)
    pub fn next(&self, after: Option<DateTime<UTC>>) -> Option<DateTime<UTC>> {
        let after = after.unwrap_or_else(UTC::now);

        match *self {
            Schedule::Periodic(ref cs) => cs.after(&after).next(),

            Schedule::Interval(ref duration) => {
                let ch_duration = match Duration::from_std(*duration) {
                    Ok(value) => value,
                    Err(_) => {
                        return None;
                    }
                };

                Some(after + ch_duration)
            }
        }
    }
}

// TODO(@rust): Replace with TryFrom impl when stable
// https://github.com/rust-lang/rust/issues/33417
pub trait TryIntoSchedule {
    fn try_into_schedule(self) -> Result<Schedule>;
}

impl<'a> TryIntoSchedule for &'a str {
    fn try_into_schedule(self) -> Result<Schedule> {
        Ok(Schedule::Periodic(self.parse()?))
    }
}

impl TryIntoSchedule for time::Duration {
    fn try_into_schedule(self) -> Result<Schedule> {
        Ok(Schedule::Interval(self))
    }
}

impl TryIntoSchedule for Duration {
    fn try_into_schedule(self) -> Result<Schedule> {
        Ok(Schedule::Interval(self.to_std()?))
    }
}
