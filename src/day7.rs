use itertools::Itertools;
#[aoc_generator(day7)]
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
pub enum HaltReason {
    Output(i32),
    Finished,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter, bool),
    JumpTrue(Parameter, Parameter),
    JumpFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Halt,
}

#[derive(Clone)]
pub enum Action {
    Output(i32, bool),
    Halt,
}

#[derive(Clone)]
pub struct Program {
    data: Vec<i32>,
    pointer: usize,
    input: Vec<i32>,
    input_pointer: usize,
    halt_on_output: bool,
}

impl Program {
    pub fn new(data: Vec<i32>, input: Vec<i32>) -> Self {
        Program {
            data,
            pointer: 0,
            input,
            input_pointer: 0,
            halt_on_output: false,
        }
    }

    pub fn push_input(&mut self, value: &mut Vec<i32>) {
        self.input.append(value);
    }

    pub fn halt_on_output(&mut self) -> Self {
        self.halt_on_output = true;
        self.clone()
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
            4 => Instruction::Output(Parameter::new(first_mode, self.read()), self.halt_on_output),
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

            Instruction::Output(output, should_halt) => {
                let value = output.get(&self.data);
                action = Some(Action::Output(value, should_halt));
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

pub fn execute_program(program: &mut Program, input: &mut Vec<i32>) -> HaltReason {
    let mut halt_reason = HaltReason::Finished;
    program.push_input(input);

    'main: loop {
        let action = program.step();

        match action {
            Some(Action::Halt) => break 'main,
            Some(Action::Output(value, should_halt)) => {
                if should_halt {
                    halt_reason = HaltReason::Output(value);
                    break 'main;
                }
            }
            None => {}
        }
    }

    halt_reason
}

fn solve_sequence(data: &[i32], sequence: &[i32]) -> i32 {
    let mut programs: Vec<Program> = vec![
        Program::new(data.into(), vec![sequence[0]]).halt_on_output(),
        Program::new(data.into(), vec![sequence[1]]).halt_on_output(),
        Program::new(data.into(), vec![sequence[2]]).halt_on_output(),
        Program::new(data.into(), vec![sequence[3]]).halt_on_output(),
        Program::new(data.into(), vec![sequence[4]]).halt_on_output(),
    ];

    let mut input = vec![0];
    for i in 0..5 {
        let reason = execute_program(&mut programs[i], &mut input);
        match reason {
            HaltReason::Finished => {}

            HaltReason::Output(value) => {
                input = vec![value];
            }
        };
    }

    input[0]
}

fn solve_sequence_feedback(data: &[i32], sequence: &[i32]) -> i32 {
    let mut programs: Vec<Program> = vec![
        Program::new(data.into(), vec![sequence[0]]).halt_on_output(),
        Program::new(data.into(), vec![sequence[1]]).halt_on_output(),
        Program::new(data.into(), vec![sequence[2]]).halt_on_output(),
        Program::new(data.into(), vec![sequence[3]]).halt_on_output(),
        Program::new(data.into(), vec![sequence[4]]).halt_on_output(),
    ];

    let mut terminate = false;
    let mut cur_program = 0;
    let mut last_output = 0;
    let mut input = vec![0];

    while !terminate {
        programs[cur_program].push_input(&mut input);
        let halt_reason = execute_program(&mut programs[cur_program], &mut input);
        match halt_reason {
            HaltReason::Finished => {
                if cur_program == 4 {
                    terminate = true;
                }
            }

            HaltReason::Output(value) => {
                input = vec![value];
                last_output = value;
            }
        };

        cur_program = (cur_program + 1) % 5;
    }

    last_output
}
// #[aoc(day7, part1)]
// fn part_one(data: &[i32]) -> i32 {
//     (0..5)
//         .permutations(5)
//         .map(|sequence| solve_sequence(&data, &sequence))
//         .max()
//         .expect("Failed to find the max")
// }

// fn part_two(data: &[i32]) -> i32 {
//     (5..10).permutations(5)
// }

#[aoc(day7, part1)]
fn day7_part_one(data: &[i32]) -> i32 {
    (0..5)
        .permutations(5)
        .map(|sequence| solve_sequence(data, &sequence))
        .max()
        .expect("Failed to find max")
}

#[aoc(day7, part2)]
fn day7_part_two(data: &[i32]) -> i32 {
    (5..10)
        .permutations(5)
        .map(|sequence| solve_sequence_feedback(data, &sequence))
        .max()
        .expect("Failed to find max")
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, solve_sequence_feedback};
    // #[test]
    // fn day7_part_one() {
    //     let input = generator_input("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    //     let sequence = vec![4, 3, 2, 1, 0];
    //     let output = solve_sequence(&input, &sequence);
    //     assert_eq!(output, 43210);
    // }
}

// #[aoc(day7, part2)]
// fn part_two(data: &[i32]) -> String {
//     let output = execute_program(&data.to_vec(), &vec![5]);

//     format!("{:?}", output)
// }
