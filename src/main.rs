use std::{env, thread, time::{Duration, Instant}, io::stdout, fs, collections::HashMap};

mod timer;
use timer::Timer;

mod configs;
use configs::{load_config, MyConfig};

mod alarm;
use alarm::Alarm;

mod errors;

mod sound;
use sound::play_sound;

mod console;
use console::help_text;

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
            choices(&mut args, cfg);
        }
        Err(_) => {
            panic!("No configs could be loaded");
        }
    }
}

//Parses the arguments taken from the commandline
fn choices(args: &mut Vec<String>, cfg: MyConfig) {
    message_handle(args);
    let show = show_handle(args);
    println!("{args:?}");
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

    time_handle(duration, show, cfg);

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

fn time_handle(duration: TimeResult, show: Show, cfg: MyConfig) {
    match duration {
        TimeResult::Time(time) => {
            match show {
                Show::Small => {
                    let stdout = stdout();
                    let mut console = console::Console::new(stdout);
                    //Approximately 0.0000017/s in fault
                    for each in (0..time.num_seconds()).rev() {
                        let (h, m, s) = get_time(each);
                        console.write_line(format!("{h}:{m}:{s}"));
                        thread::sleep(Duration::from_micros(999780));
                    }
                }
                Show::Big => {
                    let stdout = stdout();
                    let mut console = console::Console::new(stdout);
                    let art = get_art(cfg.digit_pos);
                    for each in (0..time.num_seconds()).rev() {
                        console.write_ascii(print_big_time(each, &art));
                        thread::sleep(Duration::from_micros(999780));
                    }
                }
                Show::None => {
                    let tim = Instant::now();
                    thread::sleep(time.to_std().unwrap());
                    println!("{:?}", tim.elapsed());

                }
            }
            play_sound(cfg.beep_pos).expect("Something went wrong");
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

//Code gotten from AntonMarklund00/countdown-timer-rust
fn print_big_time(time_seconds: i64, ascii_art: &HashMap<String, String>) -> Vec<String> {
    let mut time_str_vec: Vec<String> = Vec::new();
    let (h, m, s) = get_time(time_seconds);

    for digit in format!("{h}:{m}:{s}").to_string().split("") {
        if digit =="" { continue }

        let big_string: String = ascii_art[digit].clone();

        if time_str_vec.len() < 1 {
            time_str_vec = vec!["".into(); big_string.split("\n").count()];
        }

        for (j, y) in big_string.split("\n").enumerate() {
            time_str_vec[j] = time_str_vec[j].to_string() + y + &" ".repeat(10 - y.len());
        }
    }

    time_str_vec
}

fn get_art(digit_pos: String) -> HashMap<String, String> {
    let mut big_vec = HashMap::new();

    for each in 0..10 {
        big_vec.insert(format!("{each}"), "".into());
    }

    big_vec.insert(":".into(), "".into());
    for each in big_vec.clone().keys() {
        println!("{}", digit_pos.clone() + &format!("/{each}.txt")[..]);
        *big_vec.get_mut(each).unwrap() = fs::read_to_string(digit_pos.clone() + &format!("/{each}.txt")[..]).expect("");
    }
    big_vec
}

fn get_time(time_seconds: i64) -> (i64, i64, i64) {
    (time_seconds / (60 * 60), (time_seconds / 60) % 60, time_seconds % 60)
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


