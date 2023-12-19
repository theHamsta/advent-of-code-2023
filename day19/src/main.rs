use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
struct Check {
    cond_what: String,
    cond_op: char,
    cond_number: i64,
    dst: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
struct WhatToDo {
    checks: Vec<Check>,
    alternative: String,
}

fn is_rating_collection_accepted(
    ratings: &HashMap<String, i64>,
    rules: &HashMap<String, WhatToDo>,
) -> bool {


    true
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    //let input = include_str!("../example1");

    let rules_regex = regex::Regex::new(r"(\w+)\{(.*)\}").unwrap();
    let check_regex = regex::Regex::new(r"(\w+)([<>])(\d+):(\w+)").unwrap();
    let alternative_regex = regex::Regex::new(r",(\w+)$").unwrap();
    let rating_regex = regex::Regex::new(r"\{.*[=].*\}").unwrap();
    let key_value_regex = regex::Regex::new(r"(\w+)[=](\w+)").unwrap();

    let rules = rules_regex
        .captures_iter(input)
        .map(|cap| {
            let alternative = alternative_regex.captures(&cap[2]).unwrap()[1].to_string();
            let checks = check_regex
                .captures_iter(&cap[2])
                .map(|c| Check {
                    cond_what: c[1].to_string(),
                    cond_op: c[2].chars().next().unwrap(),
                    cond_number: c[3].parse().unwrap(),
                    dst: c[4].to_string(),
                })
                .collect_vec();
            (
                cap[1].to_string(),
                WhatToDo {
                    checks,
                    alternative,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let ratings = rating_regex
        .captures_iter(input)
        .map(|cap| {
            key_value_regex
                .captures_iter(&cap[0])
                .map(|c| (c[1].to_string(), c[2].parse::<i64>().unwrap()))
                .collect::<HashMap<_, _>>()
        })
        .collect_vec();

    let part1: i64 = ratings
        .iter()
        .flat_map(|r| is_rating_collection_accepted(r, &rules).then(|| r.values().sum::<i64>()))
        .sum();
    dbg!(&part1);

    Ok(())
}
