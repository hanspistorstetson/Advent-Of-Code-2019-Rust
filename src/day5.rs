use crate::intcode::{Program};

#[aoc_generator(day5)]
fn generator_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|a| a.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
fn part_one(data: &Vec<i64>) -> String {
    let mut program = Program::new(data.to_vec(), vec![1]);
    program.execute();

    format!("{:?}", program.get_output())
}

#[aoc(day5, part2)]
fn part_two(data: &Vec<i64>) -> String {
    let mut program = Program::new(data.to_vec(), vec![5]);
    program.execute();

    format!("{:?}", program.get_output())
}
