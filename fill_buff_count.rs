// https://stackoverflow.com/questions/37079342/most-efficient-way-to-read-large-file-in-chunks
use std::io::{self, BufRead, BufReader, BufWriter, stdin, Write};
extern crate bytecount;

const CAP : usize = 1024 * 128;

fn main() {
    let stdin = stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::with_capacity(CAP, stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    loop {
        let length = {
            let buffer = reader.fill_buf().unwrap();
            // do stuff with buffer here
            buffer.len();
            if buffer.len() == 0 { break; }
            writer.write(buffer).unwrap();
            buffer.len()
        };
        reader.consume(length);
    }
}
