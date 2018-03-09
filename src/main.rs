use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, stdin, Write};
use std::string::{String};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use std::thread::{self, sleep};

static READ_BUF_SIZE : usize = 1024 * 1024;
static UPDATE_INTERVAL_MS : u64 = 100;
static OUTPUT : &str = "/dev/tty";

fn main() {
    // Thanks: https://www.reddit.com/r/rust/comments/32rjdd/reading_from_a_file_or_stdin_based_on_command/
    let path = env::args()
        .nth(1)
        .unwrap_or(String::from("-"));
    let input: Box<io::Read> = match path.as_ref() {
        "-" => Box::new(stdin()),
        _   => Box::new(File::open(path).unwrap())
    };

    let stdout = io::stdout();
    let mut reader = BufReader::with_capacity(READ_BUF_SIZE, input);
    let mut writer = BufWriter::new(stdout.lock());
    let mut buffer = String::new();

    let num_lines = Arc::new(AtomicUsize::new(0));
    let num_bytes = Arc::new(AtomicUsize::new(0));

    let started : Instant = Instant::now();
    let output = Arc::new(Mutex::new(OpenOptions::new().write(true).append(true).open(OUTPUT).unwrap()));

    let (clone_lines, clone_bytes, clone_output) = (num_lines.clone(), num_bytes.clone(), output.clone());
    thread::spawn(move || {
        loop {
            sleep(Duration::from_millis(UPDATE_INTERVAL_MS));
            render(&mut clone_output.lock().unwrap(), &started, clone_lines.load(Ordering::Relaxed), clone_bytes.load(Ordering::Relaxed));
        }
    });

    while reader.read_line(&mut buffer).unwrap() > 0 {
        writer.write(buffer.as_bytes()).unwrap();
        num_lines.fetch_add(1, Ordering::Relaxed);
        num_bytes.fetch_add(buffer.len(), Ordering::Relaxed);
        buffer.clear();
    }
    render(&mut output.lock().unwrap(), &started, num_lines.load(Ordering::Relaxed), num_bytes.load(Ordering::Relaxed));
    output.lock().unwrap().write(b"\n").unwrap();
}

fn render(output: &mut File, started: &Instant, num_lines: usize, num_bytes: usize) {
    let elapsed = started.elapsed();
    let seconds : f64 = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
    if seconds == 0.0 {
        return;
    }
    let kb = num_bytes as f64 / 1024 as f64;
    let kb_per_sec = kb / seconds;
    let lines_per_sec = num_lines as f64 / seconds;
    output.write_all(
        format!(
            "\x1B[1G\x1B[2K {:.1} sec | {:.0} kb [ {:.1}/s ] | {} lines [ {:.0}/s ]",
            seconds,
            kb, kb_per_sec,
            num_lines, lines_per_sec
            ).as_bytes()
        ).unwrap();
}
