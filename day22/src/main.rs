use std::collections::{HashSet, VecDeque};
use std::io::Write;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
};

use itertools::Itertools;
use num::integer::ExtendedGcd;
use num::Integer;

type Point3dInt = i16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point3d {
    x: i16,
    y: i16,
    z: i16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cube {
    id: i16,
    low: Point3d,
    high: Point3d,
}

impl Cube {
    fn supports(&self, other: &Cube) -> bool {
        other.low.z == 1 + self.high.z
            && ((other.low.x..=other.high.x).contains(&self.low.x)
                || (other.low.x..=other.high.x).contains(&self.high.x))
            && ((other.low.y..=other.high.y).contains(&self.low.y)
                || (other.low.y..=other.high.y).contains(&self.high.y))
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    //let input = include_str!("../example1");

    let cube_regex = regex::Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();

    let mut counter = 0;
    let cubes = cube_regex
        .captures_iter(input)
        .map(|cap| {
            counter += 1;
            Cube {
                id: counter,
                low: Point3d {
                    x: cap[1].parse().unwrap(),
                    y: cap[2].parse().unwrap(),
                    z: cap[3].parse().unwrap(),
                },
                high: Point3d {
                    x: cap[4].parse().unwrap(),
                    y: cap[5].parse().unwrap(),
                    z: cap[6].parse().unwrap(),
                },
            }
        })
        .collect_vec();

    let mut supports_which = HashMap::new();
    let mut supported_by = HashMap::new();
    cubes.iter().tuple_combinations().for_each(|(a, b)| {
        if a.supports(b) {
            supports_which
                .entry(a)
                .or_insert_with(HashSet::new)
                .insert(b);
            supported_by.entry(b).or_insert_with(HashSet::new).insert(a);
        } else {
            supports_which.entry(a).or_insert_with(HashSet::new);
            supports_which.entry(b).or_insert_with(HashSet::new);
            supported_by.entry(a).or_insert_with(HashSet::new);
            supported_by.entry(b).or_insert_with(HashSet::new);
        }
    });

    let part1 = cubes
        .iter()
        .filter(|c| {
            //dbg!(&c);
            supports_which[c]
                .iter()
                .all(|d| supported_by.get(d).map(|f| f.len() > 1).unwrap_or(true))
        })
        .count();

    //let part1 = part1.0; // + 1;
    dbg!(&part1);

    //let part2 = part2.0; // + 1;
    //dbg!(&part2);

    Ok(())
}
