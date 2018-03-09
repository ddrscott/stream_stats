use std::{fmt};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

pub struct Stats {
    started : Instant,
    pub lines : AtomicUsize,
    pub bytes : AtomicUsize,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            started : Instant::now(),
            lines : AtomicUsize::new(0),
            bytes : AtomicUsize::new(0),
        }
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
        write!(f, "\x1B[1G\x1B[2K {:.1} sec | {:.0} kb [ {:.1}/s ] | {} lines [ {:.0}/s ]",
               seconds, kb, kb_per_sec, lines, lines_per_sec)
    }
}
