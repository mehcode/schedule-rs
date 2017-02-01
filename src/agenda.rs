use std::vec::Vec;

use job::Job;

#[derive(Default)]
pub struct Agenda<'a> {
    jobs: Vec<Job<'a>>,
}

impl<'a> Agenda<'a> {
    pub fn add(&mut self, job: Job<'a>) {
        // Add new job
        self.jobs.push(job);

        // Re-sort job list
        self.jobs.sort_by_key(|j| j.next_run_at);
    }

    pub fn run_pending(&mut self) {
        for job in &mut self.jobs {
            if job.is_ready() {
                job.run();
            } else {
                // The jobs array is sorted so the first non-ready job we hit
                // means we're at the end of what we care
                break;
            }
        }

        // Re-sort job list
        self.jobs.sort_by_key(|j| j.next_run_at);
    }
}
