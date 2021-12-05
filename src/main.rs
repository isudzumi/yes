use std::env;
use std::io::{stdout, Write, BufWriter };

const BUFFER_CAPACITY: usize = 64 * 1024;

fn main() {
    let expletive = env::args().nth(1).unwrap_or("y".into());
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let expletive =  expletive + "\n";
    let buffer = expletive.repeat(BUFFER_CAPACITY / expletive.len());
    loop {
        out.write_all(buffer.as_bytes()).expect("Failed to write an expletive")
    }
}
