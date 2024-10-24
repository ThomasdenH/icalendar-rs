<div align="center">

[![build](https://img.shields.io/github/actions/workflow/status/hoodie/icalendar-rs/ci.yml?branch=main)](https://github.com/hoodie/icalendar-rs/actions?query=workflow%3A"Continuous+Integration")
[![Crates.io](https://img.shields.io/crates/d/icalendar)](https://crates.io/crates/icalendar)
[![contributors](https://img.shields.io/github/contributors/hoodie/icalendar-rs)](https://github.com/hoodie/icalendar-rs/graphs/contributors)
![maintenance](https://img.shields.io/maintenance/yes/2025)

[![version](https://img.shields.io/crates/v/icalendar)](https://crates.io/crates/icalendar/)
[![documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/icalendar/)
[![license](https://img.shields.io/crates/l/icalendar.svg?style=flat)](https://crates.io/crates/icalendar/)

A builder and parser for [`rfc5545`](http://tools.ietf.org/html/rfc5545) iCalendar.

</div>

# iCalendar in Rust

You want to help make this more mature? Please talk to me, Pull Requests and suggestions are very welcome.

## Examples
Below are two examples of how to use this library. See the `examples` directory as well as the documentation for many more.

### Building a new Calendar

Use the builder-pattern to assemble the full calendar or event by event.
Display printing produces the rfc5545 format.

```rust
use icalendar::{Calendar, CalendarDateTime, Class, Component, Event, EventLike, Property, Todo};
use chrono::{Duration, NaiveDate, NaiveTime, Utc};

// let's create a calendar
let my_calendar = Calendar::new()
    .name("example calendar")
    .push(
        // add an event
        Event::new()
            .summary("test event")
            .description("here I have something really important to do")
            .starts(Utc::now())
            .class(Class::Confidential)
            .ends(Utc::now() + Duration::days(1))
            .append_property(
                Property::new("TEST", "FOOBAR")
                    .add_parameter("IMPORTANCE", "very")
                    .add_parameter("DUE", "tomorrow")
                    .done(),
            )
            .done(),
    )
    .push(
        // add a todo
        Todo::new()
            .summary("groceries")
            .description("Buy some milk")
            .done(),
    )
    .push(
        // add an all-day event
        Event::new()
            .all_day(NaiveDate::from_ymd_opt(2016, 3, 15).unwrap())
            .summary("My Birthday")
            .description("Hey, I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik")
            .done(),
    )
    .push(
        // event with utc timezone
        Event::new()
            .starts(CalendarDateTime::from(
                NaiveDate::from_ymd_opt(2024, 10, 24).unwrap()
                    .and_time(NaiveTime::from_hms_opt(20, 10, 00).unwrap())
                    .and_utc()
            ))
            .summary("Birthday Party")
            .description("I'm gonna have a party\nBYOB: Bring your own beer.\nHendrik")
            .done(),
    )
    .done();

println!("{}", my_calendar);

```

### Parsing a Calendar
There is a feature called `"parser"` which allows you to read calendars again like this:

```rust
use std::fs::File;
use std::io::Read;
use icalendar::{Calendar, CalendarComponent, Component};

let mut file = File::open("fixtures/icalendar-rb/event.ics").unwrap();
let mut contents = String::new();
file.read_to_string(&mut contents);
let parsed_calendar: Calendar = contents.parse().unwrap();

for component in &parsed_calendar.components {
    match component {
        CalendarComponent::Event(event) => {
            println!("Event: {}", event.get_summary().unwrap())
        },
        _ => {}
    }
}

```

## Structure
A [`Calendar`] represents a full calendar, which contains multiple [`Component`]s. These may be either [`Event`]s, [`Todo`]s, or [`Venue`]s. Components in turn have [`Property`]s, which may have [`Parameter`]s.

## License

icalendar-rs is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Any help in form of descriptive and friendly [issues](https://github.com/hoodie/icalendar-rs/issues) or comprehensive pull requests are welcome! 

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in icalendar-rs by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
