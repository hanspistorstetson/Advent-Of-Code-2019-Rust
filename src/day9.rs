#[aoc_generator(day5)]
fn generator_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|a| a.parse::<i32>().unwrap())
        .collect()
}

#[derive(Debug, Clone)]
pub enum Parameter {
    Position(u32),
    Immediate(i32),
}

impl Parameter {
    pub fn new(mode: i32, value: i32) -> Parameter {
        match mode {
            0 => Parameter::Position(value as u32),
            1 => Parameter::Immediate(value),
            _ => unimplemented!(),
        }
    }

    pub fn get(&self, memory: &Vec<i32>) -> i32 {
        match self {
            Parameter::Position(position) => memory[*position as usize],
            Parameter::Immediate(value) => *value,
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

#[derive(Clone)]
pub enum Action {
    Output(i32),
    Halt,
}

#[derive(Clone)]
pub struct Program {
    data: Vec<i32>,
    pointer: usize,
    input: Vec<i32>,
    input_pointer: usize,
}

impl Program {
    pub fn new(data: Vec<i32>, input: Vec<i32>) -> Self {
        Program {
            data,
            pointer: 0,
            input,
            input_pointer: 0,
        }
    }

    pub fn read(&mut self) -> i32 {
        let value = self.data[self.pointer];
        self.pointer += 1;
        value
    }

    pub fn read_input(&mut self) -> i32 {
        let value = self.input[self.input_pointer];
        self.input_pointer += 1;
        value
    }

    pub fn write(&mut self, value: i32, parameter: &Parameter) {
        match parameter {
            Parameter::Position(position) => self.data[*position as usize] = value,
            Parameter::Immediate(_) => unimplemented!(),
        };
    }

    pub fn jump(&mut self, pointer: i32) {
        self.pointer = pointer as usize;
    }

    pub fn step(&mut self) -> Option<Action> {
        let instruction = self.read();
        let mut action = None;

        let op_code = instruction % 100;
        let first_mode = (instruction / 100) % 10;
        let second_mode = (instruction / 1000) % 10;
        let third_mode = (instruction / 10000) % 10;

        let instruction = match op_code {
            1 => Instruction::Add(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
                Parameter::new(third_mode, self.read()),
            ),

            2 => Instruction::Multiply(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
                Parameter::new(third_mode, self.read()),
            ),

            3 => Instruction::Input(Parameter::new(first_mode, self.read())),
            4 => Instruction::Output(Parameter::new(first_mode, self.read())),
            5 => Instruction::JumpTrue(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
            ),
            6 => Instruction::JumpFalse(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
            ),

            7 => Instruction::LessThan(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
                Parameter::new(third_mode, self.read()),
            ),

            8 => Instruction::Equals(
                Parameter::new(first_mode, self.read()),
                Parameter::new(second_mode, self.read()),
                Parameter::new(third_mode, self.read()),
            ),

            99 => Instruction::Halt,

            _ => unimplemented!(),
        };

        match instruction {
            Instruction::Add(lhs, rhs, output) => {
                let lhs = lhs.get(&self.data);
                let rhs = rhs.get(&self.data);
                let value = lhs + rhs;

                self.write(value, &output);
            }

            Instruction::Multiply(lhs, rhs, output) => {
                let lhs = lhs.get(&self.data);
                let rhs = rhs.get(&self.data);
                let value = lhs * rhs;

                self.write(value, &output);
            }

            Instruction::Input(input) => {
                let value = self.read_input();
                self.write(value, &input);
            }

            Instruction::Output(output) => {
                let value = output.get(&self.data);
                action = Some(Action::Output(value));
            }

            Instruction::JumpTrue(param, output) => {
                let should_jump = param.get(&self.data) != 0;
                if should_jump {
                    self.jump(output.get(&self.data));
                }
            }

            Instruction::JumpFalse(param, output) => {
                let should_jump = param.get(&self.data) == 0;
                if should_jump {
                    self.jump(output.get(&self.data));
                }
            }

            Instruction::LessThan(lhs, rhs, output) => {
                let lhs = lhs.get(&self.data);
                let rhs = rhs.get(&self.data);
                if lhs < rhs {
                    self.write(1, &output);
                } else {
                    self.write(0, &output);
                }
            }

            Instruction::Equals(lhs, rhs, output) => {
                let lhs = lhs.get(&self.data);
                let rhs = rhs.get(&self.data);

                if lhs == rhs {
                    self.write(1, &output);
                } else {
                    self.write(0, &output);
                }
            }

            Instruction::Halt => {
                action = Some(Action::Halt);
            }

            _ => unimplemented!("What"),
        }

        return action;
    }
}

pub fn execute_program(memory: &Vec<i32>, input: &Vec<i32>) -> Vec<i32> {
    let mut program = Program::new(memory.clone(), input.clone());
    let mut output = Vec::new();

    'main: loop {
        let action = program.step();

        match action {
            Some(Action::Halt) => break 'main,
            Some(Action::Output(value)) => output.push(value),
            None => {}
        }
    }

    output
}

#[aoc(day5, part1)]
fn part_one(data: &[i32]) -> String {
    let output = execute_program(&data.to_vec(), &vec![1]);

    format!("{:?}", output)
}

#[aoc(day5, part2)]
fn part_two(data: &[i32]) -> String {
    let output = execute_program(&data.to_vec(), &vec![5]);

    format!("{:?}", output)
}
