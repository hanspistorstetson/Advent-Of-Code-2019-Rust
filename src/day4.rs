#[aoc_generator(day4)]
fn generator_input(input: &str) -> (u32, u32) {
    let input_numbers = input
        .split("-")
        .map(|x| x.parse::<u32>().expect("Failed to parse u32"))
        .collect::<Vec<u32>>();
    (input_numbers[0], input_numbers[1])
}

fn pass_match(x: &str) -> bool {
    if x.len() != 6 {
        return false;
    }
    let mut has_pair = false;
    for idx in 1..x.len() {
        let current_number = x.chars().nth(idx);
        let prev_number = x.chars().nth(idx - 1);
        if current_number.unwrap().to_string().parse::<u32>().unwrap()
            < prev_number.unwrap().to_string().parse::<u32>().unwrap()
        {
            return false;
        }
        if current_number == prev_number {
            has_pair = true;
        }
    }
    has_pair
}

fn pass_match2(x: &str) -> bool {
    let occurences: std::collections::HashMap<char, usize> =
        x.chars().fold(Default::default(), |mut state, c| {
            let entry = state.entry(c).or_insert(0);
            *entry += 1;
            state
        });

    occurences.values().any(|x| *x == 2)
}

#[aoc(day4, part1)]
fn part_one((a, b): &(u32, u32)) -> usize {
    (*a..=*b)
        .map(|x| format!("{}", x))
        .filter(|x| pass_match(x))
        .count()
}

#[aoc(day4, part2)]
fn part_two((a, b): &(u32, u32)) -> usize {
    (*a..=*b)
        .map(|x| format!("{}", x))
        .filter(|x| pass_match(x))
        .filter(|x| pass_match2(x))
        .count()
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part_one};
}
