use nalgebra as na;
use std::collections::HashSet;
use std::fmt;
use num::integer::lcm;

#[aoc_generator(day12)]
fn generator_input(input: &str) -> Vec<Moon> {
    input
        .lines()
        .map(|line| {
            let linepart = &line[1..line.len() - 1];
            let parts = linepart.split(",").collect::<Vec<_>>();
            let x: isize = parts[0].trim()[2..].parse().unwrap();
            let y: isize = parts[1].trim()[2..].parse().unwrap();
            let z: isize = parts[2].trim()[2..].parse().unwrap();
            Moon::new(x, y, z)
        })
        .collect()
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Moon {
    pub position: na::Vector3<isize>,
    pub velocity: na::Vector3<isize>,
}

impl Moon {
    pub fn new(x: isize, y: isize, z: isize) -> Moon {
        Moon {
            position: na::Vector3::new(x, y, z),
            velocity: na::Vector3::new(0, 0, 0),
        }
    }

    pub fn kinetic_energy(&self) -> isize {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    pub fn potential_energy(&self) -> isize {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    pub fn total_energy(&self) -> isize {
        self.kinetic_energy() * self.potential_energy()
    }

    pub fn timestep(&mut self) {
        self.position += self.velocity;
    }

    pub fn gravity_for(&mut self, other: &Moon) {
        if self.position.x > other.position.x {
            self.velocity.x -= 1
        } else if self.position.x < other.position.x {
            self.velocity.x += 1
        };

        if self.position.y > other.position.y {
            self.velocity.y -= 1
        } else if self.position.y < other.position.y {
            self.velocity.y += 1
        };

        if self.position.z > other.position.z {
            self.velocity.z -= 1
        } else if self.position.z < other.position.z {
            self.velocity.z += 1
        };
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "pos=<x={},y={},z={}>, vel=<x={}, y={}, z={}>",
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z
        )
    }
}

fn simulate(mut moons: Vec<Moon>, steps: usize) -> isize {
    for i in 0..steps {
        println!("Step #{}", i);
        for moon in &moons {
            println!("{}", moon);
        }
        step(&mut moons);
    }

    moons.iter().map(|moon| moon.total_energy()).sum()
}

fn step(moons: &mut Vec<Moon>) {
    gravity(moons);
    velocity(moons);
}

fn gravity(moons: &mut Vec<Moon>) {
    let moons2 = moons.clone();
    for moon in moons {
        for othermoon in &moons2 {
            if moon == othermoon {
                continue;
            }

            moon.gravity_for(othermoon);
        }
    }
}

fn velocity(moons: &mut Vec<Moon>) {
    for moon in moons {
        moon.timestep();
    }
}

#[aoc(day12, part1)]
fn part_one(moons: &Vec<Moon>) -> String {
    let total_energy = simulate(moons.to_vec(), 1000);
    format!("{:?}", total_energy)
}

fn dimension_state(moons: &Vec<Moon>, dim: usize) -> [isize; 8] {
    match dim {
        0 => [
            moons[0].position.x,
            moons[0].velocity.x,
            moons[1].position.x,
            moons[1].velocity.x,
            moons[2].position.x,
            moons[2].velocity.x,
            moons[3].position.x,
            moons[3].velocity.x,
        ],

        1 => [
            moons[0].position.y,
            moons[0].velocity.y,
            moons[1].position.y,
            moons[1].velocity.y,
            moons[2].position.y,
            moons[2].velocity.y,
            moons[3].position.y,
            moons[3].velocity.y,
        ],

        2 => [
            moons[0].position.z,
            moons[0].velocity.z,
            moons[1].position.z,
            moons[1].velocity.z,
            moons[2].position.z,
            moons[2].velocity.z,
            moons[3].position.z,
            moons[3].velocity.z,
        ],

        _ => panic!(),
    }
}

fn repeat(mut moons: Vec<Moon>) -> usize {
    let init_state = [
        dimension_state(&moons, 0),
        dimension_state(&moons, 1),
        dimension_state(&moons, 2)
    ];

    let mut cycle_len = [0; 3];


    let mut step = 0;
    while cycle_len[0] == 0 || cycle_len[1] == 0 || cycle_len[2] == 0 {
        gravity(&mut moons);
        velocity(&mut moons);
        step += 1;

        for dim in 0..3 {
            if cycle_len[dim] == 0 && dimension_state(&moons, dim) == init_state[dim] {
                cycle_len[dim] = step;
            }
        }
    }

    lcm(cycle_len[0], lcm(cycle_len[1], cycle_len[2]))

}

#[aoc(day12, part2)]
fn part_two(input: &Vec<Moon>) -> String {
    let mut moons = input.to_vec();

    format!("{}", repeat(moons))
}

#[cfg(test)]
mod test {
    use super::{generator_input, repeat, simulate};

    #[test]
    fn basic_test() {
        let input = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>";
        let moons = generator_input(input);
        let total_energy = simulate(moons, 10);
        assert_eq!(total_energy, 179);
    }

    #[test]
    fn bigger_test() {
        let input = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>";
        let moons = generator_input(input);
        let total_energy = simulate(moons, 100);
        assert_eq!(total_energy, 1940);
    }

    #[test]
    fn repeat_test() {
        let input = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>";
        let mut moons = generator_input(input);
        let total_steps = repeat(moons);
        assert_eq!(total_steps, 4686774924);
    }
}
