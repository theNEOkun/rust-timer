use std::{
    io::{
        BufWriter,
        Write,
        Stdout
    },
    time::Instant
};

pub struct Console {
    handle: BufWriter<Stdout>
}

impl Console {
    pub fn new(stdout: Stdout) -> Self {
        Console {
            handle: BufWriter::new(stdout)
        }
    }

    //Meant to print line by line
    pub fn write_line(&mut self, line: String) {
        writeln!(self.handle, "{esc}c{line}", esc=27 as char).expect("Message did not get through");
        self.handle.flush();
    }

    //Meant to print the ascii chars
    pub fn write_ascii(&mut self, ascii: Vec<String>) {
        write!(self.handle, "{esc}c", esc=27 as char);
        for each in ascii {
            writeln!(self.handle, "{each}").expect("Message did not get through");
        }
        self.handle.flush();
    }
}

//Prints the help-text
pub fn help_text() {
    println!("The argument flags are: ");
    println!("-t --timer : Set a timer");
    println!("-a --alarm : Set an alarm");
    println!("-[f]m -m : Set a message");
    println!("-h --help : See this text");
    println!("adding s to either -t or -a displays the countdown");
}

//Simply used to time it, and is not really intended for active use
pub fn take_time<F>(mut callback: F)
where
    F: FnMut(),
{
    let tim = Instant::now();
    callback();
    println!("{:?}", tim.elapsed());
}
