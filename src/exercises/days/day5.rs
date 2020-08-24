use super::super::super::utils;
use super::super::shared::int_code_computer::IntCodeComputer;

pub fn main() {
  println!("--- Day 5 ---");
  let message = utils::read_file_into_string("./src/exercises/data/data-day5.txt");
  let mut int_code_computer = IntCodeComputer::new(&message);
  int_code_computer.add_inputs(vec![1]);
  let result = int_code_computer.process_code();

  // println!("code: {:?}", code);
  println!("output: {:?}", result.output[0]);

  println!("--- Part 2 ---");
  let mut int_code_computer2 = IntCodeComputer::new(&message);
  int_code_computer2.add_inputs(vec![5]);
  let result = int_code_computer2.process_code();
  println!("output: {:?}", result.output[0]);
}
