#![allow(warnings)]


#[derive(Debug, Clone)]
pub enum Parameter {
    Position(usize),
    Immediate(i64),
}

impl Parameter {
    pub fn new(mode: i64, value: i64) -> Parameter {
        match mode {
            0 => Parameter::Position(value as usize),
            1 => Parameter::Immediate(value),
            _ => unimplemented!(),
        }
    }

    pub fn get(&self, memory: &Vec<i64>) -> i64 {
        match self {
            Parameter::Immediate(value) => *value,
            Parameter::Position(position) => memory[*position],
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
    Halt,
}

#[derive(Debug)]
pub enum Action {
    Nothing,
    Output(i64),
    Halt,
}

#[derive(Debug)]
pub struct Program {
    pub data: Vec<i64>,
    pointer: usize,
    pub input: Vec<i64>,
    input_pointer: usize,
    halt_on_output: bool,
    output: Vec<i64>
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
        }
    }

    pub fn halt_on_output(&mut self) -> &Self {
        self.halt_on_output = true;
        self
    }

    pub fn read_input(&mut self) -> i64 {
        let value = self.input[self.input_pointer];
        self.input_pointer += 1;
        value
    }

    pub fn read(&mut self) -> i64 {
        let value = self.data[self.pointer];
        self.pointer += 1;
        value
    }

    pub fn write(&mut self, value: i64, output: &Parameter) {
        match output {
            Parameter::Position(position) => self.data[*position] = value,
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

            99 => Instruction::Halt,
            _ => unimplemented!(),
        };

        return match instruction {
            Instruction::Add(lhs, rhs, output) => {
                let value = lhs.get(&self.data) + rhs.get(&self.data);
                self.write(value, &output);
                Action::Nothing
            }

            Instruction::Multiply(lhs, rhs, output) => {
                let value = lhs.get(&self.data) * rhs.get(&self.data);
                self.write(value, &output);
                Action::Nothing
            }

            Instruction::Input(input) => {
                let value = self.read_input();
                self.write(value, &input);
                Action::Nothing
            }

            Instruction::Output(output) => {
                let value = output.get(&self.data);
                Action::Output(value)
            }

            Instruction::JumpTrue(param, output) => {
                if param.get(&self.data) != 0 {
                    self.jump(output.get(&self.data) as usize);
                }

                Action::Nothing
            }

            Instruction::JumpFalse(param, output) => {
                if param.get(&self.data) == 0 {
                    self.jump(output.get(&self.data) as usize);
                }

                Action::Nothing
            }

            Instruction::LessThan(lhs, rhs, output) => {
                let lhs = lhs.get(&self.data);
                let rhs = rhs.get(&self.data);

                if lhs < rhs {
                    self.write(1, &output);
                } else {
                    self.write(0, &output);
                }

                Action::Nothing
            }

            Instruction::Equals(lhs, rhs, output) => {
                let lhs = lhs.get(&self.data);
                let rhs = rhs.get(&self.data);

                if lhs == rhs {
                    self.write(1, &output);
                } else {
                    self.write(0, &output);
                }

                Action::Nothing
            }

            Instruction::Halt => Action::Halt,

            _ => unimplemented!(),
        };
    }

    pub fn execute(&mut self) {
        'main: loop {
            match self.step() {
                Action::Halt => break 'main,
                Action::Output(value) => self.output.push(value),
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
        assert_eq!(param.get(&memory), 100);
    }

    #[test]
    pub fn test_parameter_position() {
        let param = Parameter::new(0, 1);
        let memory = vec![1, 2, 3];
        assert_eq!(param.get(&memory), 2);
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
}
