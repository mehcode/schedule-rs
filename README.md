# schedule-rs
> An in-process scheduler for periodic jobs. Schedule lets you run Rust functions on a cron-like schedule.

## Install

```toml
[dependencies]
schedule = "0.1"
```

## Usage

```rust
extern crate schedule;
extern crate chrono;

use schedule::{Agenda, Job};
use chrono::UTC;

fn main() {
    let mut a = Agenda::new();

    // Run every second
    a.add(Job::new(|| {
        println!("at second     :: {}", UTC::now());
    }, "* * * * * *".parse().unwrap()));

    // Run every minute
    a.add(Job::new(|| {
        println!("at minute     :: {}", UTC::now());
    }, "* * * * *".parse().unwrap()));

    // Run every hour
    a.add(Job::new(|| {
        println!("at hour       :: {}", UTC::now());
    }, "0 * * * *".parse().unwrap()));

    // Check and run pending jobs in agenda every 500 milliseconds
    loop {
        a.run_pending();

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```

## License

config-rs is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
