use crate::calendar::CalendarElement;

use super::{read_calendar, unfold, Component};
use core::fmt::{self, Write};
use std::str::FromStr;

/// Helpertype for reserialization
#[derive(Clone, Debug)]
pub struct Calendar<'a> {
    pub components: Vec<Component<'a>>,
}

impl Calendar<'_> {
    /// Writes `Component` into a `Writer` using `std::fmt`.
    pub fn fmt_write<W: Write>(&self, out: &mut W) -> Result<(), fmt::Error> {
        for component in &self.components {
            component.fmt_write(out)?;
        }
        Ok(())
    }
    /// Prints to stdout
    pub fn print(&self) -> Result<(), fmt::Error> {
        print_crlf!("{}", self);
        Ok(())
    }
}

impl fmt::Display for Calendar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_write(f)
    }
}

impl From<Calendar<'_>> for crate::Calendar {
    fn from(parsed: Calendar) -> Self {
        parsed.components.into()
    }
}

impl<'a> From<Vec<Component<'a>>> for crate::Calendar {
    fn from(mut components: Vec<Component<'a>>) -> Self {
        let root_is_calendar = components
            .get(0)
            .map(|first_root| first_root.name == "VCALENDAR")
            .unwrap_or(false);

        let components: Vec<Component<'a>> = if root_is_calendar {
            components.swap_remove(0).components
        } else {
            components
        };
        components
            .into_iter()
            .map(|c: Component<'a>| {
                let elem: CalendarElement = c.into();
                elem
            })
            .collect()
    }
}

impl<'a> FromStr for crate::Calendar {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let from_parsed = crate::Calendar::from(read_calendar(&unfold(s))?);
        Ok(from_parsed)
    }
}