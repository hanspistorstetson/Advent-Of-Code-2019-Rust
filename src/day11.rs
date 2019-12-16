use crate::intcode::{Action, Program};
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Black,
    White,
}

impl From<Color> for i64 {
    fn from(color: Color) -> i64 {
        match color {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl TryFrom<i64> for Color {
    type Error = ();

    fn try_from(value: i64) -> Result<Color, Self::Error> {
        match value {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            _ => Err(()),
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_turn(&self, turn_dir: i64) -> Direction {
        match turn_dir {
            0 => self.get_left(),
            1 => self.get_right(),
            _ => panic!(),
        }
    }
    fn get_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn move_forward(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[aoc_generator(day11)]
fn generator_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|a| a.parse::<i64>().unwrap())
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position(isize, isize);

fn paint(data: &Vec<i64>, color: Color) -> HashMap<Position, Color> {
    let mut program = Program::new(data.to_vec(), vec![0]).halt_on_output();
    program.set_available_memory(2000);
    let mut panels: HashMap<Position, Color> = Default::default();
    let mut facing = Direction::Up;
    let mut position = Position(0, 0);
    panels.insert(position, color);

    'main: loop {
        let paint_action = program.execute();
        if paint_action == Action::Halt {
            break 'main;
        }

        let dir_action = program.execute();

        if let Action::Output(color) = paint_action {
            let paint_color = Color::try_from(color).unwrap_or(Color::Black);
            panels.insert(position, paint_color);
        }

        if let Action::Output(turn) = dir_action {
            facing = facing.get_turn(turn);
            let (dx, dy) = facing.move_forward();
            position.0 += dx;
            position.1 += dy;
        }

        program.write_input(*panels.entry(position).or_insert(Color::Black) as i64);
    }

    panels
}

#[aoc(day11, part1)]
fn part_one(data: &Vec<i64>) -> String {
    let panels = paint(data, Color::Black);
    format!("{}", panels.len())
}

#[aoc(day11, part2)]
fn part_two(data: &Vec<i64>) -> String {
    let panels = paint(data, Color::White);

    let min_x = panels.keys().map(|p| p.0).min().unwrap();
    let max_x = panels.keys().map(|p| p.0).max().unwrap();
    let min_y = panels.keys().map(|p| p.1).min().unwrap();
    let max_y = panels.keys().map(|p| p.1).max().unwrap();

    let mut output = String::new();
    for y in min_y..max_y {
        for x in min_x..max_x {
            output.push(
                match panels.get(&Position(x, y)).cloned().unwrap_or(Color::Black) {
                    Color::Black => ' ',
                    Color::White => '█',
                },
            )
        }

        output.push('\n');
    }

    println!("{}", output);

    format!("{}", panels.len())
}
