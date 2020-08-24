pub struct IntCodeComputer {
  pub code: Vec<i32>,
  pub inputs: Vec<i32>,
  pub output: Vec<i32>,
  pub instruction_pointer: usize,
  pub completed: bool,
}

pub struct IntCodeComputerResult {
  pub code: Vec<i32>,
  pub output: i32,
  pub completed: bool,
}

impl IntCodeComputer {
  pub fn new() -> Self {
    Self {
      code: Vec::new(),
      inputs: Vec::new(),
      output: Vec::new(),
      instruction_pointer: 0,
      completed: false,
    }
  }
  pub fn process_code(mut self, message: &str) -> IntCodeComputerResult {
    self.read(message);
    println!("inputs: {:?}", self.inputs);

    loop {
      println!("opcode: {}", self.code[self.instruction_pointer] % 100);
      println!("code: {:?}", self.code);
      match self.code[self.instruction_pointer] % 100 {
        1 => self.add(),
        2 => self.multiply(),
        3 => {
          if self.inputs.len() == 0 {
            println!("Your inputs are empty ugh");
            break;
          }
          self.insert();
        }
        4 => self.output(),
        5 => self.jump_if_true(),
        6 => self.jump_if_false(),
        7 => self.less_than(),
        8 => self.equals(),
        99 => {
          self.completed = true;
          break;
        }
        _any => {
          println!("uh oh, got input: {}", _any);
          panic!()
        }
      }
    }

    let output = self.output.iter().sum();
    println!("output arr: {:?}", self.output);

    IntCodeComputerResult {
      code: self.code,
      output,
      completed: self.completed,
    }
  }

  fn get_positions(&mut self, parameter_length: usize) -> [usize; 3] {
    let opcode = self.code[self.instruction_pointer];
    let modes: [usize; 3] = [
      (opcode / 100 % 10) as usize,
      (opcode / 1000 % 10) as usize,
      (opcode / 10000 % 10) as usize,
    ];

    let mut positions: [usize; 3] = [0 as usize; 3];
    for index in 0..parameter_length {
      positions[index] = if modes[index] == 0 {
        self.code[self.instruction_pointer + index + 1] as usize
      } else {
        self.instruction_pointer + index + 1
      }
    }
    positions
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

  fn add(&mut self) {
    let [address_1, address_2, address_3] = self.get_positions(3);
    self.code[address_3] = self.code[address_1] + self.code[address_2];
    self.instruction_pointer += 4;
  }

  fn multiply(&mut self) {
    let [address_1, address_2, address_3] = self.get_positions(3);
    self.code[address_3] = self.code[address_1] * self.code[address_2];
    self.instruction_pointer += 4;
  }

  fn insert(&mut self) {
    let [address_1, _, _] = self.get_positions(1);
    self.code[address_1] = self.inputs[0];
    self.inputs.drain(0..1);
    self.instruction_pointer += 2;
  }
  fn output(&mut self) {
    let [address_1, _, _] = self.get_positions(1);
    self.output.push(self.code[address_1]);
    self.instruction_pointer += 2;
  }

  fn jump_if_true(&mut self) {
    let [address_1, address_2, _] = self.get_positions(2);
    println!("{}, {}", address_1, address_2);
    if self.code[address_1] != 0 {
      self.instruction_pointer = self.code[address_2] as usize;
    } else {
      self.instruction_pointer += 3;
    }
  }
  fn jump_if_false(&mut self) {
    let [address_1, address_2, _] = self.get_positions(2);
    if self.code[address_1] == 0 {
      self.instruction_pointer = self.code[address_2] as usize;
    } else {
      self.instruction_pointer += 3;
    }
  }

  fn less_than(&mut self) {
    let [address_1, address_2, address_3] = self.get_positions(3);
    if self.code[address_1] < self.code[address_2] {
      self.code[address_3] = 1;
    } else {
      self.code[address_3] = 0;
    }
    self.instruction_pointer += 4;
  }

  fn equals(&mut self) {
    let [address_1, address_2, address_3] = self.get_positions(3);
    if self.code[address_1] == self.code[address_2] {
      self.code[address_3] = 1;
    } else {
      self.code[address_3] = 0;
    }
    self.instruction_pointer += 4;
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
