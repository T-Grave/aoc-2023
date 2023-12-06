use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_input_lines(file_path: &str) -> Result<Vec<String>, Error> {
    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    reader.lines().collect()
}
