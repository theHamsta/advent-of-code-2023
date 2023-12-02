use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Cube {
    number: u64,
}

#[derive(Debug)]
struct Game {
    id: u64,
    bags: Vec<HashMap<String, Cube>>,
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input")
    //let input = include_str!("../example1")
        .lines()
        .filter(|l| !l.is_empty())
        .collect_vec();
    let bag_regex = Regex::new(r"(\d+) (\w+)").unwrap();

    let games = input
        .iter()
        .enumerate()
        .map(|(i, l)| Game {
            id: (i + 1) as u64,
            bags: l
                .split(';')
                .map(|bag| {
                    bag_regex
                        .captures_iter(bag)
                        .map(|m| {
                            (
                                m[2].to_string(),
                                Cube {
                                    number: m[1].parse().unwrap(),
                                },
                            )
                        })
                        .collect::<HashMap<_, _>>()
                })
                .collect_vec(),
        })
        .collect_vec();

    let part1: u64 = games
        .iter()
        .filter(|g| {
            g.bags.iter().all(|b| {
                (!b.contains_key("red") || b["red"].number <= 12)
                    && (!b.contains_key("green") || b["green"].number <= 13)
                    && (!b.contains_key("blue") || b["blue"].number <= 14)
            })
        })
        .map(|g| g.id)
        .sum();
    dbg!(&part1);

    let part2: u64 = games
        .iter()
        .map(|g| {
            let red = g
                .bags
                .iter()
                .filter_map(|b| Some(b.get("red")?.number))
                .max().unwrap_or(0);
            let green = g
                .bags
                .iter()
                .filter_map(|b| Some(b.get("green")?.number))
                .max().unwrap_or(0);
            let blue = g
                .bags
                .iter()
                .filter_map(|b| Some(b.get("blue")?.number))
                .max().unwrap_or(0);
            red * green * blue
        })
        .sum();

    dbg!(&part2);

    Ok(())
}
