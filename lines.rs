use std::io::{self, BufRead, BufReader, BufWriter, stdin, Write};

const CAP: usize = 1024 * 128;

// Slower due to extra write for "\n"
fn main() {
    let sin = std::io::stdin();
    let sout = std::io::stdout();

    let mut num_lines = 0;
    let mut num_bytes = 0;
    let file = BufReader::with_capacity(CAP, sin.lock());
    let mut writer = BufWriter::new(sout.lock());
    for line in file.lines() {
        let l = line.unwrap();
        writer.write(l.as_bytes()).unwrap();
        writer.write(b"\n").unwrap();
        num_lines += 1;
        num_bytes += l.len();
    }
    eprintln!("num_lines: {}, num_bytes: {}", num_lines, num_bytes);
}

