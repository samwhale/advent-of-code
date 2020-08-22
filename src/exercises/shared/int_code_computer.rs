pub struct IntCodeComputer {
  pub code: Vec<i32>,
  pub inputs: Vec<i32>,
}

impl IntCodeComputer {
  pub fn new() -> Self {
    Self {
      code: Vec::new(),
      inputs: Vec::new(),
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

  fn get_value(&self, address: usize) {
    let address_to_lookup = self.code[address + 1] as usize;
    println!("output: {}", self.code[address_to_lookup]);
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

  pub fn process_code(mut self, message: &str) -> Vec<i32> {
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
          self.get_value(instruction_pointer);
          instruction_pointer += 2;
        }
        99 => break,
        _any => {
          println!("uh oh, got input: {}", _any);
          panic!()
        }
      }
    }

    self.code
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn process_code_1() {
    let comp1 = IntCodeComputer::new();
    assert_eq!(comp1.process_code("1,0,0,0,99"), vec![2, 0, 0, 0, 99]);
  }
  #[test]
  fn process_code_2() {
    let comp2 = IntCodeComputer::new();
    assert_eq!(comp2.process_code("2,3,0,3,99"), vec![2, 3, 0, 6, 99]);
  }

  #[test]
  fn process_code_3() {
    let comp3 = IntCodeComputer::new();
    assert_eq!(
      comp3.process_code("02,4,4,5,99,0"),
      vec![2, 4, 4, 5, 99, 9801]
    );
  }

  #[test]
  fn process_code_4() {
    let comp4 = IntCodeComputer::new();
    assert_eq!(
      comp4.process_code("01,1,1,4,99,5,6,0,99"),
      vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
  }

  #[test]
  fn process_code_5() {
    let comp4 = IntCodeComputer::new();
    assert_eq!(comp4.process_code("1002,4,3,4,33"), vec![1002, 4, 3, 4, 99]);
  }

  #[test]
  fn process_code_with_insert() {
    let mut comp1 = IntCodeComputer::new();
    comp1.add_inputs(vec![1]);

    assert_eq!(
      comp1.process_code("01,0,0,0,3,3,99"),
      vec![2, 0, 0, 1, 3, 3, 99]
    );
  }
}
