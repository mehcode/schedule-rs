extern crate schedule;
extern crate chrono;

use std::{thread, time};
use chrono::*;

fn hello_world_5() {
    println!("{}: Hello World (every 5 seconds)!", UTC::now());
}

fn main() {
    // Create new, empty agenda
    let mut a = schedule::Agenda::new();

    // Add a job from a closure, scheduled to run every 2 seconds
    a.add(|| {
                 println!("{}: Hello World (every 2 seconds)!", UTC::now());
             })
        .schedule(Duration::seconds(2))
        .unwrap();

    // Add a job from a function and give it a name
    a.add(hello_world_5).with_name("hello-world");

    // Schedule that job to run every 5 seconds
    a.get("hello-world")
        .unwrap()
        .schedule(Duration::seconds(5))
        .unwrap();

    loop {
        // Execute pending jobs
        a.run_pending();

        // Sleep for 500ms
        thread::sleep(time::Duration::from_millis(500));
    }
}
