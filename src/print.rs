use std::io::{
    BufWriter,
    Write,
    Stdout
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

    pub fn write_line(&mut self, line: String) {


    writeln!(handle, line).expect("Time couldn't be displayed");
    handle.flush();
    }

    pub fn write_ascii(&mut self, ascii: Vec<String>) {

    }
}
