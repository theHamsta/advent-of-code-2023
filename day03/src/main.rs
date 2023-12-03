use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

fn has_symbol_neighbor(input: &Vec<Vec<char>>, (x, y): (i64, i64)) -> bool {
    let width = input[0].len() as i64;
    let height = input.len() as i64;
    for y_ in ((y - 1).max(0))..=((y + 1).min(height - 1)) {
        for x_ in ((x - 1).max(0))..=((x + 1).min(width - 1)) {
            let cur = input[y_ as usize][x_ as usize];
            let is_symbol = !matches!(cur, '.' | '0'..='9');
            if is_symbol {
                return true;
            }
        }
    }
    false
}

fn has_gear_neighbor(input: &Vec<Vec<char>>, (x, y): (i64, i64)) -> Option<(i64, i64)> {
    let width = input[0].len() as i64;
    let height = input.len() as i64;
    for y_ in ((y - 1).max(0))..=((y + 1).min(height - 1)) {
        for x_ in ((x - 1).max(0))..=((x + 1).min(width - 1)) {
            let cur = input[y_ as usize][x_ as usize];
            let is_symbol = cur == '*';
            if is_symbol {
                return Some((x_, y_));
            }
        }
    }
    None
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");
    let chars = raw_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let number_regex = Regex::new(r"\d+").unwrap();
    let mut part1 = 0u64;
    for (y, line) in raw_input.lines().enumerate() {
        for m in number_regex.find_iter(line) {
            for x in m.start()..m.end() {
                let has_neighbor = has_symbol_neighbor(&chars, (x as i64, y as i64));
                if has_neighbor {
                    part1 += m.as_str().parse::<u64>()?;
                    break;
                }
            }
        }
    }
    dbg!(&part1);

    let mut matches = HashMap::new();

    let number_regex = Regex::new(r"\d+").unwrap();
    for (y, line) in raw_input.lines().enumerate() {
        for m in number_regex.find_iter(line) {
            for x in m.start()..m.end() {
                let gear_pos = has_gear_neighbor(&chars, (x as i64, y as i64));
                if let Some(gear_pos) = gear_pos {
                    matches
                        .entry(gear_pos)
                        .or_insert_with(Vec::new)
                        .push(m.as_str().parse::<u64>()?);
                    break;
                }
            }
        }
    }

    let part2: u64 = matches
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .map(|(_, v)| v.iter().product::<u64>())
        .sum();
    dbg!(&part2);

    Ok(())
}
