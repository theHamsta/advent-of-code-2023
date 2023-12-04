use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");
    let input = raw_input.lines().filter(|l| !l.is_empty()).collect_vec();

    let number_regex = Regex::new(r"\d+").unwrap();

    let cards = input
        .iter()
        .map(|l| l.split(':').nth(1).unwrap())
        .map(|c| {
            c.split('|')
                .map(|c| {
                    number_regex
                        .find_iter(c)
                        .map(|c| c.as_str())
                        .collect::<HashSet<_>>()
                })
                .collect_vec()
        })
        .collect_vec();

    let part1: u64 = cards
        .iter()
        .map(|c| {
            let mut total = 0u64;
            for cur in c[1].iter() {
                if c[0].contains(cur) {
                    total += 1;
                }
            }
            (2_i32.pow(total as u32) / 2) as u64
        })
        .sum();
    dbg!(&part1);

    let mut copies = vec![1; cards.len()];
    cards.iter().enumerate().for_each(|(i, c)| {
        let mut num_wins = 0u64;
        for cur in c[1].iter() {
            if c[0].contains(cur) {
                num_wins += 1;
            }
        }
        for j in 0..num_wins {
            if (i + 1 + j as usize) < copies.len() {
                copies[i + 1 + j as usize] += copies[i];
            }
        }
    });

    let part2: u64 = copies.iter().sum();
    dbg!(&part2);

    Ok(())
}
