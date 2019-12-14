use pathfinding::directed::bfs::bfs;
use std::str::FromStr;

#[aoc_generator(day6)]
fn generator_input(input: &str) -> Vec<OrbitRelation> {
    input
        .lines()
        .map(|s| OrbitRelation::from_str(s).unwrap())
        .collect()
}

pub struct OrbitRelation {
    pub parent: String,
    pub identity: String,
}

impl FromStr for OrbitRelation {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<OrbitRelation, Self::Err> {
        let parts: Vec<&str> = s.split(")").collect();
        Ok(OrbitRelation {
            parent: parts[0].into(),
            identity: parts[1].into(),
        })
    }
}

fn calc_orbits(
    map: &std::collections::HashMap<String, Vec<String>>,
    identity: &str,
    indirects: u32,
) -> u32 {
    let node = map.get(identity);
    match node {
        Some(n) => {
            n.iter()
                .map(|n| calc_orbits(map, n, indirects + 1))
                .sum::<u32>()
                + indirects
        }
        None => indirects,
    }
}

fn calc_distance(
    map: &std::collections::HashMap<String, (Vec<String>, Vec<String>)>,
    start: String,
    end: String,
) -> usize {
    let neighbors = |p: &String| -> Vec<String> {
        let s = map.get(p).unwrap();
        s.0.iter().chain(s.1.iter()).cloned().collect()
    };

    let success = |s: &String| -> bool { s == &end };
    bfs(&start, neighbors, success).unwrap().len()
}

#[aoc(day6, part1)]
fn part_one(input: &[OrbitRelation]) -> u32 {
    let mut map: std::collections::HashMap<String, Vec<String>> = Default::default();
    input.iter().for_each(|or| {
        let entry = map.entry(or.parent.clone()).or_insert(vec![]);
        entry.push(or.identity.clone());
    });

    calc_orbits(&map, "COM", 0)
}

#[aoc(day6, part2)]
fn part_two(input: &[OrbitRelation]) -> usize {
    let mut map: std::collections::HashMap<String, (Vec<String>, Vec<String>)> = Default::default();
    input.iter().for_each(|or| {
        let entry = map.entry(or.parent.clone()).or_insert((vec![], vec![]));
        entry.0.push(or.identity.clone());
        let entry = map.entry(or.identity.clone()).or_insert((vec![], vec![]));
        entry.1.push(or.parent.clone());
    });

    calc_distance(&map, "YOU".into(), "SAN".into())
}
