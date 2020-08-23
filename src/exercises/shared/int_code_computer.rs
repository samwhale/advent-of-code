pub struct IntCodeComputer {
  pub code: Vec<i32>,
  pub inputs: Vec<i32>,
  pub output: Vec<i32>,
}

impl IntCodeComputer {
  pub fn new() -> Self {
    Self {
      code: Vec::new(),
      inputs: Vec::new(),
      output: Vec::new(),
    }
  }

  fn read(&mut self, message: &str) {
    for val in message.trim().split(",") {
      let val = val.parse::<i32>().unwrap();
      self.code.push(val);
    }
  }

  pub fn add_inputs(&mut self, inputs: Vec<i32>) {
    self.inputs = inputs;
  }

  /**
   * Sum the values at `address_of_param_1` and `address_of_param_2`.
   * Insert the result at the `address_to_update`
   */
  fn add(&mut self, instruction_pointer: usize, [mode_3, mode_2, mode_1]: [i32; 3]) {
    let address_of_param_1 = self.get_address_from_mode(instruction_pointer + 1, mode_1);
    let address_of_param_2 = self.get_address_from_mode(instruction_pointer + 2, mode_2);
    let address_to_update = self.get_address_from_mode(instruction_pointer + 3, mode_3);

    let sum = self.code[address_of_param_1] + self.code[address_of_param_2];

    self.code[address_to_update] = sum;
  }

  /**
   * Multiply the values of parameter 1 and 2
   * Insert the result at the `address_to_update`
   */
  fn multiply(&mut self, instruction_pointer: usize, [mode_3, mode_2, mode_1]: [i32; 3]) {
    let address_of_param_1 = self.get_address_from_mode(instruction_pointer + 1, mode_1);
    let address_of_param_2 = self.get_address_from_mode(instruction_pointer + 2, mode_2);
    let address_to_update = self.get_address_from_mode(instruction_pointer + 3, mode_3);

    let mult = self.code[address_of_param_1] * self.code[address_of_param_2];

    self.code[address_to_update] = mult;
  }

  /**
   * Multiply the values at `address_of_param_1` and `address_of_param_2`.
   * Insert the result at the `address_to_update`
   */
  fn insert(&mut self, instruction_pointer: usize) {
    let address_to_update = self.code[instruction_pointer + 1] as usize;

    self.code[address_to_update] = self.inputs[0];
    self.inputs.drain(0..1);
  }

  /**
   * 0 == position mode
   * 1 == immediate mode
   */
  fn get_value(&mut self, address: usize, mode: i32) {
    let address_of_param = self.get_address_from_mode(address + 1, mode);
    self.output.push(self.code[address_of_param]);
  }

  fn get_address_from_mode(&self, address: usize, parameter: i32) -> usize {
    if parameter == 1 {
      return address as usize;
    }

    self.code[address as usize] as usize
  }

  fn get_opcode_and_modes(&self, address: usize) -> ([i32; 3], i32) {
    let full_opcode = self.code[address];
    let opcode = full_opcode % 100;
    let modes: [i32; 3] = [
      full_opcode / 10000 % 10,
      full_opcode / 1000 % 10,
      full_opcode / 100 % 10,
    ];
    (modes, opcode)
  }

