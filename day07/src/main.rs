use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Kind {
    High,
    One,
    Pair,
    Tripple,
    TwoPair,
    FiveOfAKind,
}

#[derive(Debug)]
struct Round {
    card: [u32; 5],
    unsorted: Vec<usize>,
    bid: u64,
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");

    let card_regex = Regex::new(r"(\w+) (\d+)").unwrap();

    let mut rounds = card_regex
        .captures_iter(raw_input)
        .flat_map(|c| {
            let mut unsorted = HashMap::new();
            for l in c[1].chars() {
                *unsorted.entry(l).or_default() += 1;
            }
            let mut unsorted: Vec<_> = unsorted.values().copied().collect_vec();
            unsorted.sort();
            unsorted = unsorted.iter().copied().rev().collect_vec();
            let round = Round {
                card: c[1]
                    .chars()
                    .map(|c| {
                        if c.is_ascii_digit() {
                            c as u32 - '0' as u32
                        } else {
                            match c {
                                'T' => 10,
                                'J' => 11,
                                'Q' => 12,
                                'K' => 13,
                                'A' => 14,
                                _ => unreachable!(),
                            }
                        }
                    })
                    .collect_vec()
                    .try_into()
                    .ok()?,
                unsorted,
                bid: c[2].parse().ok()?,
            };
            Some(round)
        })
        .collect_vec();

    rounds.sort_by(|a, b| a.card.cmp(&b.card));
    rounds.sort_by(|a, b| a.unsorted.cmp(&b.unsorted));

    dbg!(&rounds);

    let part1: u64 = rounds
        .iter()
        .enumerate()
        .map(|(i, r)| (i + 1) as u64 * r.bid)
        .sum();
    dbg!(&part1);

    let mut rounds = card_regex
        .captures_iter(raw_input)
        .flat_map(|c| {
            let mut unsorted = HashMap::new();
            let mut joker_count = 0;
            for l in c[1].chars() {
                if l == 'J' {
                    joker_count += 1;
                } else {
                    *unsorted.entry(l).or_default() += 1;
                }
            }
            let mut unsorted: Vec<_> = unsorted.values().copied().collect_vec();
            unsorted.sort();
            unsorted = unsorted.iter().copied().rev().collect_vec();
            if unsorted.is_empty() {
                unsorted.push(joker_count);
            } else {
                unsorted[0] += joker_count;
            }
            let round = Round {
                card: c[1]
                    .chars()
                    .map(|c| {
                        if c.is_ascii_digit() {
                            c as u32 - '0' as u32
                        } else {
                            match c {
                                'T' => 10,
                                'J' => 0,
                                'Q' => 12,
                                'K' => 13,
                                'A' => 14,
                                _ => unreachable!(),
                            }
                        }
                    })
                    .collect_vec()
                    .try_into()
                    .ok()?,
                unsorted,
                bid: c[2].parse().ok()?,
            };
            Some(round)
        })
        .collect_vec();

    rounds.sort_by(|a, b| a.card.cmp(&b.card));
    rounds.sort_by(|a, b| a.unsorted.cmp(&b.unsorted));

    dbg!(&rounds);

    let part1: u64 = rounds
        .iter()
        .enumerate()
        .map(|(i, r)| (i + 1) as u64 * r.bid)
        .sum();
    dbg!(&part1);

    Ok(())
}
