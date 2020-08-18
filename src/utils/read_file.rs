use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(filename: &str) -> Vec<String> {
  println!("in file {}", filename);

  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let mut vec: Vec<String> = Vec::new();

  for (_, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    vec.push(line);
  }

  vec
}
