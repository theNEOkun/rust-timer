use chrono::{self, Duration, NaiveDateTime, NaiveTime};
use regex::Regex;

use super::{errors::*, TimeResult};

pub struct Alarm {
    time: u64,
}

impl Alarm {
    pub fn new() -> Self {
        Alarm { time: 0 }
    }

    pub fn set_alarm(&mut self, args: &mut Vec<String>) -> TimeResult {
        if let Ok(times) = self.check_time(&args.remove(0)) {
            match self.parse_time(times) {
                Ok(_) => TimeResult::Time(Duration::seconds(self.time as i64)),
                Err(_) => {
                    panic!("Could not parse time")
                }
            }
        } else {
            TimeResult::Err
        }
    }

    fn check_time(&mut self, time: &String) -> TimeParseResult<Vec<String>> {
        if Regex::new("[0-9]|[0-9]1[0-9]2[0-3]:[0-5][0-9]")
            .unwrap()
            .is_match(&time[..])
            || Regex::new("[0-9]|[0-9]1[0-9]2[0-3]:[0-5][0-9]:[0-5][0-9]")
                .unwrap()
                .is_match(&time[..])
        {
            return Ok(time.split(":").map(|s| s.into()).collect());
        }
        Err(ParseError)
    }

    fn parse_time(&mut self, time: Vec<String>) -> TimeParseResult<bool> {
        let now = chrono::prelude::Local::now().naive_local();
        if time.len() == 2 {
            let full_time = get_naive_time(&time[0][..], &time[1][..], "0", now);
            let durr = find_diff(full_time, now);
            self.time += durr.num_seconds() as u64;
            return Ok(true);
        }
        if time.len() == 3 {
            let full_time = get_naive_time(&time[0][..], &time[1][..], &time[2][..], now);
            let durr = find_diff(full_time, now);
            self.time += durr.num_seconds() as u64;
            return Ok(true);
        }
        Ok(true)
    }
}

fn parse_time(time: &str) -> u32 {
    if let Ok(add_time) = time.parse::<u32>() {
        add_time
    } else {
        0
    }
}

fn find_diff(time: NaiveDateTime, now: NaiveDateTime) -> chrono::Duration {
    if time < now {
        return now - time;
    }
    return time - now;
}

fn get_naive_time(hours: &str, minutes: &str, seconds: &str, now: NaiveDateTime) -> NaiveDateTime {
    let time = NaiveTime::from_hms(parse_time(hours), parse_time(minutes), parse_time(seconds));
    let date = now.date()
        + if time < now.time() {
            chrono::Duration::days(1)
        } else {
            chrono::Duration::days(0)
        };
    NaiveDateTime::new(date, time)
}

#[cfg(test)]
mod alarm_tests {
    use super::*;

    #[test]
    fn test_time_parse() {
        let mut alarm = Alarm::new();
        let now = NaiveDateTime::parse_from_str("2022-02-21 10:09", "%Y-%m-%d %H:%M").unwrap();
        let times: Vec<String> = alarm.check_time(&"10:10".into()).unwrap();
        assert_eq!(times, vec!["10".to_string(), "10".to_string()]);
        let time = get_naive_time(&times[0][..], &times[1][..], "0", now);
        assert_eq!(time.time(), NaiveTime::from_hms(10, 10, 0));
    }
}
