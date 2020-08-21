pub struct IntCodeComputer {
  pub code: Vec<u32>,
}

fn add(code: &Vec<u32>, [addr1, addr2, addr3]: [u32; 3]) -> Vec<u32> {
  let mut code = code.clone();
  let sum = code[addr1 as usize] + code[addr2 as usize];
  code[addr3 as usize] = sum;

  return code;
}

fn multiply(code: &Vec<u32>, [addr1, addr2, addr3]: [u32; 3]) -> Vec<u32> {
  let mut code = code.clone();
  let mult = code[addr1 as usize] * code[addr2 as usize];
  code[addr3 as usize] = mult;

  return code;
}

impl IntCodeComputer {
  pub fn new() -> Self {
    Self { code: Vec::new() }
  }

  fn read(&mut self, message: &str) {
    for val in message.trim().split(",") {
      let val = val.parse::<u32>().unwrap();
      self.code.push(val);
    }
  }

  pub fn process_code(mut self, message: &str) -> Vec<u32> {
    self.read(message);

    let code_len = self.code.len();
    let chunk_len = code_len / 4;

    for i in 0..chunk_len {
      let instr_pointer = i * 4;
      match self.code[instr_pointer] {
        1 => {
          self.code = add(
            &self.code,
            [
              self.code[instr_pointer + 1],
              self.code[instr_pointer + 2],
              self.code[instr_pointer + 3],
            ],
          );
        }
        2 => {
          self.code = multiply(
            &self.code,
            [
              self.code[instr_pointer + 1],
              self.code[instr_pointer + 2],
              self.code[instr_pointer + 3],
            ],
          );
        }
        99 => break,
        _ => break,
      }
    }

    self.code
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn process_code_test() {
    let comp1 = IntCodeComputer::new();
    let comp2 = IntCodeComputer::new();
    let comp3 = IntCodeComputer::new();
    let comp4 = IntCodeComputer::new();
    assert_eq!(comp1.process_code("1,0,0,0,99"), vec![2, 0, 0, 0, 99]);
    assert_eq!(comp2.process_code("2,3,0,3,99"), vec![2, 3, 0, 6, 99]);
    assert_eq!(
      comp3.process_code("2,4,4,5,99,0"),
      vec![2, 4, 4, 5, 99, 9801]
    );
    assert_eq!(
      comp4.process_code("1,1,1,4,99,5,6,0,99"),
      vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
  }
}
