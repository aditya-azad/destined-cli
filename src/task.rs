use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use fast_float::parse_partial;

use crate::errors::ParsingError;

#[derive(Debug)]
pub struct Task {
    body: String,
    goal: Option<String>,
    due: Option<chrono::DateTime<Local>>,
    timestamp: Option<chrono::DateTime<Local>>,
    tracking: Option<bool>,
    duration: Option<Duration>,
    repeat: Option<char>,
}

impl Task {
    // parses the string to task object with following rules:
    // - can start with '-' but is trimmed off
    // - should start with alpha numeric
    pub fn from_string(task_string: &str) -> Result<Task, ParsingError> {
        let mut t = Task::new();
        let s = task_string.trim().trim_start_matches(char::is_alphanumeric);
        for word in s.split(" ") {
            if word.starts_with("_") {
                if let Ok(ts) = Task::parse_date_time(word) {
                    // parse timestamp
                    t.timestamp = Some(ts);
                } else if let Ok(ts) = Task::parse_date_time(word.trim().trim_start_matches("_due"))
                {
                    // parse due
                    t.due = Some(ts);
                } else if let Ok(rp) = Task::parse_repeat(word) {
                    // parse repeat
                    t.repeat = Some(rp);
                } else if let Ok(tr) = Task::parse_tracking(word) {
                    // parse tracking
                    t.tracking = Some(tr);
                } else if let Ok(dur) = Task::parse_duration(word) {
                    // parse duration
                    t.duration = Some(dur);
                } else {
                    return Err(ParsingError::String(format!(
                        "Error parsing keyword '{}'",
                        word
                    )));
                }
            } else {
                if word != "-" {
                    t.body = t.body + " " + word;
                }
            }
        }
        // parse body
        if t.body.len() == 0 {
            return Err(ParsingError::String(
                "Task body should not be empty".to_string(),
            ));
        }
        t.body = t.body.trim().to_string();
        Ok(t)
    }

    pub fn set_goal(&mut self, val: String) -> bool {
        if val.len() == 0 {
            return false;
        }
        self.goal = Some(val);
        return true;
    }

    fn new() -> Task {
        Task {
            body: "".to_string(),
            goal: None,
            due: None,
            timestamp: None,
            duration: None,
            tracking: None,
            repeat: None,
        }
    }

    // parses repeat info into character. supports following formats:
    // - _RD or _RW or _RM or _RY
    // - case insensitive
    // - returns the value in lower case if matches
    fn parse_repeat(repeat_string: &str) -> Result<char, ParsingError> {
        let repeat_string = repeat_string.to_lowercase();
        let repeat_string = repeat_string.trim().trim_start_matches("_r");
        if repeat_string.is_empty() {
            return Err(ParsingError::String(
                "Repeat body must be 'D', 'W', 'M' or 'Y'".to_string(),
            ));
        } else if ["d", "w", "m", "y"].contains(&repeat_string) {
            return Ok(repeat_string.chars().next().unwrap());
        } else {
            return Err(ParsingError::String(
                "Repeat body must be 'D', 'W', 'M' or 'Y'".to_string(),
            ));
        }
    }

    // parses if tracking or not
    // - _T
    fn parse_tracking(tracking_string: &str) -> Result<bool, ParsingError> {
        let tracking_string = tracking_string.to_lowercase();
        let tracking_string = tracking_string.trim().trim_start_matches("_r");
        if tracking_string.starts_with("_t") {
            return Ok(true);
        }
        Err(ParsingError::String(
            "Repeat body must be 'D', 'W', 'M' or 'Y'".to_string(),
        ))
    }

    // parses string into local datetime. supports following formats
    // _12jan2020
    // _12jan2020_10:23pm
    // _10:30am
    fn parse_date_time(date_time_string: &str) -> Result<chrono::DateTime<Local>, ParsingError> {
        let cleaned_input = date_time_string
            .trim()
            .trim_start_matches('_')
            .to_lowercase();
        let parse_result = if cleaned_input.contains(':') {
            // Format: _12jan2020_10:23pm or _10:30am
            NaiveDateTime::parse_from_str(&cleaned_input, "%d%b%Y_%I:%M%P").or_else(|_| {
                NaiveTime::parse_from_str(&cleaned_input, "%I:%M%P")
                    .map(|t| Local::now().date_naive().and_time(t))
            })
        } else {
            // Format: _12jan2020
            NaiveDate::parse_from_str(&cleaned_input, "%d%b%Y")
                .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
        };
        match parse_result {
            Ok(d) => Ok(Local.from_local_datetime(&d).unwrap()),
            Err(e) => Err(ParsingError::String(format!(
                "Error parsing string '{}' to date time: {}",
                date_time_string, e
            ))),
        }
    }

    // parses duration. supports following formats
    // _for_2h
    // _for_10m
    fn parse_duration(duration_string: &str) -> Result<Duration, ParsingError> {
        let cleaned_input = duration_string.to_lowercase();
        let cleaned_input = cleaned_input.trim().trim_start_matches("_for");
        if cleaned_input.is_empty() {
            return Err(ParsingError::String(format!(
                "Duration must not be empty: {}",
                cleaned_input
            )));
        }
        let splits: Vec<&str> = cleaned_input.split('_').collect();
        if splits.len() != 2 {
            return Err(ParsingError::String(format!(
                "Duration must not be empty: {}",
                cleaned_input
            )));
        }
        let duration_in_units = match parse_partial::<f32, _>(splits[1]) {
            Ok(p) => p.0,
            Err(e) => {
                return Err(ParsingError::String(format!(
                    "Duration '{}' must have a number: {}",
                    cleaned_input, e
                )))
            }
        };
        let duration = match splits[1].chars().last() {
            Some(c) => match c {
                'h' => Duration::new((duration_in_units * 3600.0) as i64, 0),
                'm' => Duration::new((duration_in_units * 60.0) as i64, 0),
                _ => {
                    return Err(ParsingError::String(format!(
                        "Duration '{}' must have a number",
                        cleaned_input
                    )))
                }
            },
            None => {
                return Err(ParsingError::String(format!(
                    "Duration '{}' must have a unit 'h' or 'm'",
                    cleaned_input
                )))
            }
        };
        match duration {
            Some(d) => Ok(d),
            None => {
                return Err(ParsingError::String(
                    "Duration could not be parsed".to_string(),
                ))
            }
        }
    }
}
