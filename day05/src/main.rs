use std::{
    collections::{HashMap, HashSet},
    process::exit,
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Dict {
    dest: String,
    dict: HashMap<i64, (i64, i64)>,
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");
    let input = raw_input;
    let sections = input.split("\n\n");

    let seed_regex = Regex::new(r"(\d+)").unwrap();
    let map_regex = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
    let entry_regex = Regex::new(r"(\d+)\s+(\d+)\s+(\d+)").unwrap();

    let seeds = seed_regex
        .captures_iter(input.lines().next().unwrap())
        .map(|c| c[1].parse::<i64>().unwrap())
        .collect_vec();

    let mut dicts = HashMap::new();
    for s in sections {
        let title = map_regex.captures(s);
        if let Some(title) = title {
            let mut dict = HashMap::new();

            for map in entry_regex.captures_iter(s) {
                let dst_start = map[1].parse::<i64>().unwrap();
                let src_start = map[2].parse::<i64>().unwrap();
                let length = map[3].parse::<i64>().unwrap();
                dict.insert(src_start, (dst_start, length));
            }
            dicts.insert(
                title[1].to_string(),
                Dict {
                    dest: title[2].to_string(),
                    dict,
                },
            );
        }
    }

    let mapping_chain = [
        "seed",
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
    ];

    dbg!(&seeds);
    let mut locations = Vec::new();
    for &s in seeds.iter() {
        let mut cur = s;
        for m in mapping_chain {
            //println!("{m} {cur}");
            //dbg!(&dicts[m].dict);
            cur = dicts[m]
                .dict
                .iter()
                .find(|(&k, &v)| cur >= k && cur < k + v.1)
                .map(|(k, v)| v.0 + (cur - k))
                .unwrap_or(cur);
        }
        locations.push(cur);
    }

    let part1 = locations.iter().min().unwrap();
    dbg!(&part1);

    let mut min = i64::max_value();
    let total_seeds = (seeds[0]..(seeds[0] + seeds[1]))
        .chain(seeds[2]..(seeds[2] + seeds[3]))
        .count();
    for (i, s) in (seeds[0]..(seeds[0] + seeds[1]))
        .chain(seeds[2]..(seeds[2] + seeds[3]))
        .enumerate()
    {
        if i % 1000 == 0 {
            println!("{:.02}", i as f32 / total_seeds as f32 * 100.0);
        }
        let mut cur = s;
        for m in mapping_chain {
            //dbg!(&dicts[m].dict);
            cur = dicts[m]
                .dict
                .iter()
                .find(|(&k, &v)| cur >= k && cur < k + v.1)
                .map(|(k, v)| v.0 + (cur - k))
                .unwrap_or(cur);
        }
        min = min.min(cur);
    }

    let part2 = min;
    dbg!(&part2);
    Ok(())
}
