use std::{path::PathBuf, env, thread};

use chrono::Duration;

mod timer;
use timer::Timer;

mod configs;
use configs::load_config;

mod alarm;
use alarm::Alarm;

mod errors;

mod sound;
use sound::play_sound;

pub enum TimeResult {
    Time(chrono::Duration),
    Err,
    Help,
}

enum Show {
    Small,
    Big,
    None
}

//Main function, takes care of reading the arguments, the config, and sending it forward
fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Look into the docs for uses");
        return;
    }
    args.remove(0);
    match load_config() {
        Ok(cfg) => {
            choices(&mut args, cfg.beep_pos);
        }
        Err(_) => {
            panic!("No configs could be loaded");
        }
    }
}

//Parses the arguments taken from the commandline
fn choices(args: &mut Vec<String>, beep_pos: PathBuf) {
    let show = if args[0].contains("s") {
        let pos = args[0].find("s").unwrap();
        args[0].replace_range(pos..pos+1, "");
        Show::Small
    } else if args[0].contains("S") { 
        Show::Big
    } else {
        Show::None
    };
    if args[0].contains("m") {
        let pos = args[0].find("m").unwrap();
        args[0].replace_range(pos..pos+1, "");
        args.insert(args.len() - 1, "-m".into());
    } 
    let duration = match &args[0][..] {
        "-t" | "--timer" => {
            args.remove(0);
            timer(args)
        }
        "-a" | "--alarm" => {
            args.remove(0);
            alarm(args)
        }
        "-h" | "[-]{2}help" => {
            TimeResult::Help
        }
        _ => {
            TimeResult::Help
        }
    };

    match duration {
        TimeResult::Time(time) => {
            match show {
                Show::Small => {
                    for each in 0..time.num_seconds() {
                        print_time(time.num_seconds()-each);
                        thread::sleep(Duration::seconds(1).to_std().unwrap());
                    }
                }
                Show::Big => {

                }
                Show::None => thread::sleep(time.to_std().unwrap())
            }
            play_sound(beep_pos).expect("Something went wrong");
        }
        TimeResult::Err => panic!("Something went wrong"),
        TimeResult::Help => help_text()
    }

    extra_choices(args);
}

fn extra_choices(args: &mut Vec<String>) {
    if args.len() <= 0 {
        return;
    }
    match &args[0][..] {
        "-m" => {
            args.remove(0);
            if args.len() > 0 {
                println!("{}", args.remove(0))
            }
        }
        _ => {}
    }
}

fn print_time(time_seconds: i64) {
    print!("{esc}c", esc = 27 as char);
    let h = time_seconds / (60 * 60);
    let m = time_seconds / 60;
    let s = time_seconds % 60;

    println!("{h}:{m}:{s}");
}

fn help_text() {
    println!("The argument flags are: ");
    println!("-t --timer : Set a timer");
    println!("-a --alarm : Set an alarm");
    println!("-[f]m -m : Set a message");
    println!("-h --help : See this text");
}

fn timer(args: &mut Vec<String>) -> TimeResult {
    if args.len() <= 0 {
        println!("You have to add a time, stupid");
        return TimeResult::Err;
    }
    let mut timer = Timer::new();
    timer.find_time(args)
}

fn alarm(args: &mut Vec<String>) -> TimeResult {
    if args.len() <= 0 {
        println!("You have to add a time, stupid");
        return TimeResult::Err;
    }
    let mut alarm = Alarm::new();
    alarm.set_alarm(args)
}


