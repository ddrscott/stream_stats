use std::{env};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, stdin, Write};
use std::string::{String};
use std::sync::{Arc, Mutex};
use std::time::{Duration};
use std::thread::{self, sleep};

mod stats;
use stats::{Stats};

static READ_BUF_SIZE : usize = 1024 * 1024;
static UPDATE_INTERVAL_MS : u64 = 100;
static OUTPUT : &str = "/dev/tty";

fn main() {
    let stdout = io::stdout();
    let mut reader = BufReader::with_capacity(READ_BUF_SIZE, file_or_stdin());
    let mut writer = BufWriter::new(stdout.lock());
    let mut buffer = String::new();

    let stats = Arc::new(Stats::new());
    let output = Arc::new(Mutex::new(OpenOptions::new().write(true).append(true).open(OUTPUT).unwrap()));

    // Setup a thread to render stats periodically.
    let stats_clone = stats.clone();
    let output_clone = output.clone();
    thread::spawn(move || {
        loop {
            sleep(Duration::from_millis(UPDATE_INTERVAL_MS));
            write!(&mut output_clone.lock().unwrap(), "{}", &stats_clone)
                .expect("Could not write to output!");
        }
    });

    while reader.read_line(&mut buffer).unwrap() > 0 {
        writer.write(buffer.as_bytes()).unwrap();
        stats.add(&buffer);
        buffer.clear();
    }
    writeln!(output.lock().unwrap(), "{}", &stats)
        .expect("Could not write to output!");
}

/// Thanks: https://www.reddit.com/r/rust/comments/32rjdd/reading_from_a_file_or_stdin_based_on_command/
fn file_or_stdin() -> Box<io::Read> {
    let path = env::args().nth(1).unwrap_or(String::from("-"));
    match path.as_ref() {
        "-" => Box::new(stdin()),
        _   => Box::new(File::open(path).unwrap())
    }
}
