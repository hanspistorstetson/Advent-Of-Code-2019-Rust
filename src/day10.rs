#[aoc_generator(day10)]
fn generator_input(input: &str) -> Vec<Asteroid> {
    let mut asteroids = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.push(Asteroid::new(x, y));
            }
        }
    }


    asteroids
}

#[derive(Debug, PartialEq)]
pub struct Asteroid {
    x: usize,
    y: usize,
}

impl Asteroid {
    pub fn new(x: usize, y: usize) -> Asteroid {
        Asteroid { x, y }
    }

    pub fn distance_to(&self, other: &Asteroid) -> f64 {
        let dx = (other.x - self.x) as f64;
        let dy = (other.y - self.y) as f64;

        (dx * dx + dy * dy).sqrt()
    }
}

#[aoc(day10, part1)]
fn part_one(asteroids: &Vec<Asteroid>) -> String {
    format!("{:?}", asteroids)
}
