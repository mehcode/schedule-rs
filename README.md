# schedule-rs
![Rust](https://img.shields.io/badge/rust-nightly-red.svg)
[![Crates.io](https://img.shields.io/crates/d/schedule.svg)](https://crates.io/crates/schedule)
[![Docs.rs](https://docs.rs/schedule/badge.svg)](https://docs.rs/schedule)
[![IRC](https://img.shields.io/badge/chat-%23schedule-yellow.svg)](https://kiwiirc.com/client/irc.mozilla.org/#schedule)
> An in-process scheduler for periodic jobs. Schedule lets you run Rust functions on a cron-like schedule.

## Install

```toml
[dependencies]
schedule = { git = "https://github.com/mehcode/schedule-rs" }
```

## Usage

```rust
extern crate schedule;
extern crate chrono;

use schedule::Agenda;
use chrono::UTC;

fn main() {
    let mut a = Agenda::new();

    // Run every second
    a.add(|| {
        println!("at second     :: {}", UTC::now());
    }).schedule("* * * * * *").unwrap();

    // Run every minute
    a.add(|| {
        println!("at minute     :: {}", UTC::now());
    }).schedule("0 * * * * *").unwrap();

    // Run every hour
    a.add(|| {
        println!("at hour       :: {}", UTC::now());
    }).schedule("0 0 * * * *").unwrap();

    // Check and run pending jobs in agenda every 500 milliseconds
    loop {
        a.run_pending();

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
```

## License

Schedule is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
