use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, BufWriter, stdin, Write};
use std::string::{String};
use std::time::{Duration, Instant};
use std::thread::{self, sleep};

static READ_BUF_SIZE : usize = 1024 * 1024;
static UPDATE_INTERVAL_MS : u64 = 500;

// Using globals for performance reasons.
// We know we're updating in the `while` loop and the thread is occationally
// rendering info back to terminal.
//
// Adding atomic updates or mutex is too much
// overhead for a comparitively small amount of concurrent access against these
// variables.
static mut NUM_LINES : u64 = 0;
static mut NUM_BYTES : u64 = 0;

// Any data from `stdin` is passed to `stdout`.
fn main() {
    let stdin = stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::with_capacity(READ_BUF_SIZE, stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    let mut buffer = String::new();

    let started : Instant = Instant::now();

    thread::spawn(move || {
        loop {
            sleep(Duration::from_millis(UPDATE_INTERVAL_MS));
            unsafe { render(started, NUM_LINES, NUM_BYTES) }
        }
    });

    while reader.read_line(&mut buffer).unwrap() > 0 {
        writer.write(buffer.as_bytes()).unwrap();
        unsafe {
            NUM_LINES += 1;
            NUM_BYTES += buffer.len() as u64;
        }
        buffer.clear();
    }
    unsafe { render(started, NUM_LINES, NUM_BYTES) }
    eprint!("\n")
}

fn render(started : Instant, num_lines : u64, num_bytes : u64) {
    let elapsed = started.elapsed();
    let seconds = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
    if seconds == 0.0 {
        return;
    }
    let kb = num_bytes / 1024;
    let kb_per_sec : f64 = kb as f64 / 1024.0 / seconds;
    let lines_per_sec : f64 = num_lines as f64 / seconds;
    let mut tty = OpenOptions::new().write(true).append(true).open("/dev/tty").unwrap();
    tty.write_all(
        format!(
            "\x1B[1G\x1B[2K {:.1} sec | {} kb [ {:.1} kb/sec ] | {} lines [ {:.0} lines/sec ]",
              seconds, kb, kb_per_sec, num_lines, lines_per_sec).as_bytes()
        ) .unwrap();
}
