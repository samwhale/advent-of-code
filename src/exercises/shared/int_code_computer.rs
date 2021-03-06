pub struct IntCodeComputer {
    code: Vec<i64>,
    inputs: Vec<i64>,
    instruction_pointer: usize,
    pub is_done: bool,
    output: Vec<i64>,
    relative_base: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntCodeComputerResult {
    pub code: Vec<i64>,
    pub output: Vec<i64>,
}

pub fn read_code(message: &str) -> Vec<i64> {
    let mut code: Vec<i64> = vec![0; 100000];
    for (index, val) in message.trim().split(",").enumerate() {
        let val = val.parse::<i64>().unwrap();
        code.insert(index, val);
    }
    code
}

impl IntCodeComputer {
    pub fn new(message: &str) -> Self {
        Self {
            code: read_code(message),
            inputs: Vec::new(),
            instruction_pointer: 0,
            is_done: false,
            output: Vec::new(),
            relative_base: 0,
        }
    }

    pub fn process_code(&mut self) -> IntCodeComputerResult {
        self.is_done = false;
        self.output = Vec::new();
        loop {
            match self.code[self.instruction_pointer] % 100 {
                1 => self.add(),
                2 => self.multiply(),
                3 => {
                    if self.inputs.len() == 0 {
                        // wait for new input
                        break;
                    }
                    self.insert();
                }
                4 => self.output(),
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.less_than(),
                8 => self.equals(),
                9 => self.adjust_relative_base(),
                99 => {
                    self.is_done = true;
                    break;
                }
                _any => {
                    println!("uh oh, got input: {}", _any);
                    panic!()
                }
            }
        }

        IntCodeComputerResult {
            code: self.code.clone(),
            output: self.output.clone(),
        }
    }

    /**
     * 0 => position mode
     * 1 => immediate mode
     * 2 => relative mode
     */
    fn get_positions(&mut self, parameter_length: usize) -> [usize; 3] {
        let opcode = self.code[self.instruction_pointer];
        let modes: [usize; 3] = [
            (opcode / 100 % 10) as usize,
            (opcode / 1000 % 10) as usize,
            (opcode / 10000 % 10) as usize,
        ];

        let mut positions: [usize; 3] = [0 as usize; 3];
        for index in 0..parameter_length {
            match modes[index] {
                0 => positions[index] = self.code[self.instruction_pointer + index + 1] as usize,
                1 => positions[index] = self.instruction_pointer + index + 1,
                2 => {
                    positions[index] = (self.code[self.instruction_pointer + index + 1]
                        + self.relative_base) as usize
                }
                _ => panic!("Why did you get a mode that was not 1, 2, or 3"),
            }
        }
        positions
    }

    pub fn add_inputs(&mut self, inputs: Vec<i64>) {
        for input in inputs.iter() {
            self.inputs.push(*input);
        }
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

    fn adjust_relative_base(&mut self) {
        let [address_1, _, _] = self.get_positions(1);

        self.relative_base += self.code[address_1];
        self.instruction_pointer += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_input_equal_to_8_position_mode() {
        let message = "3,9,8,9,10,9,4,9,99,-1,8";
        let mut comp1 = IntCodeComputer::new(message);
        let mut comp2 = IntCodeComputer::new(message);
        comp1.add_inputs(vec![8]);
        comp2.add_inputs(vec![1]);

        // should return 1 if input equal to 8, else 0
        assert_eq!(comp1.process_code().output[0], 1);
        assert_eq!(comp2.process_code().output[0], 0);
    }

    #[test]
    fn process_input_equal_to_8_immediate_mode() {
        let message = "3,3,1108,-1,8,3,4,3,99";
        let mut comp1 = IntCodeComputer::new(message);
        let mut comp2 = IntCodeComputer::new(message);
        comp1.add_inputs(vec![8]);
        comp2.add_inputs(vec![1]);

        // should return 1 if input equal to 8, else 0
        assert_eq!(comp1.process_code().output[0], 1);
        assert_eq!(comp2.process_code().output[0], 0);
    }

    #[test]
    fn process_input_less_than_8_position_mode() {
        let message = "3,9,7,9,10,9,4,9,99,-1,8";
        let mut comp1 = IntCodeComputer::new(message);
        let mut comp2 = IntCodeComputer::new(message);
        comp1.add_inputs(vec![1]);
        comp2.add_inputs(vec![10]);

        // should return 1 if input less than 8, else 0
        assert_eq!(comp1.process_code().output[0], 1);
        assert_eq!(comp2.process_code().output[0], 0);
    }

    #[test]
    fn process_input_less_than_8_immediate_mode() {
        let message = "3,3,1107,-1,8,3,4,3,99";
        let mut comp1 = IntCodeComputer::new(message);
        let mut comp2 = IntCodeComputer::new(message);
        comp1.add_inputs(vec![1]);
        comp2.add_inputs(vec![10]);

        // should return 1 if input less than 8, else 0
        assert_eq!(comp1.process_code().output[0], 1);
        assert_eq!(comp2.process_code().output[0], 0);
    }

    #[test]
    fn process_jump_position_mode() {
        let message = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let mut comp1 = IntCodeComputer::new(message);
        let mut comp2 = IntCodeComputer::new(message);
        comp1.add_inputs(vec![0]);
        comp2.add_inputs(vec![-12]);

        // output 0 if the input was zero or 1 if the input was non-zero:
        assert_eq!(comp1.process_code().output[0], 0);
        assert_eq!(comp2.process_code().output[0], 1);
    }

    #[test]
    fn process_jump_immediate_mode() {
        let message = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let mut comp1 = IntCodeComputer::new(message);
        let mut comp2 = IntCodeComputer::new(message);
        comp1.add_inputs(vec![0]);
        comp2.add_inputs(vec![-12]);

        // output 0 if the input was zero or 1 if the input was non-zero:
        assert_eq!(comp1.process_code().output[0], 0);
        assert_eq!(comp2.process_code().output[0], 1);
    }

    #[test]
    fn process_large_message() {
        let message = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut comp1 = IntCodeComputer::new(message);
        let mut comp2 = IntCodeComputer::new(message);
        let mut comp3 = IntCodeComputer::new(message);
        comp1.add_inputs(vec![7]);
        comp2.add_inputs(vec![8]);
        comp3.add_inputs(vec![9]);

        // The program will then output 999 if the input value is below 8,
        // output 1000 if the input value is equal to 8,
        // or output 1001 if the input value is greater than 8.
        assert_eq!(comp1.process_code().output[0], 999);
        assert_eq!(comp2.process_code().output[0], 1000);
        assert_eq!(comp3.process_code().output[0], 1001);
    }

    #[test]
    fn large_output() {
        let message = "104,1125899906842624,99";
        let mut comp = IntCodeComputer::new(message);

        assert_eq!(comp.process_code().output[0], 1125899906842624);
    }

    #[test]
    fn copy_self_test() {
        let message = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut comp = IntCodeComputer::new(message);

        assert_eq!(comp.process_code().output[0], 109);
    }

    #[test]
    fn moar_tests() {
        let message = "1102,34915192,34915192,7,4,7,99,0";
        let mut comp = IntCodeComputer::new(message);

        assert_eq!(comp.process_code().output[0], 1219070632396864);
    }
}
