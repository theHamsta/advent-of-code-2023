use std::io::Write;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct Connection {
    ty: char,
    memory: bool,
    input_state: BTreeMap<String, Pulse>,
    dst: Vec<String>,
    //last_enabled: (Option<u64>, Option<u64>),
    //last_enabled_distances: (HashSet<u64>, HashSet<u64>),
}

impl Connection {
    fn send(&mut self, from: &str, pulse: Pulse) -> Option<Pulse> {
        match self.ty {
            'B' => Some(pulse),
            '%' => {
                if pulse == Pulse::High {
                    None
                } else {
                    let before = self.memory;
                    self.memory = !self.memory;
                    if before {
                        Some(Pulse::Low)
                    } else {
                        Some(Pulse::High)
                    }
                }
            }
            '&' => {
                *self.input_state.get_mut(from).unwrap() = pulse;
                if self.input_state.values().all(|s| *s == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            _ => unreachable!(),
        }
    }
}

fn calc_round(
    connections: &mut BTreeMap<String, Connection>,
    repetitions: u64,
    part2: bool,
) -> u64 {
    let mut low_counter = 0;
    let mut high_counter = 0;

    for i in 0..repetitions {
        if i % 1_000_000 == 0 {
            println!("{i}");
            dbg!(&connections);
        }
        let mut stack = vec![("button".to_owned(), "broadcaster".to_string(), Pulse::Low)];
        while let Some((from, to, received)) = stack.pop() {
            if part2 && to.as_str() == "rx" && received == Pulse::Low {
                return low_counter + high_counter;
            }
            //dbg!((&from, &to, received));
            //println!("{from} -{received:?}-> {to}");
            if received == Pulse::Low {
                low_counter += 1;
            } else {
                high_counter += 1;
            }
            let cur = connections.get_mut(&to);
            let Some(cur) = cur else { continue };
            //if received == Pulse::Low && cur.ty == '%' {
            //low_counter += 1;
            //if let Some(last) = cur.last_enabled.0 {
            //cur.last_enabled_distances.0.insert(i - last);
            //}
            //cur.last_enabled.0 = Some(i);
            //} else if cur.ty == '&' {
            //high_counter += 1;
            //low_counter += 1;
            //if let Some(last) = cur.last_enabled.1 {
            //cur.last_enabled_distances.1.insert(i - last);
            //}
            //cur.last_enabled.1 = Some(i);
            //}
            let sent = cur.send(&from, received);
            let Some(sent) = sent else { continue };
            for d in cur.dst.iter() {
                stack.push((to.clone(), d.to_string(), sent));
            }
        }
        //println!();
    }
    if part2 {
        unreachable!();
    }
    dbg!(low_counter) * dbg!(high_counter)
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    //let input = include_str!("../example1");
    //154526162
    let connection_regex = regex::Regex::new(r"([%&])?(\w+)\s[-]>\s(\w+.*)").unwrap();
    let dst_regex = regex::Regex::new(r"(\w+),?").unwrap();

    let mut connections = connection_regex
        .captures_iter(input)
        .map(|cap| {
            let dst = dst_regex
                .captures_iter(&cap[3])
                .map(|c| c[1].to_string())
                .collect_vec();
            (
                cap[2].to_string(),
                Connection {
                    ty: cap
                        .get(1)
                        .map(|c| c.as_str().chars().next().unwrap())
                        .unwrap_or('B'),
                    memory: false,
                    input_state: Default::default(),
                    dst,
                    //last_enabled: Default::default(),
                    //last_enabled_distances: Default::default(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let mut dot_file = File::create("foo.dot")?;
    writeln!(dot_file, "digraph {{")?;

    for (name, cur) in connections.clone().iter() {
        for dst in cur.dst.iter() {
            writeln!(dot_file, "{name} -> {dst}")?;
            if let Some(dst_connection) = connections.get_mut(dst) {
                dst_connection
                    .input_state
                    .insert(name.to_owned(), Pulse::Low);
            }
        }
    }
    writeln!(dot_file, "}}")?;

    let mut part1_input = connections.clone();
    println!();
    let part1 = calc_round(&mut part1_input, 1000, false);
    dbg!(part1);
    let mut part2_input = connections.clone();
    let part2 = calc_round(&mut part2_input, u64::max_value(), true);
    dbg!(part2);

    Ok(())
}
