use std::io::{self, BufRead, BufReader, BufWriter, stdin, Write};
use std::string::{String};
use std::time::{Duration, Instant};

const CAP : usize = 1024 * 1024;

fn main() {
    let stdin = stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::with_capacity(CAP, stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    let mut buffer = String::new();
    let mut num_lines = 0;
    let mut num_bytes = 0;

    let started = Instant::now();
    let mut last = Instant::now();

    while reader.read_line(&mut buffer).unwrap() > 0 {
        writer.write(buffer.as_bytes()).unwrap();
        num_lines += 1;
        num_bytes += buffer.len();
        buffer.clear();
        if last.elapsed() > Duration::from_millis(789) {
            render(&started, &num_lines, &num_bytes);
            last = Instant::now();
        }
    }
    render(&started, &num_lines, &num_bytes);
    eprint!("\n")
}

fn render(started: &Instant, num_lines : &usize, num_bytes: &usize) {
    let elapsed = started.elapsed();
    let seconds = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
    if seconds == 0.0 {
        return;
    }
    let kb = num_bytes / 1024;
    let kb_per_sec : f64 = kb as f64 / 1024.0 / seconds;
    let lines_per_sec : f64 = *num_lines as f64 / seconds;

    eprint!("\x1B[1G\x1B[2K {:.1} sec | {} kb [ {:.1} kb/sec ] | {} lines [ {:.0} lines/sec ]",
              seconds, kb, kb_per_sec, num_lines, lines_per_sec);
}
