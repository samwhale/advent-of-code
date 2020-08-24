use super::super::super::utils;
use super::super::shared::int_code_computer::IntCodeComputer;
use permutator::Permutation;

pub fn run_amplification_circuit(
  message: &str,
  input_to_amplifier_a: i32,
  phase_settings: Vec<i32>,
) -> i32 {
  let mut amplifier_a = IntCodeComputer::new(message);
  let mut amplifier_b = IntCodeComputer::new(message);
  let mut amplifier_c = IntCodeComputer::new(message);
  let mut amplifier_d = IntCodeComputer::new(message);
  let mut amplifier_e = IntCodeComputer::new(message);
  amplifier_a.add_inputs(vec![phase_settings[0], input_to_amplifier_a]);
  let result_a = amplifier_a.process_code();
  amplifier_b.add_inputs(vec![phase_settings[1], result_a.output[0]]);
  let result_b = amplifier_b.process_code();
  amplifier_c.add_inputs(vec![phase_settings[2], result_b.output[0]]);
  let result_c = amplifier_c.process_code();
  amplifier_d.add_inputs(vec![phase_settings[3], result_c.output[0]]);
  let result_d = amplifier_d.process_code();
  amplifier_e.add_inputs(vec![phase_settings[4], result_d.output[0]]);
  let result_e = amplifier_e.process_code();
  result_e.output[0]
}

pub fn run_feedback_loop(
  message: &str,
  input_to_amplifier_a: i32,
  phase_settings: &Vec<i32>,
) -> i32 {
  let mut amplifier_a = IntCodeComputer::new(message);
  let mut amplifier_b = IntCodeComputer::new(message);
  let mut amplifier_c = IntCodeComputer::new(message);
  let mut amplifier_d = IntCodeComputer::new(message);
  let mut amplifier_e = IntCodeComputer::new(message);
  amplifier_a.add_inputs(vec![phase_settings[0], input_to_amplifier_a]);
  let mut result_a = amplifier_a.process_code();
  amplifier_b.add_inputs(vec![phase_settings[1], result_a.output[0]]);
  let mut result_b = amplifier_b.process_code();
  amplifier_c.add_inputs(vec![phase_settings[2], result_b.output[0]]);
  let mut result_c = amplifier_c.process_code();
  amplifier_d.add_inputs(vec![phase_settings[3], result_c.output[0]]);
  let mut result_d = amplifier_d.process_code();
  amplifier_e.add_inputs(vec![phase_settings[4], result_d.output[0]]);
  let mut system_result = amplifier_e.process_code();
  while result_a.is_waiting
    || result_b.is_waiting
    || result_c.is_waiting
    || result_d.is_waiting
    || system_result.is_waiting
  {
    amplifier_a.add_inputs(vec![system_result.output[0]]);
    result_a = amplifier_a.process_code();
    amplifier_b.add_inputs(vec![result_a.output[0]]);
    result_b = amplifier_b.process_code();
    amplifier_c.add_inputs(vec![result_b.output[0]]);
    result_c = amplifier_c.process_code();
    amplifier_d.add_inputs(vec![result_c.output[0]]);
    result_d = amplifier_d.process_code();
    amplifier_e.add_inputs(vec![result_d.output[0]]);
    system_result = amplifier_e.process_code();
  }
  system_result.output[0]
}

pub fn part_1(message: &str) -> i32 {
  let mut results: Vec<i32> = Vec::new();
  [0, 1, 2, 3, 4]
    .permutation()
    .for_each(|permutation| results.push(run_amplification_circuit(&message, 0, permutation)));

  *results.iter().max().unwrap()
}

pub fn part_2(message: &str) -> i32 {
  let mut results: Vec<i32> = Vec::new();
  [9, 8, 7, 6, 5]
    .permutation()
    .for_each(|permutation| results.push(run_feedback_loop(&message, 0, &permutation)));

  *results.iter().max().unwrap()
}

pub fn main() {
  println!("--- Day 7 ---");
  let message = utils::read_file_into_string("./src/exercises/data/data-day7.txt");

  println!("max output from amplifiers: {}", part_1(&message));
  println!(
    "max output from amplifiers with feedback loop: {}",
    part_2(&message)
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_part_1() {
    let message = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    let result = part_1(message);
    assert_eq!(result, 43210);
  }

  #[test]
  pub fn feedback_loop_test() {
    let message =
      "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    let result = run_feedback_loop(message, 0, &vec![9, 8, 7, 6, 5]);
    assert_eq!(result, 139629729);
  }

  #[test]
  pub fn feedback_loop_test_2() {
    let message =
      "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
    let result = run_feedback_loop(message, 0, &vec![9, 7, 8, 5, 6]);
    assert_eq!(result, 18216);
  }
}
