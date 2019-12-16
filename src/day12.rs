use nalgebra as na;

#[aoc_generator(day12)]
fn generator_input(input: &str) -> Vec<Moon> {
    input
        .lines()
        .map(|line| {
            let linepart = &line[1..line.len() - 1];
            let parts = linepart.split(",").collect::<Vec<_>>();
            let x: i32 = parts[0].trim()[2..].parse().unwrap();
            let y: i32 = parts[1].trim()[2..].parse().unwrap();
            let z: i32 = parts[2].trim()[2..].parse().unwrap();
            Moon::new(x, y, z)
        })
        .collect()
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Moon {
    pub position: na::Vector3<i32>,
    pub velocity: na::Vector3<i32>,
}

impl Moon {
    pub fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            position: na::Vector3::new(x, y, z),
            velocity: na::Vector3::new(0, 0, 0),
        }
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

fn step(moons: &mut Vec<Moon>) {
    gravity(moons);
    velocity(moons);
}

fn gravity(moons: &Vec<Moon>) {
    for moon in moons.iter() {
        for other_moon in moons.iter_mut() {}
    }
    for moon in moons.iter() {
        for other_moon in moons {
            if moon == other_moon {
                continue;
            }

            moon.gravity_for(other_moon);
        }
    }
}

fn velocity(moons: &Vec<Moon>) {}

#[aoc(day12, part1)]
fn part_one(moons: &mut Vec<Moon>) -> String {
    step(moons);
    format!("{:?}", 5)
}
