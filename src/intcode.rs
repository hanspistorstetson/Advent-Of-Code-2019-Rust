#[derive(Debug, Clone)]
pub enum Parameter {
    Position(usize),
    Relative(i64),
    Immediate(i64),
}

impl Parameter {
    pub fn new(mode: i64, value: i64) -> Parameter {
        match mode {
            0 => Parameter::Position(value as usize),
            1 => Parameter::Immediate(value),
            2 => Parameter::Relative(value),
            _ => unimplemented!(),
        }
    }

    pub fn get(&self, memory: &Vec<i64>, relative_base: i64) -> i64 {
        match self {
            Parameter::Immediate(value) => *value,
            Parameter::Position(position) => memory[*position],
            Parameter::Relative(position) => memory[(position + relative_base) as usize],
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpTrue(Parameter, Parameter),
    JumpFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    AdjustBase(Parameter),
    Halt,
}

#[derive(Debug)]
pub enum Action {
    Nothing,
    Output(i64),
    Halt,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub data: Vec<i64>,
    pointer: usize,
    pub input: Vec<i64>,
    input_pointer: usize,
    halt_on_output: bool,
    output: Vec<i64>,
    relative_base: i64,
}

impl Program {
    pub fn new(data: Vec<i64>, input: Vec<i64>) -> Self {
        Program {
            data,
            pointer: 0,
            input,
            input_pointer: 0,
            halt_on_output: false,
            output: vec![],
            relative_base: 0,
        }
    }

    pub fn set_available_memory(&mut self, memory: usize) {
        self.data.resize(memory, 0);
    }

    pub fn halt_on_output(&mut self) -> Self {
        self.halt_on_output = true;
        self.clone()
    }

    pub fn read_input(&mut self) -> i64 {
        let value = self.input[self.input_pointer];
        self.input_pointer += 1;
        value
    }

    pub fn write_input(&mut self, value: i64) {
        self.input.push(value);
    }

    pub fn read(&mut self) -> i64 {
        let value = self.data[self.pointer];
        self.pointer += 1;
        value
    }

    pub fn write(&mut self, value: i64, output: &Parameter) {
        match output {
            Parameter::Position(position) => self.data[*position] = value,
            Parameter::Relative(position) => self.data[(*position + self.relative_base) as usize] = value,
            _ => unimplemented!(),
        };
    }

    pub fn jump(&mut self, position: usize) {
        self.pointer = position;
    }

    pub fn step(&mut self) -> Action {
        let instruction = self.read();

        let op_code = instruction % 100;
        let mode1 = (instruction / 100) % 10;
        let mode2 = (instruction / 1000) % 10;
        let mode3 = (instruction / 10000) % 10;

        let instruction = match op_code {
            1 => Instruction::Add(
                Parameter::new(mode1, self.read()),
                Parameter::new(mode2, self.read()),
                Parameter::new(mode3, self.read()),
            ),

            2 => Instruction::Multiply(
                Parameter::new(mode1, self.read()),
                Parameter::new(mode2, self.read()),
                Parameter::new(mode3, self.read()),
            ),

            3 => Instruction::Input(Parameter::new(mode1, self.read())),
            4 => Instruction::Output(Parameter::new(mode1, self.read())),

            5 => Instruction::JumpTrue(
                Parameter::new(mode1, self.read()),
                Parameter::new(mode2, self.read()),
            ),

            6 => Instruction::JumpFalse(
                Parameter::new(mode1, self.read()),
                Parameter::new(mode2, self.read()),
            ),

            7 => Instruction::LessThan(
                Parameter::new(mode1, self.read()),
                Parameter::new(mode2, self.read()),
                Parameter::new(mode3, self.read()),
            ),

            8 => Instruction::Equals(
                Parameter::new(mode1, self.read()),
                Parameter::new(mode2, self.read()),
                Parameter::new(mode3, self.read()),
            ),

            9 => Instruction::AdjustBase(Parameter::new(mode1, self.read())),

            99 => Instruction::Halt,
            _ => unimplemented!(),
        };

        return match instruction {
            Instruction::Add(lhs, rhs, output) => {
                let value = lhs.get(&self.data, self.relative_base)
                    + rhs.get(&self.data, self.relative_base);
                self.write(value, &output);
                Action::Nothing
            }

            Instruction::Multiply(lhs, rhs, output) => {
                let value = lhs.get(&self.data, self.relative_base)
                    * rhs.get(&self.data, self.relative_base);
                self.write(value, &output);
                Action::Nothing
            }

            Instruction::Input(input) => {
                let value = self.read_input();
                self.write(value, &input);
                Action::Nothing
            }

            Instruction::Output(output) => {
                let value = output.get(&self.data, self.relative_base);
                Action::Output(value)
            }

            Instruction::JumpTrue(param, output) => {
                if param.get(&self.data, self.relative_base) != 0 {
                    self.jump(output.get(&self.data, self.relative_base) as usize);
                }

                Action::Nothing
            }

            Instruction::JumpFalse(param, output) => {
                if param.get(&self.data, self.relative_base) == 0 {
                    self.jump(output.get(&self.data, self.relative_base) as usize);
                }

                Action::Nothing
            }

            Instruction::LessThan(lhs, rhs, output) => {
                let lhs = lhs.get(&self.data, self.relative_base);
                let rhs = rhs.get(&self.data, self.relative_base);

                if lhs < rhs {
                    self.write(1, &output);
                } else {
                    self.write(0, &output);
                }

                Action::Nothing
            }

            Instruction::Equals(lhs, rhs, output) => {
                let lhs = lhs.get(&self.data, self.relative_base);
                let rhs = rhs.get(&self.data, self.relative_base);

                if lhs == rhs {
                    self.write(1, &output);
                } else {
                    self.write(0, &output);
                }

                Action::Nothing
            }

            Instruction::AdjustBase(base) => {
                self.relative_base += base.get(&self.data, self.relative_base);

                Action::Nothing
            }

            Instruction::Halt => Action::Halt,
        };
    }

    pub fn execute(&mut self) -> Action {
        loop {
            match self.step() {
                Action::Halt => return Action::Halt,

                Action::Output(value) => {
                    self.output.push(value);
                    if self.halt_on_output {
                        return Action::Output(value);
                    }
                }

                Action::Nothing => (),
            }
        }
    }

    pub fn get_output(&self) -> Vec<i64> {
        self.output.to_vec()
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Parameter, Program};
    pub fn parse_input(input: &str) -> Vec<i64> {
        input
            .split(",")
            .map(|a| a.parse::<i64>().unwrap())
            .collect()
    }

    #[test]
    pub fn test_parse_input() {
        let input = "109,1,204";
        let data = parse_input(input);
        assert_eq!(data, vec![109, 1, 204]);
    }

    #[test]
    pub fn test_parameter_immediate() {
        let param = Parameter::new(1, 100);
        let memory = vec![1, 2, 3];
        assert_eq!(param.get(&memory, 0), 100);
    }

    #[test]
    pub fn test_parameter_position() {
        let param = Parameter::new(0, 1);
        let memory = vec![1, 2, 3];
        assert_eq!(param.get(&memory, 0), 2);
    }

    #[test]
    pub fn test_parameter_relative() {
        let param = Parameter::new(2, 1);
        let memory = vec![1, 2, 3, 4, 5];
        assert_eq!(param.get(&memory, 0), 2);
    }

    #[test]
    pub fn halt_on_output() {
        let data = parse_input("1,2,3,4");
        let input = vec![];
        let mut program = Program::new(data, input);
        program.halt_on_output();
    }

    #[test]
    pub fn test_add() {
        let data = parse_input("1,0,0,0,99");
        let input = vec![];
        let mut program = Program::new(data, input);
        program.execute();
        assert_eq!(program.data, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    pub fn test_multiply() {
        let data = parse_input("2,3,0,3,99");
        let input = vec![];
        let mut program = Program::new(data, input);
        program.execute();
        assert_eq!(program.data, vec![2, 3, 0, 6, 99])
    }

    #[test]
    pub fn test_relative_base() {
        let data = parse_input("1102,34915192,34915192,7,4,7,99,0");
        let mut program = Program::new(data, vec![]);
        program.execute();

        assert_eq!(program.get_output()[0], 1_219_070_632_396_864);
    }
}
