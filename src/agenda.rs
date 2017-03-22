use job::Job;
use schedule::TryIntoSchedule;
use error::*;

#[derive(Default)]
pub struct Agenda {
    jobs: Vec<Job>,
}

impl Agenda {
    pub fn new() -> Self {
        Agenda::default()
    }

    /// Returns true if the Agenda contains no jobs.
    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }

    /// Returns the number of jobs in the Agenda.
    pub fn len(&self) -> usize {
        self.jobs.len()
    }

    /// Clear the Agenda, removing all jobs.
    pub fn clear(&mut self) {
        self.jobs.clear()
    }

    /// Returns true if there is at least one job pending.
    pub fn is_pending(&self) -> bool {
        for job in &self.jobs {
            if job.is_pending() {
                return true;
            }
        }

        false
    }

    /// Run pending jobs in the Agenda.
    pub fn run_pending(&mut self) {
        for job in &mut self.jobs {
            if job.is_pending() {
                job.run();
            }
        }
    }

    /// Adds a job to the agenda.
    pub fn add<'a, F: FnMut() + Send + Sync>(&'a mut self, function: F) -> JobBuilder<'a>
        where F: 'static
    {
        self.jobs.push(Job::new(function));

        let index = self.jobs.len() - 1;

        JobBuilder {
            agenda: self,
            index: index,
        }
    }

    pub fn get<'a>(&'a mut self, name: &str) -> Option<JobOperator<'a>> {
        let mut index: Option<usize> = None;

        for (i, j) in self.jobs.iter().enumerate() {
            if let Some(ref job_name) = j.name {
                if job_name == name {
                    index = Some(i);
                    break;
                }
            }
        }

        index.map(move |index| {
                      JobOperator {
                          index: index,
                          agenda: self,
                      }
                  })
    }
}

/// View into an Agenda used to operate on a Job. Returned from `Agenda::get`.
pub struct JobOperator<'a> {
    agenda: &'a mut Agenda,
    index: usize,
}

impl<'a> JobOperator<'a> {
    // Schedule this job.
    pub fn schedule<S>(&mut self, s: S) -> Result<&mut JobOperator<'a>>
        where S: TryIntoSchedule
    {
        self.agenda.jobs[self.index].schedule(s)?;

        Ok(self)
    }

    /// Pause the evaluation of the Job's schedule.
    pub fn pause(&mut self) -> &mut JobOperator<'a> {
        self.agenda.jobs[self.index].is_active = false;
        self
    }

    /// Resume the evaluation of the Job's schedule.
    pub fn resume(&mut self) -> &mut JobOperator<'a> {
        self.agenda.jobs[self.index].is_active = true;
        self
    }

    /// Remove this Job from the agenda.
    pub fn remove(self) {
        self.agenda.jobs.remove(self.index);
    }
}

/// View into an Agenda used to build a Job. Returned from `Agenda::add`.
pub struct JobBuilder<'a> {
    agenda: &'a mut Agenda,
    index: usize,
}

impl<'a> JobBuilder<'a> {
    /// Define a name for this job.
    pub fn with_name(&mut self, name: &str) -> &mut JobBuilder<'a> {
        self.agenda.jobs[self.index].name = Some(name.into());

        self
    }

    // Schedule this job.
    pub fn schedule<S>(self, s: S) -> Result<JobOperator<'a>>
        where S: TryIntoSchedule
    {
        self.agenda.jobs[self.index].schedule(s)?;

        Ok(JobOperator {
               agenda: self.agenda,
               index: self.index,
           })
    }
}
