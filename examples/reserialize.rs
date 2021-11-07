use std::{env::args, fs::read_to_string};

use icalendar::parse::{normalize, read_calendar};

fn print_with_lines(content: &str) {
    println!(
        "{}",
        content
            .lines()
            .enumerate()
            .map(|(num, content)| format!("{:4}. {}\n", num + 1, content))
            .collect::<String>()
    );
}

fn main() {
    if let Some(sample) = args().nth(1).map(read_to_string) {
        let normalized = normalize(&sample.unwrap());
        print_with_lines(&normalized);

        let calendar = match read_calendar(&normalized) {
            Ok(calendar) => calendar,
            Err(error) => {
                println!("{}", error);
                return;
            }
        };

        let calendar = calendar.to_owned();

        print_with_lines(&calendar.to_string());
    }
}
