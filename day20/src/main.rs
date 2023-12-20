use std::io::Write;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
};

use itertools::Itertools;
use num::integer::{lcm, ExtendedGcd};
use num::Integer;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Hash)]
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
            println!("{i} {:?}", &connections["vr"]);
        }
        let mut stack = vec![("button".to_owned(), "broadcaster".to_string(), Pulse::Low)];
        while let Some((from, to, received)) = stack.pop() {
            if part2 && to.as_str() == "rx" && received == Pulse::Low {
                return i;
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

#[derive(Clone, Debug, PartialEq, Eq, Default, Hash)]
struct Cycle {
    offset: u64,
    length: u64,
    when_high: Vec<u64>,
}

fn get_high_cycle(
    connections: &BTreeMap<String, Connection>,
    start: &str,
    who_received_high: &str,
) -> Cycle {
    let mut already_seen = HashMap::new();
    let mut connections = connections.clone();
    let mut when_high = Vec::new();
    let mut offset = None;
    let mut length = None;

    for i in 1.. {
        let mut stack = vec![("broadcaster".to_owned(), start.to_string(), Pulse::Low)];
        while let Some((from, to, received)) = stack.pop() {
            if received == Pulse::High && to == who_received_high && when_high.last() != Some(&i) {
                when_high.push(i);
            }
            let cur = connections.get_mut(&to);
            let Some(cur) = cur else { continue };
            let sent = cur.send(&from, received);
            let Some(sent) = sent else { continue };
            for d in cur.dst.iter() {
                stack.push((to.clone(), d.to_string(), sent));
            }
        }
        match already_seen.entry(connections.clone()) {
            std::collections::hash_map::Entry::Occupied(o) => {
                length = Some(i - o.get());
                offset = Some(i - length.unwrap());
                when_high.iter_mut().for_each(|e| *e %= length.unwrap());
                break;
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(i);
            }
        }
    }
    Cycle {
        offset: offset.unwrap(),
        length: length.unwrap(),
        when_high,
    }
}

fn pairwise_chinese(mut a: i128, n: i128, mut b: i128, m: i128) -> (i128, i128) {
    assert!(a >= 0);
    assert!(b >= 0);
    assert!(n >= 0);
    assert!(m >= 0);
    a %= n;
    b %= m;
    let ExtendedGcd {
        gcd: d, x: y, y: z, ..
    } = n.extended_gcd(&m);
    assert_eq!(a % d, b % d);
    assert_eq!(d, y * n + z * m);

    let (mut x, modulo) = (a - y * n * (a - b) / d, (n * m) / d);
    while x <= 0 {
        x += modulo;
        x %= modulo;
    }
    assert_eq!(x % n, a);
    assert_eq!(x % m, b);
    (x, modulo)
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

    let mut dot_file = File::create("/tmp/graph.dot")?;
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
    drop(dot_file);

    let part1 = calc_round(&mut connections.clone(), 1000, false);
    dbg!(part1);
    //let part2 = calc_round(&mut connections.clone(), u64::max_value(), true);
    //dbg!(part2);
    //let connected_components = get_connected_components(&connections, &connections["broadcaster"].dst.clone(), "vr");
    //dbg!(&connected_components);

    let mut cycles = connections["broadcaster"]
        .dst
        .iter()
        .map(|start| get_high_cycle(&connections, start, "vr"))
        .collect_vec();
    dbg!(&cycles);
    for c in &cycles {
        println!(
            "offset: {}, length: {}, when_high: {:?}",
            c.offset, c.length, c.when_high
        )
    }

    let to_chinsese = cycles
        .iter()
        .map(|c| (c.when_high[0] as i128, c.length as i128));
    let mut part2 = to_chinsese
        .reduce(|(a, n), (b, m)| pairwise_chinese(a, n, b, m))
        .unwrap();
    dbg!(&part2);
    while part2.0 < 0 {
        part2.0 += part2.1;
    }
    part2.0 %= part2.1;

    //cycles
    //.iter_mut()
    //.for_each(|c| c.offset = (c.offset + c.when_high[0] + c.length) % c.length);

    //let m = cycles.iter().map(|c| c.length as i64).product::<i64>(); //.reduce(lcm).unwrap();
    //let e = cycles
    //.iter()
    //.map(|c| c.when_high[0] as i64)
    //.map(|o| {
    //let ExtendedGcd {
    //gcd: _, x: _, y, ..
    //} = o.extended_gcd(&(m / o));
    //y * (m / o)
    //})
    //.collect_vec();
    //dbg!(&e);
    //let part2 = (cycles
    //.iter()
    //.zip(e.iter())
    //.map(|(c, e)| c.when_high[0] as i64 * e)
    //.sum::<i64>())
    //% m
    //+ m;

    //dbg!(&part2);

    cycles
        .iter()
        .for_each(|c| assert_eq!(part2.0 % c.length as i128, c.when_high[0] as i128));
    
    //let part2 = part2.0 + 1;
    let part2 = part2.0;// + 1;
    dbg!(&part2);

    Ok(())
}

fn get_connected_components(
    connections: &BTreeMap<String, Connection>,
    start_nodes: &[String],
    until: &str,
) -> Vec<HashSet<String>> {
    let mut rtn = Vec::new();
    for start in start_nodes {
        let mut stack = vec![start];
        let mut visited = HashSet::new();
        while let Some(cur_name) = stack.pop() {
            if !visited.insert(cur_name.clone()) {
                continue;
            }
            if cur_name == until {
                continue;
            }
            let cur = &connections[cur_name];
            for d in &cur.dst {
                stack.push(d);
            }
        }

        rtn.push(visited);
    }

    rtn
}
