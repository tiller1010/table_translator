use std::io::{BufReader, BufRead, Error};
use std::fs::File;

pub fn read_lines(filename: &str) -> Result<Vec<String>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn get_vector_from_csv_line(line: &str) -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();
    for value in line.split(",") {
        vector.push(value.parse::<String>().unwrap());
    }
    vector
}

