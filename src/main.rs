use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, stdin, Write};
use std::string::{String};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use std::thread::{self, sleep};

static READ_BUF_SIZE : usize = 1024 * 1024;
static UPDATE_INTERVAL_MS : u64 = 500;
static OUTPUT: &'static str = "/dev/tty";

// Any data from `stdin` is passed to `stdout`.
fn main() {
    let stdin = stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::with_capacity(READ_BUF_SIZE, stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    let mut buffer = String::new();

    let num_lines = Arc::new(AtomicUsize::new(0));
    let num_bytes = Arc::new(AtomicUsize::new(0));

    let started : Instant = Instant::now();
    let output = Arc::new(Mutex::new(OpenOptions::new().write(true).append(true).open(OUTPUT).unwrap()));

    {
        let (num_lines, num_bytes, output) = (num_lines.clone(), num_bytes.clone(), output.clone());
        thread::spawn(move || {
            loop {
                sleep(Duration::from_millis(UPDATE_INTERVAL_MS));
                render(&mut output.lock().unwrap(), &started, num_lines.load(Ordering::Relaxed), num_bytes.load(Ordering::Relaxed));
            }
        });
    }

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
    // let seconds : f64 = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
    let millis : f64 = (elapsed.as_secs() * 1000 + (elapsed.subsec_nanos() as u64 / 1000000 as u64)) as f64;
    if millis == 0.0 {
        return;
    }
    let kb = num_bytes / 1024 as usize;
    let kb_per_millis = kb / (millis as usize);
    let lines_per_millis = num_lines / millis as usize;
    output.write_all(
        format!(
            "\x1B[1G\x1B[2K {:.1} sec | {} kb [ {:.1} kb/s ] | {} lines [ {:.0} ln/s ]",
            millis as f64 / 1000.0 as f64,
            kb,
            kb_per_millis as f64 / 1000.0 as f64,
            num_lines,
            lines_per_millis as f64 / 1000.0 as f64
            ).as_bytes()
        ) .unwrap();
}
