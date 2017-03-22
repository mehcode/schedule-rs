extern crate schedule;
extern crate chrono;

use std::{thread, time};
use chrono::*;

fn cron_1() {
    println!("{}: Cron (#1)", UTC::now());
}

fn cron_2() {
    println!("{}: Cron (#2)", UTC::now());
}

fn cron_3() {
    println!("{}: Cron (#3)", UTC::now());
}

fn main() {
    // Create new, empty agenda
    let mut a = schedule::Agenda::new();

    // Schedule a job to run every second
    a.add(cron_1).schedule("* * * * * *").unwrap();

    // Schedule a job to run every 10th second
    a.add(cron_2).schedule("0,10,20,30,40,50 * * * * *").unwrap();

    // Schedule a job to run every minute
    a.add(cron_3).schedule("0 * * * * *").unwrap();

    loop {
        // Execute pending jobs
        a.run_pending();

        // Sleep for 500ms
        thread::sleep(time::Duration::from_millis(500));
    }
}
