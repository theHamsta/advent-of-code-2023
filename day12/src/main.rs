use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug, Hash)]
struct Row<'a> {
    chars: &'a str,
    spec: Vec<u64>,
}

fn num_arrangements(
    chars: &mut [u8],
    spec: &mut [u64],
    must_pound: bool,
    cache: &mut HashMap<Row, u64>,
) -> u64 {
    if chars.is_empty() {
        if spec.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    //dbg!(String::from_utf8(chars.to_vec()).unwrap());
    match chars[0] {
        b'?' => {
            chars[0] = b'#';
            let mut rtn = num_arrangements(chars, spec, must_pound, cache);
            if !must_pound {
                chars[0] = b'.';
                rtn += num_arrangements(chars, spec, false, cache);
            }
            chars[0] = b'?';
            rtn
        }
        b'#' => {
            if spec.is_empty() {
                return 0;
            }
            spec[0] -= 1;
            let rtn = if spec[0] == 0 {
                // end of group, next must be end of string or '.' or '?' disguised as '.'
                match chars.get(1) {
                    Some(b'.' | b'?') => {
                        // check for gap
                        num_arrangements(&mut chars[2..], &mut spec[1..], false, cache)
                    }
                    Some(b'#') => 0,
                    Some(_) => unreachable!(),
                    None => num_arrangements(&mut [], &mut spec[1..], false, cache),
                }
            } else {
                num_arrangements(&mut chars[1..], spec, true, cache)
            };
            spec[0] += 1;

            rtn
        }
        b'.' => {
            if must_pound {
                0
            } else {
                num_arrangements(&mut chars[1..], spec, false, cache)
            }
        }
        _ => unreachable!(),
    }
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");
    //let raw_input = include_str!("../example2");
    //let raw_input = include_str!("../example3");
    //let raw_input = include_str!("../example4");

    let input = raw_input.lines().filter(|l| !l.is_empty()).collect_vec();

    let mut rows = input
        .iter()
        .map(|i| {
            let mut s = i.split(' ');
            Row {
                chars: s.next().unwrap(),
                spec: s
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect(),
            }
        })
        .collect_vec();

    let mut cache = HashMap::new();
    let part1 = rows
        .iter_mut()
        .map(|n| {
            dbg!(&n);
            let rtn = num_arrangements(
                &mut n.chars.as_bytes().iter().copied().collect_vec(),
                &mut n.spec.clone(),
                false,
                &mut cache,
            );
            dbg!(&rtn);
            rtn
        })
        .sum::<u64>();
    dbg!(&part1);
    //9390

    Ok(())
}
