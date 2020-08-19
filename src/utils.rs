use std::fs;
use std::io::{BufRead, BufReader};

pub fn read_file_into_vector(filename: &str) -> Vec<String> {
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut vec: Vec<String> = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        vec.push(line);
    }

    vec
}

pub fn read_file_into_string(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}
