use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn calc_reflection_sum(input: &[Vec<char>]) -> u64 {
    let mut sum = 0u64;

    for y in 0..(input.len() as i64) {
        let mut is_symetric = true;
        let mut has_one_comparision = false;
        'outer: for y_ in 1..(input.len() as i64) {
            let low_y = y - y_ + 1;
            let high_y = y + y_;
            if low_y < 0 || high_y >= (input.len() as i64) {
                continue;
            }
            for x in 0..(input[0].len() as i64) {
                has_one_comparision = true;
                if input[low_y as usize][x as usize] != input[high_y as usize][x as usize] {
                    is_symetric = false;
                    break 'outer;
                }
            }
        }
        if is_symetric && has_one_comparision {
            sum += (y as u64 + 1) * 100;
        }
    }

    for x in 0..(input[0].len() as i64) {
        let mut is_symetric = true;
        let mut has_one_comparision = false;
        'outer: for x_ in 1..(input.len() as i64) {
            let low_x = x - x_ + 1;
            let high_x = x + x_;
            if low_x < 0 || high_x >= (input[0].len() as i64) {
                continue;
            }
            for y in 0..(input.len() as i64) {
                has_one_comparision = true;
                if input[y as usize][low_x as usize] != input[y as usize][high_x as usize] {
                    is_symetric = false;
                    break 'outer;
                }
            }
        }
        if is_symetric && has_one_comparision {
            sum += x as u64 + 1;
        }
    }
    sum
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");
    //let raw_input = include_str!("../example2");
    //let raw_input = include_str!("../example3");
    //let raw_input = include_str!("../example4");

    let input = raw_input
        .split("\n\n")
        .map(|l| {
            l.lines()
                .map(|l| l.chars().collect_vec())
                .collect::<Vec<Vec<char>>>()
        })
        .collect_vec();

    let part1: u64 = input.iter().map(|i| calc_reflection_sum(i)).sum();
    dbg!(&part1);

    Ok(())
}