  pub fn process_code(mut self, message: &str) -> (Vec<i32>, i32) {
    self.read(message);
    let mut instruction_pointer = 0;
    loop {
      let (modes, opcode) = self.get_opcode_and_modes(instruction_pointer);
      match opcode {
        1 => {
          self.add(instruction_pointer, modes);
          instruction_pointer += 4
        }
        2 => {
          self.multiply(instruction_pointer, modes);
          instruction_pointer += 4
        }
        3 => {
          self.insert(instruction_pointer);
          instruction_pointer += 2;
        }
        4 => {
          self.get_value(instruction_pointer, modes[2]);
          instruction_pointer += 2;
        }
        5 => {
          /*
           * Opcode 5 is jump-if-true: if the first parameter is non-zero,
           * it sets the instruction pointer to the value from the second parameter.
           * Otherwise, it does nothing.
           */
          let address_of_param_1 = self.get_address_from_mode(instruction_pointer + 1, modes[2]);
          let address_of_param_2 = self.get_address_from_mode(instruction_pointer + 2, modes[1]);
          if self.code[address_of_param_1] != 0 {
            instruction_pointer = self.code[address_of_param_2] as usize;
          } else {
            instruction_pointer += 3;
          }
        }
        6 => {
          /*
           * Opcode 6 is jump-if-false: if the first parameter is zero,
           * it sets the instruction pointer to the value from the second parameter.
           * Otherwise, it does nothing.
           */
          let address_of_param_1 = self.get_address_from_mode(instruction_pointer + 1, modes[2]);
          let address_of_param_2 = self.get_address_from_mode(instruction_pointer + 2, modes[1]);
          if self.code[address_of_param_1] == 0 {
            instruction_pointer = self.code[address_of_param_2] as usize;
          } else {
            instruction_pointer += 3;
          }
        }
        7 => {
          /*
           * Opcode 7 is less than: if the first parameter is less than the second parameter,
           * it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
           */
          let address_of_param_1 = self.get_address_from_mode(instruction_pointer + 1, modes[2]);
          let address_of_param_2 = self.get_address_from_mode(instruction_pointer + 2, modes[1]);
          let address_of_param_3 = self.get_address_from_mode(instruction_pointer + 3, modes[0]);
          if self.code[address_of_param_1] < self.code[address_of_param_2] {
            self.code[address_of_param_3] = 1;
          } else {
            self.code[address_of_param_3] = 0;
          }
          instruction_pointer += 4;
        }
        8 => {
          /*
           * Opcode 8 is equals: if the first parameter is equal to the second parameter,
           * it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
           */
          let address_of_param_1 = self.get_address_from_mode(instruction_pointer + 1, modes[2]);
          let address_of_param_2 = self.get_address_from_mode(instruction_pointer + 2, modes[1]);
          let address_of_param_3 = self.get_address_from_mode(instruction_pointer + 3, modes[0]);
          if self.code[address_of_param_1] == self.code[address_of_param_2] {
            self.code[address_of_param_3] = 1;
          } else {
            self.code[address_of_param_3] = 0;
          }
          instruction_pointer += 4;
        }
        99 => break,
        _any => {
          println!("uh oh, got input: {}", _any);
          panic!()
        }
      }
    }

    let output = self.output.iter().sum();

    (self.code, output)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn process_code_1() {
    let comp = IntCodeComputer::new();
    assert_eq!(comp.process_code("1,0,0,0,99").0, vec![2, 0, 0, 0, 99]);
  }
  #[test]
  fn process_code_2() {
    let comp = IntCodeComputer::new();
    assert_eq!(comp.process_code("2,3,0,3,99").0, vec![2, 3, 0, 6, 99]);
  }

  #[test]
  fn process_code_3() {
    let comp = IntCodeComputer::new();
    assert_eq!(
      comp.process_code("02,4,4,5,99,0").0,
      vec![2, 4, 4, 5, 99, 9801]
    );
  }

  #[test]
  fn process_code_4() {
    let comp = IntCodeComputer::new();
    assert_eq!(
      comp.process_code("01,1,1,4,99,5,6,0,99").0,
      vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
  }

  #[test]
  fn process_code_5() {
    let comp = IntCodeComputer::new();
    assert_eq!(
      comp.process_code("1002,4,3,4,33").0,
      vec![1002, 4, 3, 4, 99]
    );
  }

  #[test]
  fn process_code_with_insert() {
    let mut comp = IntCodeComputer::new();
    comp.add_inputs(vec![1]);
    assert_eq!(
      comp.process_code("01,0,0,0,3,3,99").0,
      vec![2, 0, 0, 1, 3, 3, 99]
    );
  }
  #[test]
  fn process_input_equal_to_8_position_mode() {
    let mut comp1 = IntCodeComputer::new();
    let mut comp2 = IntCodeComputer::new();
    comp1.add_inputs(vec![8]);
    comp2.add_inputs(vec![1]);

    // should return 1 if input equal to 8, else 0
    assert_eq!(comp1.process_code("3,9,8,9,10,9,4,9,99,-1,8").1, 1);
    assert_eq!(comp2.process_code("3,9,8,9,10,9,4,9,99,-1,8").1, 0);
  }

  #[test]
  fn process_input_equal_to_8_immediate_mode() {
    let mut comp1 = IntCodeComputer::new();
    let mut comp2 = IntCodeComputer::new();
    comp1.add_inputs(vec![8]);
    comp2.add_inputs(vec![1]);

    // should return 1 if input equal to 8, else 0
    assert_eq!(comp1.process_code("3,3,1108,-1,8,3,4,3,99").1, 1);
    assert_eq!(comp2.process_code("3,3,1108,-1,8,3,4,3,99").1, 0);
  }

  #[test]
  fn process_input_less_than_8_position_mode() {
    let mut comp1 = IntCodeComputer::new();
    let mut comp2 = IntCodeComputer::new();
    comp1.add_inputs(vec![1]);
    comp2.add_inputs(vec![10]);

    // should return 1 if input less than 8, else 0
    assert_eq!(comp1.process_code("3,9,7,9,10,9,4,9,99,-1,8").1, 1);
    assert_eq!(comp2.process_code("3,9,7,9,10,9,4,9,99,-1,8").1, 0);
  }

  #[test]
  fn process_input_less_than_8_immediate_mode() {
    let mut comp1 = IntCodeComputer::new();
    let mut comp2 = IntCodeComputer::new();
    comp1.add_inputs(vec![1]);
    comp2.add_inputs(vec![10]);

    // should return 1 if input less than 8, else 0
    assert_eq!(comp1.process_code("3,3,1107,-1,8,3,4,3,99").1, 1);
    assert_eq!(comp2.process_code("3,3,1107,-1,8,3,4,3,99").1, 0);
  }

  #[test]
  fn process_jump_position_mode() {
    let mut comp1 = IntCodeComputer::new();
    let mut comp2 = IntCodeComputer::new();
    comp1.add_inputs(vec![0]);
    comp2.add_inputs(vec![-12]);

    // output 0 if the input was zero or 1 if the input was non-zero:
    assert_eq!(
      comp1
        .process_code("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")
        .1,
      0
    );
    assert_eq!(
      comp2
        .process_code("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")
        .1,
      1
    );
  }

  #[test]
  fn process_jump_immediate_mode() {
    let mut comp1 = IntCodeComputer::new();
    let mut comp2 = IntCodeComputer::new();
    comp1.add_inputs(vec![0]);
    comp2.add_inputs(vec![-12]);

    // output 0 if the input was zero or 1 if the input was non-zero:
    assert_eq!(
      comp1.process_code("3,3,1105,-1,9,1101,0,0,12,4,12,99,1").1,
      0
    );
    assert_eq!(
      comp2.process_code("3,3,1105,-1,9,1101,0,0,12,4,12,99,1").1,
      1
    );
  }

  #[test]
  fn process_large_message() {
    let mut comp1 = IntCodeComputer::new();
    let mut comp2 = IntCodeComputer::new();
    let mut comp3 = IntCodeComputer::new();
    comp1.add_inputs(vec![7]);
    comp2.add_inputs(vec![8]);
    comp3.add_inputs(vec![9]);

    let message = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

    // The program will then output 999 if the input value is below 8,
    // output 1000 if the input value is equal to 8,
    // or output 1001 if the input value is greater than 8.
    assert_eq!(comp1.process_code(message).1, 999);
    assert_eq!(comp2.process_code(message).1, 1000);
    assert_eq!(comp3.process_code(message).1, 1001);
  }
}
