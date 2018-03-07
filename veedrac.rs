/*
 * https://codereview.stackexchange.com/questions/94941/simple-cat-in-rust
 */
use std::env;
use std::io::{self, Read, Write};
use std::iter;
use std::fs::File;

const SMALL_BUFFER_SIZE: usize = 256;
const LARGE_BUFFER_SIZE: usize = 1024 * 1024;

macro_rules! println_stderr(
    ($($arg:tt)*) => (
        if let Err(err) = writeln!(&mut io::stderr(), $($arg)* ) {
            panic!("Unable to write to stderr: {}", err);
        }
    )
);


fn redirect_stream<R, W>(reader: &mut R, writer: &mut W, buffer: &mut Vec<u8>) -> io::Result<()>
    where R: Read, W: Write
{
    loop {
        let len_read = try!(reader.read(buffer));

        if len_read == 0 {
            return Ok(())
        }

        try!(writer.write_all(&buffer[..len_read]));

        if len_read == buffer.len() && len_read < LARGE_BUFFER_SIZE {
            buffer.extend(iter::repeat(0).take(len_read));
        }
    }
}


fn main() {
    let mut args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        args.push("-".into());
    }

    fn handle_arg<R, W>(reader: &mut R, writer: &mut W, buffer: &mut Vec<u8>)
        where R: Read, W: Write
    {
        if let Err(err) = redirect_stream(reader, writer, buffer) {
            println_stderr!("{}", err.to_string());
        }
    }

    let stdout = &mut io::stdout();
    let buffer = &mut vec![0; SMALL_BUFFER_SIZE];
    for arg in args {
        if arg == "-" {
            handle_arg(&mut io::stdin(), stdout, buffer);
            continue;
        }

        match File::open(arg) {
            Ok(ref mut file) => {
                handle_arg(file, stdout, buffer)
            },
            Err(err) => {
                println_stderr!("{}", err);
                continue;
            }
        }
    }
}

