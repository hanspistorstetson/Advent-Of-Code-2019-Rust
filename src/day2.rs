use crate::intcode::{Program};

#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|a| a.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part_one(input: &Vec<i64>) -> i64 {
    solve(input.to_vec(), 12, 2)
}

#[aoc(day2, part2)]
fn part_two(input: &Vec<i64>) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            if solve(input.to_vec(), noun, verb) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }

    0
}

fn solve(input: Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut program = Program::new(input, vec![]);
    program.data[1] = noun;
    program.data[2] = verb;
    program.execute();

    program.data[0]
}
