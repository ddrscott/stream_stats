extern crate memmap;

use std::env;
use std::io::{self, Write};
use std::fs::File;

use memmap::Mmap;

/// Output a file's contents to stdout. The file path must be provided as the first process
/// argument.
fn main() {
    let path = env::args()
        .nth(1)
        .expect("supply a single path as the program argument");

    let file = File::open(path).expect("failed to open the file");

    let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writer.write(b"!!!!!!!!!!!!!!!!!!");

    let mut iter = mmap.split(|num| num == '\n');
    eprintln!("!!!!!!! lines: {}", iter.count());
    for line in iter {
        writer.write_all(&line).unwrap();
    }
    eprintln!("!!!!!!! lines: {}", iter.count());
}
