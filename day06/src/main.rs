use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

fn wins(hold: i64, allowed: i64, record: i64) -> bool {
    let remaining = allowed - hold;
    let distance = remaining * hold;
    distance >  record
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");
    let input = raw_input.lines().collect_vec();
    let number_regex = Regex::new(r"\d+").unwrap();
    let allowed_time = number_regex
        .captures_iter(input[0])
        .map(|n| n[0].to_string().parse::<i64>().unwrap())
        .collect_vec();
    let record = number_regex
        .captures_iter(input[1])
        .map(|n| n[0].to_string().parse::<i64>().unwrap())
        .collect_vec();

    let part1: u64 = allowed_time.iter().copied().zip(record.iter().copied()).map(|(time, record)| {
        (1..time).filter(|h| wins(*h, time, record)).count() as u64
    }).product();
    dbg!(&part1);

    let raw_input = raw_input.replace(' ', "");
    let input = raw_input.lines().collect_vec();
    let number_regex = Regex::new(r"\d+").unwrap();
    let allowed_time = number_regex
        .captures_iter(input[0])
        .map(|n| n[0].to_string().parse::<i64>().unwrap())
        .collect_vec();
    let record = number_regex
        .captures_iter(input[1])
        .map(|n| n[0].to_string().parse::<i64>().unwrap())
        .collect_vec();
    let part2: u64 = allowed_time.iter().copied().zip(record.iter().copied()).map(|(time, record)| {
        (1..time).filter(|h| wins(*h, time, record)).count() as u64
    }).product();
    dbg!(&part2);


    Ok(())
}
