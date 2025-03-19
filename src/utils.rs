use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> std::io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    buf.lines().collect()
}

pub fn read_corpus<P>(filename: P) -> std::io::Result<String>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename)?;
    Ok(lines.join("\n"))
}