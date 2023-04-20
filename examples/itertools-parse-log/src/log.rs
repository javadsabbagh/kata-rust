use std::cmp::Ordering;

use chrono::prelude::*;
use regex::Regex;

/* Define custom struct for log entries */
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord)]
pub struct ApacheLogEntry {
    client_id: String,
    timestamp: DateTime<FixedOffset>,
    request: Struing,
    status_code: u8,
    size: u32,
}

/* For comparing lines based on timestamp */
impl PartialOrd for ApacheLogEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

/* For creating log entry struct from each line string */
impl TryFrom<&str> for ApacheLogEntry {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(ApacheLogEntry { ..Default::default() })
    }
}
