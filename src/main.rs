use std::env;
use std::io::{stdout, Write, BufWriter};

fn main() {
    let expletive = env::args().nth(1).unwrap_or("y".into());
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    loop {
        writeln!(out, "{}", expletive).expect("Failed to write an expletive");
    }
}
