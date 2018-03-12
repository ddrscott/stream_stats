use std::{env};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::string::{String};
use std::sync::{Arc, Mutex};
use std::time::{Duration};
use std::thread::{self, sleep};

mod stats;
use stats::{Stats};

static READ_BUF_SIZE : usize = 1024 * 1024;
static UPDATE_INTERVAL_MS : u64 = 100;
static OUTPUT_TTY : &str = "/dev/tty";

fn main() {
    let stats = Stats::new();
    let stats = Arc::new(stats);
    let tty = OpenOptions::new()
        .write(true)
        .append(true)
        .open(OUTPUT_TTY)
        .expect("Cannot open tty for writing!");
    let tty = Arc::new(Mutex::new(tty));

    // Setup a thread to render stats periodically.
    let stats_clone = stats.clone();
    let output_clone = tty.clone();
    thread::spawn(move || {
        loop {
            sleep(Duration::from_millis(UPDATE_INTERVAL_MS));
            write!(&mut output_clone.lock().unwrap(), "{}", &stats_clone)
                .expect("Could not write to tty!");
        }
    });

    let mut reader = BufReader::with_capacity(READ_BUF_SIZE, file_or_stdin());
    let mut writer = BufWriter::new(io::stdout());
    let mut buffer = vec![];

    while reader.read_until(b'\n', &mut buffer).unwrap() > 0 {
        writer.write(&buffer).unwrap();
        stats.add(&buffer);
        buffer.clear();
    }
    writer.flush().unwrap();

    // write file stats
    writeln!(tty.lock().unwrap(), "{}", &stats)
        .expect("Could not write to tty!");
}

/// Thanks: https://www.reddit.com/r/rust/comments/32rjdd/reading_from_a_file_or_stdin_based_on_command/
fn file_or_stdin() -> Box<io::Read> {
    let path = env::args().nth(1).unwrap_or(String::from("-"));
    match path.as_ref() {
        "-" => Box::new(io::stdin()),
        _   => Box::new(File::open(path).unwrap())
    }
}
