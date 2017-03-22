use cron;
use time;

error_chain! {
    links {
        Cron(cron::error::Error, cron::error::ErrorKind);
    }

    foreign_links {
        TimeOutOfRange(time::OutOfRangeError);
    }
}
