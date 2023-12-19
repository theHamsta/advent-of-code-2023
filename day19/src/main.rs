use std::collections::HashMap;

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
struct Check {
    what: String,
    op: char,
    number: i32,
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
    let mut cur = "in";
    'outer: loop {
        if cur == "R" {
            return false;
        }
        if cur == "A" {
            return true;
        }
        let rule = &rules[cur];

        for c in &rule.checks {
            match c {
                Check {
                    what,
                    op: '<',
                    number,
                    dst,
                } => {
                    if ratings[what] < (*number).into() {
                        cur = dst.as_str();
                        continue 'outer;
                    }
                }
                Check {
                    what,
                    op: '>',
                    number,
                    dst,
                } => {
                    if ratings[what] > (*number).into() {
                        cur = dst.as_str();
                        continue 'outer;
                    }
                }
                _ => unreachable!(),
            }
        }
        cur = rule.alternative.as_str();
    }
}

fn count_rating_collection_accepted_possibilities(
    ratings: &mut HashMap<String, (i32, i32)>,
    rules: &HashMap<String, WhatToDo>,
    cur: &str,
) -> i64 {
    if cur == "R" {
        return 0;
    }
    if cur == "A" {
        return ratings
            .values()
            .copied()
            .map(|(low, high)| high as i64 + 1 - low as i64)
            .product();
    }
    let rule = &rules[cur];
    let mut sum_accepted = 0;

    for c in &rule.checks {
        match c {
            Check {
                what,
                op: '<',
                number,
                dst,
            } => {
                let (low, high) = ratings[what];
                if low < *number {
                    let mut clone = ratings.clone();
                    *clone.get_mut(what).unwrap() = (low, (*number - 1).min(high));
                    sum_accepted += count_rating_collection_accepted_possibilities(
                        &mut clone,
                        rules,
                        dst.as_str(),
                    );
                    if high < *number {
                        return sum_accepted;
                    }
                    *ratings.get_mut(what).unwrap() = (*number, high);
                }
            }
            Check {
                what,
                op: '>',
                number,
                dst,
            } => {
                let (low, high) = ratings[what];
                if high > *number {
                    let mut clone = ratings.clone();
                    *clone.get_mut(what).unwrap() = (low.max(*number + 1), high);
                    sum_accepted += count_rating_collection_accepted_possibilities(
                        &mut clone,
                        rules,
                        dst.as_str(),
                    );
                    if low > *number {
                        return sum_accepted;
                    }
                    *ratings.get_mut(what).unwrap() = (low, *number);
                }
            }
            _ => unreachable!(),
        }
    }
    sum_accepted +=
        count_rating_collection_accepted_possibilities(ratings, rules, rule.alternative.as_str());
    sum_accepted
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
                    what: c[1].to_string(),
                    op: c[2].chars().next().unwrap(),
                    number: c[3].parse().unwrap(),
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

    let mut rating = HashMap::new();
    rating.insert("x".to_string(), (1, 4000));
    rating.insert("m".to_string(), (1, 4000));
    rating.insert("a".to_string(), (1, 4000));
    rating.insert("s".to_string(), (1, 4000));
    let part2 = count_rating_collection_accepted_possibilities(&mut rating, &rules, "in");

    dbg!(&part2);

    Ok(())
}
