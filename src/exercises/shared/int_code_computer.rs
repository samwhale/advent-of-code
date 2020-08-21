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

  fn read(&mut self, message: String) {
    for val in message.trim().split(",") {
      let val = val.parse::<u32>().unwrap();
      self.code.push(val);
    }
  }

  pub fn process_code(mut self, message: String) -> Vec<u32> {
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
