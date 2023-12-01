use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input")
        //let input = include_str!("../input/example2.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .collect_vec();

    let part1: u64 = input
        .iter()
        .flat_map(|line| {
            let it = line.chars().filter(|a| a.is_ascii_digit());
            let first: char = it.clone().next()?;
            let second: char = it.last()?;
            format!("{first}{second}").parse::<u64>().ok()
        })
        .sum();
    dbg!(&part1);

    let mut values = HashMap::new();
    values.insert("0", 0);
    values.insert("1", 1);
    values.insert("2", 2);
    values.insert("3", 3);
    values.insert("4", 4);
    values.insert("5", 5);
    values.insert("6", 6);
    values.insert("7", 7);
    values.insert("8", 8);
    values.insert("9", 9);
    //values.insert("zero", 0);
    values.insert("one", 1);
    values.insert("two", 2);
    values.insert("three", 3);
    values.insert("four", 4);
    values.insert("five", 5);
    values.insert("six", 6);
    values.insert("seven", 7);
    values.insert("eight", 8);
    values.insert("nine", 9);

    let first_regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine).*").unwrap();
    let last_regex = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let part2: u64 = input
        .iter()
        .flat_map(|line| {
            let first = &first_regex.captures(line)?[1];
            let second = &last_regex.captures(line)?[1];

            let first = values[first];
            let second = values[second];

            Some(first * 10 + second)
        })
        .sum();
    dbg!(&part2);
    Ok(())
}
