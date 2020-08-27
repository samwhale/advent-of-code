use super::super::super::utils;
use super::super::shared::int_code_computer::IntCodeComputer;

pub fn main() {
  println!("--- Day 9 ---");
  let message = utils::read_file_into_string("src/exercises/data/data-day9.txt");
  let mut int_code_computer1 = IntCodeComputer::new(&message);
  int_code_computer1.add_inputs(vec![1]);
  let result1 = int_code_computer1.process_code();
  println!("Test mode -> output: {:?}", result1.output[0]);
  println!("--- Part 2 ---");
  let mut int_code_computer2 = IntCodeComputer::new(&message);
  int_code_computer2.add_inputs(vec![2]);
  let result2 = int_code_computer2.process_code();
  println!("BOOST MODE -> output: {:?}", result2.output[0]);
}
