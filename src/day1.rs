#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<i32> {
    input.lines().map(|a| a.parse::<i32>().unwrap()).collect()
}

fn calc_mass(x: &i32) -> i32 {
    x / 3 - 2
}

fn calc_total_mass(x: &i32) -> i32 {
    (0..)
        .scan(*x, |xx, _| {
            *xx = calc_mass(xx);
            if *xx > 0 {
                return Some(*xx);
            }

            None
        })
        .sum()
}

#[aoc(day1, part1)]
fn part_one(input: &[i32]) -> i32 {
    input.iter().map(calc_mass).sum()
}

#[aoc(day1, part2)]
fn part_two(input: &[i32]) -> i32 {
    input.iter().map(calc_total_mass).sum()
}
