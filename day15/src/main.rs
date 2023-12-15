use std::collections::HashMap;

use itertools::Itertools;

fn ascii_hash(input: &str) -> u64 {
    let mut h = 0u64;
    for c in input.chars() {
        h += c as u64;
        h *= 17;
        h %= 256;
    }
    h
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");
    //let raw_input = include_str!("../example2");
    //let raw_input = include_str!("../example3");
    //let raw_input = include_str!("../example4");

    let hash: u64 = raw_input.split(',').map(|s| ascii_hash(s.trim())).sum();
    dbg!(&hash);

    let instruction_regex = regex::Regex::new(r"(\w+)([=-])(\d+)?").unwrap();
    let mut boxes = vec![HashMap::<String, (usize, u64)>::new(); 256];

    raw_input.split(',').enumerate().for_each(|(i, s)| {
        let cap = instruction_regex.captures(s).unwrap();
        let key = &cap[1];
        let hash = ascii_hash(key);
        let op = &cap[2];
        let val = &cap.get(3);
        match op {
            "-" => {
                boxes[hash as usize].remove(key);
            }
            "=" => {
                let val = val.unwrap().as_str().parse::<u64>().unwrap();
                match boxes[hash as usize].entry(key.to_string()) {
                    std::collections::hash_map::Entry::Occupied(mut o) => {
                        o.get_mut().1 = val;
                    }
                    std::collections::hash_map::Entry::Vacant(v) => {
                        v.insert((i, val));
                    }
                };
            }
            _ => unreachable!(),
        };
    });

    let part2: u64 = boxes
        .iter()
        .enumerate()
        .map(|(box_idx, b)| {
            b.iter()
                .sorted_by_key(|(_k, (i, _))| i)
                .enumerate()
                //.inspect(|(slot_idx, slot)| {
                //dbg!(box_idx + 1);
                //dbg!(slot_idx + 1);
                //dbg!(&slot);
                //})
                .map(|(slot_idx, slot)| (box_idx + 1) as u64 * (slot_idx + 1) as u64 * slot.1 .1)
                //.inspect(|s| {
                //dbg!(&s);
                //})
                .sum::<u64>()
        })
        .sum();
    dbg!(&part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_name_test() {
        assert_eq!(ascii_hash("HASH"), 52);
    }
}
