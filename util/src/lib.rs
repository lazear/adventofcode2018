use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

pub fn read<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let f = File::open(path)?;
    let buf = BufReader::new(f);
    buf.lines().collect::<io::Result<Vec<String>>>()
}