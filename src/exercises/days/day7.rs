use super::super::super::utils;
use super::super::shared::int_code_computer::IntCodeComputer;
use permutator::Permutation;

pub fn run_amplification_circuit(message: &str, phase_settings: Vec<i32>) -> i32 {
  let mut amplifier_a = IntCodeComputer::new();
  let mut amplifier_b = IntCodeComputer::new();
  let mut amplifier_c = IntCodeComputer::new();
  let mut amplifier_d = IntCodeComputer::new();
  let mut amplifier_e = IntCodeComputer::new();
  amplifier_a.add_inputs(vec![phase_settings[0], 0]);
  let (_code, output_a) = amplifier_a.process_code(&message);
  amplifier_b.add_inputs(vec![phase_settings[1], output_a]);
  let (_code, output_b) = amplifier_b.process_code(&message);
  amplifier_c.add_inputs(vec![phase_settings[2], output_b]);
  let (_code, output_c) = amplifier_c.process_code(&message);
  amplifier_d.add_inputs(vec![phase_settings[3], output_c]);
  let (_code, output_d) = amplifier_d.process_code(&message);
  amplifier_e.add_inputs(vec![phase_settings[4], output_d]);
  let (_code, output_e) = amplifier_e.process_code(&message);
  output_e
}

pub fn main() {
  println!("--- Day 7 ---");
  let message = utils::read_file_into_string("./src/exercises/data/data-day7.txt");

  let mut results: Vec<i32> = Vec::new();
  [0, 1, 2, 3, 4]
    .permutation()
    .for_each(|permutation| results.push(run_amplification_circuit(&message, permutation)));

  let output = results.iter().max();

  println!("output from amplifiers: {}", output.unwrap());
}
