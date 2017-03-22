use schedule::{Schedule, TryIntoSchedule};
use chrono::{DateTime, UTC};
use error::*;

pub struct Job {
    pub function: Box<FnMut() + Send + Sync>,
    pub name: Option<String>,
    pub schedule: Option<Schedule>,
    pub remaining: Option<usize>,
    pub next_run_at: Option<DateTime<UTC>>,
    pub last_run_at: Option<DateTime<UTC>>,
    pub is_active: bool,
}

impl Job {
    pub fn new<F: FnMut() + Send + Sync>(function: F) -> Self
        where F: 'static
    {
        Job {
            function: Box::new(function),
            name: None,
            remaining: None,
            schedule: None,
            next_run_at: None,
            last_run_at: None,
            is_active: true,
        }
    }

    /// Returns true if this job is pending execution.
    pub fn is_pending(&self) -> bool {
        // Check if paused
        if !self.is_active {
            return false;
        }

        // Check if we have a limit
        if let Some(rem) = self.remaining {
            if rem == 0 {
                return false;
            }
        }

        // Check if NOW is on or after next_run_at
        if let Some(next_run_at) = self.next_run_at {
            UTC::now() >= next_run_at
        } else {
            false
        }
    }

    /// Re-schedule the job.
    pub fn schedule<S>(&mut self, s: S) -> Result<()>
        where S: TryIntoSchedule
    {
        // Parse a new schedule
        self.schedule = Some(s.try_into_schedule()?);

        // Reset the remaining count
        self.remaining = None;

        // Determine the next time it should run
        self.next_run_at = if let Some(ref schedule) = self.schedule {
            schedule.next(self.last_run_at)
        } else {
            None
        };

        Ok(())
    }

    /// Run the job immediately and re-schedule it.
    pub fn run(&mut self) {
        // Execute the job function
        (self.function)();

        // Decrement remaining if set
        if let Some(ref mut rem) = self.remaining {
            if *rem > 0 {
                *rem -= 1;
            }
        }

        // Save the last time this ran
        self.last_run_at = Some(UTC::now());

        // Determine the next time it should run
        self.next_run_at = if let Some(ref schedule) = self.schedule {
            schedule.next(self.last_run_at)
        } else {
            None
        };
    }
}
