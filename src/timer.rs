use super::{errors::*, TimeResult};

use chrono::Duration;

use regex::Regex;

pub struct Timer {
    timer: u64,
    seconds: bool,
    minutes: bool,
    hours: bool,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            timer: 0,
            seconds: false,
            minutes: false,
            hours: false,
        }
    }

    pub fn find_time(&mut self, args: &mut Vec<String>) -> TimeResult {
        let test_regex = Regex::new("[0-9]+[smh]").unwrap();
        while args.len() > 0 {
            if args[0].contains("-") || !test_regex.is_match(&args[0][..]) {
                break;
            }
            match self.get_time(&args.remove(0)) {
                Ok(time) => {
                    self.timer += time;
                }
                Err(_) => println!("Houston, we have a problem"),
            }
        }
        TimeResult::Time(Duration::seconds(self.timer as i64))
    }

    fn get_time(&mut self, incoming_str: &String) -> CountResult<u64> {
        return match incoming_str {
            s if !self.seconds && Regex::new("[0-9]+s").unwrap().is_match(s) => {
                if let Some(s) = incoming_str.find('s') {
                    self.seconds = true;
                    Ok(parse_time(incoming_str, s))
                } else {
                    Err(CountError)
                }
            }
            m if !self.minutes && Regex::new("[0-9]+m").unwrap().is_match(m) => {
                if let Some(s) = incoming_str.find('m') {
                    self.minutes = true;
                    Ok(parse_time(incoming_str, s) * 60)
                } else {
                    Err(CountError)
                }
            }
            h if !self.hours && Regex::new("[0-9]+h").unwrap().is_match(h) => {
                if let Some(s) = incoming_str.find('h') {
                    self.hours = true;
                    Ok(parse_time(incoming_str, s) * 60 * 60)
                } else {
                    Err(CountError)
                }
            }
            _ => Ok(0),
        };
    }
}

fn parse_time(time: &String, position: usize) -> u64 {
    let secs: String = time.chars().take(position).collect();
    if let Ok(add_time) = secs.parse::<u64>() {
        add_time
    } else {
        0
    }
}

#[cfg(test)]
mod timer_test {
    use super::*;

    #[test]
    fn test_seconds() {
        let mut timer = Timer::new();
        assert_eq!(timer.seconds, false);
        assert_eq!(timer.get_time(&"10s".into()).unwrap(), 10);
        assert_eq!(timer.seconds, true);
        assert_ne!(timer.get_time(&"10s".into()).unwrap(), 10);
    }

    #[test]
    fn test_minutes() {
        let mut timer = Timer::new();
        assert_eq!(timer.minutes, false);
        assert_eq!(timer.get_time(&"10m".into()).unwrap(), 10 * 60);
        assert_eq!(timer.minutes, true);
        assert_ne!(timer.get_time(&"10m".into()).unwrap(), 10 * 60);
    }

    #[test]
    fn test_hours() {
        let mut timer = Timer::new();
        assert_eq!(timer.hours, false);
        assert_eq!(timer.get_time(&"10h".into()).unwrap(), 10 * 60 * 60);
        assert_eq!(timer.hours, true);
        assert_ne!(timer.get_time(&"10h".into()).unwrap(), 10 * 60 * 60);
    }

    #[test]
    fn test_time() {
        let mut timer = Timer::new();
        let mut times: Vec<String> = vec![
            String::from("10s"),
            String::from("10m"),
            String::from("10h"),
        ];
        assert_eq!(timer.seconds, false);
        assert_eq!(timer.minutes, false);
        assert_eq!(timer.hours, false);
        match timer.find_time(&mut times) {
            TimeResult::Time(time) => {
                assert_eq!(timer.seconds, true);
                assert_eq!(timer.minutes, true);
                assert_eq!(timer.hours, true);
                assert_eq!(times.len(), 0);
                assert_eq!(time, Duration::seconds(36610));
            }
            _ => {}
        }
    }
}
