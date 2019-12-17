use crate::intcode::{Action, Program};
use std::collections::HashMap;

#[aoc_generator(day13)]
fn generator_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|a| a.parse::<i64>().unwrap())
        .collect()
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    pub fn new(tile_id: i64) -> Tile {
        match tile_id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Position(i64, i64);

fn draw_tiles(data: Vec<i64>) -> HashMap<Tile, Vec<Position>> {
    let mut program = Program::new(data, vec![]).halt_on_output();
    program.set_available_memory(3000);
    let mut tilemap: HashMap<Tile, Vec<Position>> = HashMap::new();

    'main: loop {
        let x_pos_action = program.execute();
        if let Action::Halt = x_pos_action {
            break 'main;
        }

        let y_pos_action = program.execute();
        let tile_id_action = program.execute();

        if let Action::Output(x_pos) = x_pos_action {
            if let Action::Output(y_pos) = y_pos_action {
                if let Action::Output(tile_id) = tile_id_action {
                    let current_tiles_of_type = tilemap.entry(Tile::new(tile_id)).or_insert(vec![]);
                    current_tiles_of_type.push(Position(x_pos, y_pos))
                }
            }
        }
    }

    tilemap
}

fn play_game(data: Vec<i64>, tilemap: HashMap<Tile, Vec<Position>>) -> i64 {
    let mut program = Program::new(data, vec![]).halt_on_output();
    program.set_available_memory(3000);
    program.data[0] = 2;
    let mut score = 0;

    'main: loop {
    }


    score
}

#[aoc(day13, part1)]
fn part_one(data: &Vec<i64>) -> String {
    let tiles = draw_tiles(data.to_vec());


    format!("{}", tiles.get(&Tile::Block).unwrap().len())
}

#[aoc(day13, part2)]
fn part_two(data: &Vec<i64>) -> String {
    let tilemap = draw_tiles(data.to_vec());
    let score = play_game(data.to_vec(), tilemap);

    format!("{}", score)
}
