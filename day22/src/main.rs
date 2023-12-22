use std::collections::{HashSet, VecDeque};
use std::io::Write;
use std::{collections::HashMap, fs::File};

use itertools::Itertools;
use num::integer::ExtendedGcd;
use num::Integer;

type Point3dInt = i16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point3d {
    x: Point3dInt,
    y: Point3dInt,
    z: Point3dInt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cube {
    id: i16,
    low: Point3d,
    high: Point3d,
}

impl Cube {
    fn supports(&self, other: &Cube) -> bool {
        if self.id == other.id {
            return false;
        }
        other.low.z == 1 + self.high.z
            && other.low.x.max(self.low.x) <= other.high.x.min(self.high.x)
            && other.low.y.max(self.low.y) <= other.high.y.min(self.high.y)
    }
}

fn play_tetris(cubes: &mut [Cube], without: i16) {
    loop {
        let mut moved = false;
        for i in 0..cubes.len() {
            let copy = cubes[i];
            if copy.low.z > 1
                && !cubes
                    .iter()
                    .any(|other| other.id != without && other.supports(&copy))
            {
                let c = cubes.get_mut(i).unwrap();
                c.low.z -= 1;
                c.high.z -= 1;
                moved = true;
            }
        }
        if !moved {
            break;
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    //let input = include_str!("../example1");

    let cube_regex = regex::Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();

    let mut counter = 0;
    let mut cubes = cube_regex
        .captures_iter(input)
        .map(|cap| {
            let cube = Cube {
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
            };
            assert!(cube.low.x <= cube.high.x);
            assert!(cube.low.y <= cube.high.y);
            assert!(cube.low.z <= cube.high.z);
            counter += 1;
            cube
        })
        .collect_vec();

    play_tetris(&mut cubes, -1);

    let mut supports_which = HashMap::new();
    let mut supported_by = HashMap::new();
    cubes.iter().permutations(2).for_each(|vec| {
        let a = vec[0];
        let b = vec[1];
        if a.supports(b) {
            supports_which
                .entry(a)
                .or_insert_with(HashSet::new)
                .insert(b);
            supported_by.entry(b).or_insert_with(HashSet::new).insert(a);
        } else {
            supports_which.entry(a).or_insert_with(HashSet::new);
            supported_by.entry(b).or_insert_with(HashSet::new);
        }
    });

    let mut file = File::create("/tmp/foo.dot")?;
    writeln!(file, "digraph {{")?;
    for (k, v) in supports_which.iter() {
        if !v.is_empty() {
            writeln!(
                file,
                "{} -> {}",
                k.id,
                v.iter().map(|v| format!("{}", v.id)).join(",")
            );
        }
    }
    writeln!(file, "}}")?;

    let part1 = cubes
        .iter()
        .filter(|c| supports_which[c].iter().all(|d| supported_by[d].len() > 1))
        .count();

    dbg!(&part1);

    let part2: u64 = cubes
        .iter()
        .map(|c| {
            let mut copy = cubes.clone();
            play_tetris(&mut copy, c.id);
            let mut count = 0;
            for i in 0..cubes.len() {
                if copy[i] != cubes[i] {
                    count += 1;
                }
            }
            count
        })
        //.inspect(|c| println!("{}", c.id))
        .sum();
    dbg!(part2);

    Ok(())
}
