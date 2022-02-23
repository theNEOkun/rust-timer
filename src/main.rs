use std::{path::PathBuf, env, thread, time::{Duration, Instant}, io::{self, Write, StdoutLock}};

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
    let show = show_handle(args);
    message_handle(args);
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

    time_handle(duration, show, beep_pos);

    extra_choices(args);
}

//Handles the show-flag
fn show_handle(args: &mut Vec<String>) -> Show {
    return if args[0].contains("s") {
        let pos = args[0].find("s").unwrap();
        args[0].replace_range(pos..pos+1, "");
        Show::Small
    } else if args[0].contains("S") { 
        let pos = args[0].find("S").unwrap();
        args[0].replace_range(pos..pos+1, "");
        Show::Big
    } else {
        Show::None
    }
}

//Handles the message-flag
fn message_handle(args: &mut Vec<String>) {
    if args[0].contains("m") {
        let pos = args[0].find("m").unwrap();
        args[0].replace_range(pos..pos+1, "");
        args.insert(args.len() - 1, "-m".into());
    }
}

struct Console<'a> {
    handle: StdoutLock<'a>
}

fn time_handle(duration: TimeResult, show: Show, beep_pos: PathBuf) {
    match duration {
        TimeResult::Time(time) => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            match show {
                Show::Small => {
                    let tim = Instant::now();
                    for each in (0..time.num_seconds()).rev() {
                        print_time(each, &mut handle);
                        thread::sleep(Duration::from_micros(999880));
                    }
                    writeln!(handle, "{:?}", tim.elapsed());
                }
                Show::Big => {
                    for each in (0..time.num_seconds()).rev() {
                        print_big_time(each, &mut handle);
                        thread::sleep(Duration::from_secs(1));
                    }
                }
                Show::None => {

                    let tim = Instant::now();
                    thread::sleep(time.to_std().unwrap());
                    println!("{:?}", tim.elapsed());

                }
            }
            play_sound(beep_pos).expect("Something went wrong");
        }
        TimeResult::Err => panic!("Something went wrong"),
        TimeResult::Help => help_text()
    }
}

//Handles the output of the message-flag
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

fn print_time(time_seconds: i64, handle: &mut StdoutLock) {
    let (h, m, s) = get_time(time_seconds);

    writeln!(handle, "{esc}c{h}:{m}:{s}", esc = 27 as char).expect("Time couldn't be displayed");
}

fn print_big_time(time_seconds: i64, handle: &mut StdoutLock) {
    print!("{esc}c", esc = 27 as char);
    let (h, m, s) = get_time(time_seconds);
}

fn get_time(time_seconds: i64) -> (i64, i64, i64) {
    (time_seconds / (60 * 60), time_seconds / 60, time_seconds % 60)
}

fn help_text() {
    println!("The argument flags are: ");
    println!("-t --timer : Set a timer");
    println!("-a --alarm : Set an alarm");
    println!("-[f]m -m : Set a message");
    println!("-h --help : See this text");
    println!("adding s to either -t or -a displays the countdown");
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


