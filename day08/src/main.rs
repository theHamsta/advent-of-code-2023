use std::collections::HashMap;

use itertools::Itertools;
use num::{
    integer::{gcd, lcm},
    Integer,
};
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example3");

    let instructions = raw_input.lines().next().unwrap().chars().cycle();
    let network_regex = Regex::new(r"(\w+) = [(](\w+), (\w+)[)]").unwrap();

    let network: HashMap<_, _> = network_regex
        .captures_iter(raw_input)
        .map(|c| (c[1].to_owned(), (c[2].to_owned(), c[3].to_owned())))
        .collect();

    let mut cur = "AAA";
    let mut num_steps = 0u64;
    for i in instructions {
        num_steps += 1;
        cur = match i {
            'L' => &network[cur].0,
            'R' => &network[cur].1,
            _ => unreachable!(),
        };
        if cur == "ZZZ" {
            break;
        }
    }
    dbg!(&num_steps);
    let first_line = raw_input
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .collect_vec();

    let instructions = first_line.iter().cycle();

    let mut cur = network
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|s| s.as_str())
        .collect_vec();
    dbg!(&cur);
    let mut visited = cur.iter().map(|_| HashMap::new()).collect_vec();
    let mut cycles = cur.iter().map(|_| None).collect_vec();
    let mut num_steps = 0u64;
    for (i, ins) in instructions {
        num_steps += 1;
        for (j, (ghost, visited)) in cur.iter_mut().zip(visited.iter_mut()).enumerate() {
            if cycles[j].is_some() {
                continue;
            }
            *ghost = match ins {
                'L' => &network[*ghost].0,
                'R' => &network[*ghost].1,
                _ => unreachable!(),
            };
            if ghost.ends_with('Z') {
                let entry = visited.entry((*ghost, i)).or_insert(num_steps);
                println!("First Z: {num_steps} {:?}", (*ghost, i));
                if *entry != num_steps {
                    dbg!(&num_steps);
                    let period = num_steps - *entry;
                    cycles[j] = Some((*entry, period, i, *ghost));
                    println!("Found cycle {:?}", cycles[j]);
                }
            }
        }
        if cur.iter().all(|c| c.ends_with('Z')) {
            break;
        }
    }
    dbg!(&visited);

    dbg!(&num_steps);
    let lcm = cycles.iter().flatten().map(|a| a.0).reduce(lcm);
    dbg!(&lcm);
    let part2 = lcm;
    dbg!(&part2);
    // Via debug prints is x0 of chinese rest theorem = 0; they all find the Zs at instruction mod 262
    // They need as many steps to get to the first Z as they need to reach their first cycle

    Ok(())
}
