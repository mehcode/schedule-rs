use std::collections::HashSet;
use std::str::FromStr;

use nom::IResult;

use chrono::{DateTime, UTC, TimeZone, Datelike, Duration, Timelike};

use parser::{Field, parse};

pub trait Schedule {
    fn next(&self, from: Option<DateTime<UTC>>) -> Option<DateTime<UTC>>;
}

/// A Cron-like schedule that specifies a duty cycle.
#[derive(Debug)]
pub struct CronSchedule {
    /// Second(s) of the minute [0-59]
    seconds: HashSet<u32>,

    /// Minute(s) of the hour [0-59]
    minutes: HashSet<u32>,

    /// Hour(s) [0-23]
    hours: HashSet<u32>,

    /// Day(s) of the Month [1-31]
    days_of_month: HashSet<u32>,

    /// Month(s) [1-12]
    months: HashSet<u32>,

    /// Day(s) of Week [0-6]; 0 = Sunday
    /// If both `day_of_week` and `day_of_month` are specified; the
    /// schedule is run when either event happens.
    days_of_week: HashSet<u32>,
}

impl Schedule for CronSchedule {
    /// Returns the next time this schedule should be ran, greater than the given time.
    fn next(&self, from: Option<DateTime<UTC>>) -> Option<DateTime<UTC>> {
        // Start at the earliest possible time, the next second
        let mut r = from.unwrap_or_else(UTC::now).with_nanosecond(0).unwrap() +
                    Duration::seconds(1);

        loop {
            // If we've gone more than 2 years past _now_; stop
            if (r.year() - UTC::now().year()) >= 2 {
                return None;
            }

            // Find the next applicable month
            while !self.months.is_empty() && !self.months.contains(&r.month()) {
                let mut overflow = false;
                let mut month = r.month() + 1;

                // Handle overflow from 12 to 1
                if month > 12 {
                    overflow = true;
                    month = 1;
                }

                r = UTC.ymd(r.year(), month, 1).and_hms(0, 0, 0);

                // On overflow, restart process
                if overflow {
                    continue;
                }
            }

            // Find the next applicable day
            while !self.days_of_month.is_empty() && !self.days_of_week.is_empty() {
                if !self.days_of_month.is_empty() && self.days_of_month.contains(&r.day()) {
                    break;
                }

                if !self.days_of_week.is_empty() &&
                   self.days_of_week.contains(&(r.weekday() as u32)) {
                    break;
                }

                r = UTC.ymd(r.year(), r.month(), r.day()).and_hms(0, 0, 0) + Duration::hours(24);

                // On overflow, restart process
                if r.day() == 1 {
                    continue;
                }
            }

            // Find the next applicable hour
            while !self.hours.is_empty() && !self.hours.contains(&r.hour()) {
                r = UTC.ymd(r.year(), r.month(), r.day()).and_hms(r.hour(), 0, 0) +
                    Duration::hours(1);

                // On overflow, restart process
                if r.hour() == 0 {
                    continue;
                }
            }

            // Find the next applicable minute
            while !self.minutes.is_empty() && !self.minutes.contains(&r.minute()) {
                r = UTC.ymd(r.year(), r.month(), r.day()).and_hms(r.hour(), r.minute(), 0) +
                    Duration::minutes(1);

                // On overflow, restart process
                if r.minute() == 0 {
                    continue;
                }
            }

            // Find the next applicable second
            while !self.seconds.is_empty() && !self.seconds.contains(&r.second()) {
                r = UTC.ymd(r.year(), r.month(), r.day())
                    .and_hms(r.hour(), r.minute(), r.second()) +
                    Duration::seconds(1);

                // On overflow, restart process
                if r.second() == 0 {
                    continue;
                }
            }

            break;
        }

        Some(r)
    }
}

// TODO: Have nice error messages
impl FromStr for Box<Schedule> {
    type Err = ();

    fn from_str(s: &str) -> Result<Box<Schedule>, ()> {
        // Try to parse a series of fields
        let fields = match parse(s.as_bytes()) {
            IResult::Done(_, fields) => fields,
            IResult::Error(_) |
            IResult::Incomplete(_) => return Err(()),
        };

        // Assert that we have 5-6 fields
        if fields.len() < 5 || fields.len() > 6 {
            return Err(());
        }

        let mut seconds = Vec::new();
        let mut minutes = Vec::new();
        let mut hours = Vec::new();
        let mut days_of_month = Vec::new();
        let mut months = Vec::new();
        let mut days_of_week = Vec::new();

        {
            let mut buckets =
                vec![&mut minutes, &mut hours, &mut days_of_month, &mut months, &mut days_of_week];
            if fields.len() == 6 {
                buckets.insert(0, &mut seconds);
            } else {
                // Using 5-field format; default seconds to {0}
                seconds.push(0);
            }

            for (field, bucket) in fields.iter().zip(buckets.iter_mut()) {
                match *field {
                    Field::Number(number) => {
                        bucket.push(number);
                    }

                    Field::All => {
                        // Empty bucket corresponds to All
                    }

                    Field::Range { start, end } => {
                        for number in start..(end + 1) {
                            bucket.push(number);
                        }
                    }
                }
            }
        }

        // Adjust days-of-week to have 0 as Sunday
        for day_of_week in &mut days_of_week {
            *day_of_week = if *day_of_week == 0 {
                6
            } else {
                *day_of_week - 1
            };
        }

        let s = CronSchedule {
            seconds: seconds.into_iter().collect(),
            minutes: minutes.into_iter().collect(),
            hours: hours.into_iter().collect(),
            days_of_month: days_of_month.into_iter().collect(),
            days_of_week: days_of_week.into_iter().collect(),
            months: months.into_iter().collect(),
        };

        // TODO: Validate input
        // s.validate();

        println!("{:?}", s);

        Ok(Box::new(s))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_standard() {
        let s: Box<Schedule> = "1 2 3 4 *".parse().unwrap();
        let next_at = s.next(None).unwrap();

        assert_eq!(next_at.minute(), 1);
        assert_eq!(next_at.hour(), 2);
        assert_eq!(next_at.day(), 3);
        assert_eq!(next_at.month(), 4);
    }
}
