use super::super::super::utils;
use super::super::shared::int_code_computer::IntCodeComputer;

pub fn sensor_boost(message: &str, boost_mode: bool) -> i64 {
  let mut int_code_computer = IntCodeComputer::new(&message);
  let mode = if boost_mode { 2 } else { 1 };
  int_code_computer.add_inputs(vec![mode]);
  int_code_computer.process_code().output[0]
}

pub fn main() {
  println!("--- Day 9 ---");
  let message = utils::read_file_into_string("src/exercises/data/data-day9.txt");
  let result1 = sensor_boost(&message, false);
  let result2 = sensor_boost(&message, true);
  println!("Test mode -> output: {:?}", result1);
  println!("BOOST MODE -> output: {:?}", result2);
}
