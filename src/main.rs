use std::io::{self, BufRead, BufReader, BufWriter, stdin, Write};
use std::string::{String};

const CAP : usize = 1024 * 1024;

fn main() {
    let stdin = stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::with_capacity(CAP, stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    let mut buffer = String::new();
    let mut num_lines = 0;
    let mut num_bytes = 0;
    while reader.read_line(&mut buffer).unwrap() > 0 {
        writer.write(buffer.as_bytes()).unwrap();
        num_lines += 1;
        num_bytes += buffer.len();
        buffer.clear();
    }
    eprintln!("num_lines: {}, num_bytes: {}",
              num_lines, num_bytes);
}
