use chrono::{DateTime, UTC};

use schedule::Schedule;

pub struct Job<'a> {
    /// Schedule used to determine the next run
    schedule: Box<Schedule + 'a>,

    /// DateTime of the last run
    last_run_at: Option<DateTime<UTC>>,

    /// DateTime of the next run
    pub next_run_at: Option<DateTime<UTC>>,

    /// Function to run
    function: Box<(FnMut() -> ()) + 'a>,
}

impl<'a> Job<'a> {
    pub fn new<T>(function: T, schedule: Box<Schedule>) -> Job<'a>
        where T: 'a,
              T: FnMut() -> ()
    {
        Job {
            next_run_at: schedule.next(None),
            last_run_at: None,
            schedule: schedule,
            function: Box::new(function),
        }
    }

    /// Return `true` if the job should be run now.
    pub fn is_ready(&self) -> bool {
        if let Some(next_run_at) = self.next_run_at {
            UTC::now() >= next_run_at
        } else {
            false
        }
    }

    /// Run the job immediately and reschedule it.
    pub fn run(&mut self) {
        self.last_run_at = Some(UTC::now());

        (self.function)();

        self.next_run_at = self.schedule.next(self.last_run_at);
    }
}
