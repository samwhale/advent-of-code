use super::super::super::utils;
use super::super::shared::int_code_computer::IntCodeComputer;

pub fn main() {
  println!("--- Day 5 ---");
  let message = utils::read_file_into_string("./src/exercises/data/data-day5.txt");
  let mut int_code_computer = IntCodeComputer::new();
  int_code_computer.add_inputs(vec![1]);
  int_code_computer.process_code(&message);
}

#[cfg(test)]
mod tests {}
