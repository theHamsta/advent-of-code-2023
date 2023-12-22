use std::collections::VecDeque;
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
    a: Point3d,
    b: Point3d,
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    let input = include_str!("../example1");

    let cube_regex = regex::Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();

    let mut cubes = cube_regex
        .captures_iter(input)
        .map(|cap| Cube {
            a: Point3d {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                z: cap[3].parse().unwrap(),
            },
            b: Point3d {
                x: cap[4].parse().unwrap(),
                y: cap[5].parse().unwrap(),
                z: cap[6].parse().unwrap(),
            },
        })
        .collect_vec();
    dbg!(&cubes);

    //let part1 = part1.0; // + 1;
    //dbg!(&part1);

    //let part2 = part2.0; // + 1;
    //dbg!(&part2);

    Ok(())
}
