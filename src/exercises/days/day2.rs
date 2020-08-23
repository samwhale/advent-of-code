use super::super::super::utils;
use super::super::shared::int_code_computer::IntCodeComputer;

pub fn main() {
    let message = utils::read_file_into_string("./src/exercises/data/data-day2.txt");
    let int_code_computer = IntCodeComputer::new();
    let code = int_code_computer.process_code(&message).0;

    println!("--- Day 2 ---");
    println!("message: {:?}", message);
    println!("processed: {:?}", code);
}
