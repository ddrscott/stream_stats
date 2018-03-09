use std::{fmt};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

static CLEAR_LINE : &str = "\x1B[1G\x1B[2K";

pub struct Stats {
    started : Instant,
    lines : AtomicUsize,
    bytes : AtomicUsize,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            started : Instant::now(),
            lines : AtomicUsize::new(0),
            bytes : AtomicUsize::new(0),
        }
    }

    pub fn add(&self, buffer : &Vec<u8>) {
        self.lines.fetch_add(1, Ordering::Relaxed);
        self.bytes.fetch_add(buffer.len(), Ordering::Relaxed);
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let lines = self.lines.load(Ordering::Relaxed);
        let bytes = self.bytes.load(Ordering::Relaxed);

        let elapsed = self.started.elapsed();
        let seconds : f64 = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
        if seconds == 0.0 {
            return write!(f, "");
        }
        let kb = bytes as f64 / 1024 as f64;
        let kb_per_sec = kb / seconds;
        let lines_per_sec = lines as f64 / seconds;
        write!(f, "{} {:.1} sec | {:.0} kb [ {:.1}/s ] | {} lines [ {:.0}/s ]",
               CLEAR_LINE, seconds, kb, kb_per_sec, lines, lines_per_sec)
    }
}
