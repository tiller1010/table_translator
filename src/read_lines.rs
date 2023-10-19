use std::io::{BufReader, BufRead, Error};
use std::fs::File;

pub fn read_lines(filename: &str) -> Result<Vec<String>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn get_vector_from_csv_line(line: &str) -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();
    // get vector from csv line
    let csv_vector: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
    let mut skip_iterations = 0;

    for value in csv_vector.iter() {

        if skip_iterations > 0 {
            skip_iterations -= 1;
            continue;
        }

        let original_value = value.to_string();
        let mut value = value.to_string();

        // If the value starts with a quote, add the next value until the value ends with a quote
        if value.starts_with("\"") {
            value.remove(0);
            let mut next_count = 1;
            while !value.ends_with("\"") {
                let current_value_position = csv_vector.iter().position(|x| x == &original_value);
                if current_value_position.is_none() {
                    break;
                }
                let next_value_position = current_value_position.unwrap() + next_count;
                next_count += 1;
                skip_iterations += 1;
                if next_value_position >= csv_vector.len() {
                    break;
                }
                let next_value = csv_vector.get(next_value_position);
                if next_value.is_none() {
                    break;
                }
                let next_value = next_value.unwrap();
                value.push_str(",");
                value.push_str(next_value);
            }
            value.remove(value.len() - 1);
        }

        vector.push(value.to_string());
    }
    vector
}

