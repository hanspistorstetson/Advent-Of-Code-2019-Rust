use crate::intcode::{Program};

#[aoc_generator(day9)]
fn generator_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|a| a.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day9, part1)]
fn part_one(data: &Vec<i64>) -> i64 {
    let mut program = Program::new(data.to_vec(), vec![1]);
    program.set_available_memory(2000);
    program.execute();

    program.get_output()[0]
}

#[aoc(day9, part2)]
fn part_two(data: &Vec<i64>) -> i64 {
    let mut program = Program::new(data.to_vec(), vec![2]);
    program.set_available_memory(2000);
    program.execute();

    program.get_output()[0]
}

#[cfg(test)]
mod test {
    use crate::intcode::{Program};
    use super::generator_input;

    #[test]
    fn test_relative_base() {
    }
}
