use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");
    //let raw_input = include_str!("../example2");
    //let raw_input = include_str!("../example3");
    //let raw_input = include_str!("../example4");

    let input = raw_input
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect::<Vec<Vec<char>>>();

    let mut empty_rows = (0..input.len()).collect::<HashSet<_>>();
    let mut empty_cols = (0..input[0].len()).collect::<HashSet<_>>();

    let mut galaxies = HashSet::new();

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let c = input[y][x];
            if c == '#' {
                empty_cols.remove(&x);
                empty_rows.remove(&y);
                galaxies.insert((x, y));
            }
        }
    }

    let mut sum = 0;
    for (a, b) in galaxies.iter().tuple_combinations() {
        let mut dist = 0i64;
        for y in a.1.min(b.1)..a.1.max(b.1) {
            if empty_rows.contains(&y) {
                dist += 2;
            } else {
                dist += 1;
            }
        }
        for x in a.0.min(b.0)..a.0.max(b.0) {
            if empty_cols.contains(&x) {
                dist += 2;
            } else {
                dist += 1;
            }
        }
        sum += dist;
    }

    let part1 = sum;
    dbg!(&part1);

    let mut sum = 0;
    for (a, b) in galaxies.iter().tuple_combinations() {
        let mut dist = 0i64;
        for y in a.1.min(b.1)..a.1.max(b.1) {
            if empty_rows.contains(&y) {
                dist += 1000000;
            } else {
                dist += 1;
            }
        }
        for x in a.0.min(b.0)..a.0.max(b.0) {
            if empty_cols.contains(&x) {
                dist += 1000000;
            } else {
                dist += 1;
            }
        }
        sum += dist;
    }

    let part2 = sum;
    dbg!(&part2);

    Ok(())
}
