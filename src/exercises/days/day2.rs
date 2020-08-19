use super::super::super::utils;

struct IntCodeComputer {
    code: Vec<u32>,
}

pub fn add(code: &Vec<u32>, [index1, index2, index3]: [u32; 3]) -> Vec<u32> {
    let mut code = code.clone();
    let sum = code[index1 as usize] + code[index2 as usize];
    code[index3 as usize] = sum;

    return code;
}

pub fn multiply(code: &Vec<u32>, [index1, index2, index3]: [u32; 3]) -> Vec<u32> {
    let mut code = code.clone();
    let mult = code[index1 as usize] * code[index2 as usize];
    code[index3 as usize] = mult;

    return code;
}

impl IntCodeComputer {
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
            let vec_index = i * 4;
            match self.code[vec_index] {
                1 => {
                    self.code = add(
                        &self.code,
                        [
                            self.code[vec_index + 1],
                            self.code[vec_index + 2],
                            self.code[vec_index + 3],
                        ],
                    );
                }
                2 => {
                    self.code = multiply(
                        &self.code,
                        [
                            self.code[vec_index + 1],
                            self.code[vec_index + 2],
                            self.code[vec_index + 3],
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

pub fn main() {
    let message = utils::read_file_into_string("./src/exercises/data/data-day2.txt");
    let int_code_computer = IntCodeComputer { code: Vec::new() };
    let code = int_code_computer.process_code(message.clone());

    println!("--- Day 2 ---");
    println!("message: {:?}", message);
    println!("processed: {:?}", code);
}
