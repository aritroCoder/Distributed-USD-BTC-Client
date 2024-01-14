use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn write_to_file(filename: &str, numbers: &[f64]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename)?;

    for &num in numbers {
        writeln!(file, "{}", num)?;
    }
    Ok(())
}