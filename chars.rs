use std::io::{self, BufRead, BufReader, BufWriter, stdin, Read, Write};

const CAP: usize = 1024 * 128;

fn main() {
    let sin = std::io::stdin();
    let sout = std::io::stdout();

    let mut num_lines = 0;
    let mut num_bytes = 0;
    let file = BufReader::with_capacity(CAP, sin.lock());
    let mut writer = BufWriter::new(sout.lock());
    for b in file.bytes() {
        // writer.write(&[b.unwrap()]);
        // let l = line.unwrap();
        // writer.write(l.as_bytes());
        // writer.write(b"\n");
        // num_lines += 1;
        num_bytes += 1;
    }
    eprintln!("num_lines: {}, num_bytes: {}", num_lines, num_bytes);
}

