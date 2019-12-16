use std::collections::{HashMap, HashSet};

#[aoc_generator(day10)]
fn generator_input(input: &str) -> Vec<Asteroid> {
    let mut asteroids = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.push(Asteroid::new(x as isize, y as isize));
            }
        }
    }

    asteroids
}

#[derive(Clone, Eq, Hash, Debug, PartialEq)]
pub struct Direction(isize, isize);

impl Direction {
    pub fn angle(&self) -> f64 {
        let mut angle = (self.1 as f64).atan2(self.0 as f64) - (std::f64::consts::PI / 2.0);

        while angle < 0.0 {
            angle += 2.0 * std::f64::consts::PI;
        }

        angle.to_degrees()
    }
}

impl std::cmp::PartialOrd for Direction {
    fn partial_cmp(&self, other: &Direction) -> Option<std::cmp::Ordering> {
        let self_angle = self.angle();
        let other_angle = other.angle();

        if self_angle.is_nan() || other_angle.is_nan() {
            None
        } else if self_angle > other_angle {
            Some(std::cmp::Ordering::Greater)
        } else if other_angle > self_angle {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl std::cmp::Ord for Direction {
    fn cmp(&self, other: &Direction) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Less)
    }
}

#[derive(Debug, PartialEq)]
pub struct Asteroid {
    pub x: isize,
    pub y: isize,
}

impl Asteroid {
    pub fn new(x: isize, y: isize) -> Asteroid {
        Asteroid { x, y }
    }

    pub fn distance_to(&self, other: &Asteroid) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;

        (dx * dx + dy * dy).sqrt()
    }

    pub fn direction_to(&self, other: &Asteroid) -> Direction {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        let divisor = gcd(dx, dy).abs();

        Direction(dx / divisor, dy / divisor)
    }
}

fn gcd(a: isize, b: isize) -> isize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

#[aoc(day10, part1)]
fn part_one(asteroids: &Vec<Asteroid>) -> String {
    format!("{:?}", find_best_station_count(asteroids))
}

#[aoc(day10, part2)]
fn part_two(asteroids: &Vec<Asteroid>) -> String {
    format!("{:?}", vaporize(asteroids))
}

fn vaporize(asteroids: &Vec<Asteroid>) -> isize {
    let (best_station, _) = find_best_station_count(asteroids);

    let mut list: HashMap<Direction, Vec<&Asteroid>> = HashMap::new();

    for asteroid in asteroids {
        if best_station == asteroid {
            continue;
        }

        let direction = best_station.direction_to(asteroid);
        list.entry(direction).or_default().push(asteroid);
    }

    for asteroids in list.values_mut() {
        asteroids.sort_by_cached_key(|a| (-best_station.distance_to(a) * 1000.0).round() as isize);
    }

    let directions = {
        let mut directions = list.keys().cloned().collect::<Vec<_>>();
        directions.sort();
        directions
    };

    let mut blasted = 0;
    for direction in directions.iter().cycle() {
        let asteroids = list.get_mut(direction).unwrap();

        if asteroids.len() > 0 {
            let asteroid = asteroids.pop().unwrap();
            blasted += 1;

            println!(
                "Blasting asteroid #{} at ({}, {})",
                blasted, asteroid.x, asteroid.y
            );

            if blasted == 200 {
                return asteroid.x * 100 + asteroid.y;
            }
        }
    }

    panic!("Failure")
}

fn find_best_station_count(asteroids: &Vec<Asteroid>) -> (&Asteroid, usize) {
    let mut best_station = &asteroids[0];
    let mut best_count = 0;

    for asteroid in asteroids.iter() {
        let mut station = HashSet::new();
        for other in asteroids.iter() {
            if asteroid != other {
                station.insert(asteroid.direction_to(other));
            }
        }

        if station.len() > best_count {
            best_count = station.len();
            best_station = asteroid;
        }
    }

    (best_station, best_count)
}

#[cfg(test)]
mod tests {
    use super::{find_best_station_count, generator_input, Asteroid};

    #[test]
    fn basic_test() {
        let input = ".#..#\n.....\n#####\n....#\n...##";
        let asteroids = generator_input(input);
        let (asteroid, count) = find_best_station_count(&asteroids);

        assert_eq!(count, 8);
    }
}
