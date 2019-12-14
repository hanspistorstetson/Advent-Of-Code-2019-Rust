use crate::intcode::{Action, Program};
use itertools::Itertools;

#[aoc_generator(day7)]
fn generator_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|a| a.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
fn part_one(data: &Vec<i64>) -> i64 {
    (0..5)
        .permutations(5)
        .map(|sequence| solve_sequence(data, &sequence))
        .max()
        .expect("Failed to find max")
}

#[aoc(day7, part2)]
fn part_two(data: &Vec<i64>) -> i64 {
    (5..10)
        .permutations(5)
        .map(|sequence| solve_sequence_feedback(data, &sequence))
        .max()
        .expect("Failed to find max")
}

fn solve_sequence(data: &Vec<i64>, sequence: &[i64]) -> i64 {
    let mut programs: Vec<Program> = vec![
        Program::new(data.to_vec(), vec![sequence[0]]).halt_on_output(),
        Program::new(data.to_vec(), vec![sequence[1]]).halt_on_output(),
        Program::new(data.to_vec(), vec![sequence[2]]).halt_on_output(),
        Program::new(data.to_vec(), vec![sequence[3]]).halt_on_output(),
        Program::new(data.to_vec(), vec![sequence[4]]).halt_on_output(),
    ];

    let mut input = 0;
    for i in 0..5 {
        programs[i].write_input(input);
        programs[i].execute();
        input = programs[i].get_output()[0];
    }

    input
}

fn solve_sequence_feedback(data: &Vec<i64>, sequence: &[i64]) -> i64 {
    let mut programs: Vec<Program> = vec![
        Program::new(data.to_vec(), vec![sequence[0]]).halt_on_output(),
        Program::new(data.to_vec(), vec![sequence[1]]).halt_on_output(),
        Program::new(data.to_vec(), vec![sequence[2]]).halt_on_output(),
        Program::new(data.to_vec(), vec![sequence[3]]).halt_on_output(),
        Program::new(data.to_vec(), vec![sequence[4]]).halt_on_output(),
    ];

    let mut input = 0;
    let mut index = 0;
    let mut terminate = false;

    while !terminate {
        programs[index].write_input(input);
        let action = programs[index].execute();
        match action {
            Action::Output(value) => input = value,

            Action::Halt => {
                if index == programs.len() - 1 {
                    terminate = true;
                }
            }

            _ => unimplemented!(),
        }
        index = (index + 1) % programs.len();
    }

    *programs[programs.len() - 1]
        .get_output()
        .iter()
        .last()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{generator_input, solve_sequence, solve_sequence_feedback};
    use crate::intcode::Program;

    #[test]
    fn test_solve_sequence() {
        let data = generator_input("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let sequence = vec![4, 3, 2, 1, 0];
        assert_eq!(solve_sequence(&data, &sequence), 43210);
    }

    #[test]
    fn test_solve_sequence_feedback() {
        let data = generator_input("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
        let sequence = vec![9, 8, 7, 6, 5];
        assert_eq!(solve_sequence_feedback(&data, &sequence), 139_629_729);
    }
}
