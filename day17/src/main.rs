use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[allow(dead_code)]
fn plot(input: &Vec<Vec<char>>) {
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            print!("{}", input[y][x]);
        }
        println!();
    }
    println!();
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Repetitions {
    repetitions: u32,
    dir: (i64, i64),
}

type Key = ((i64, i64), Repetitions);
//type Key = ((i64, i64), VecDeque<(i64, i64)>);
//type Key = (i64, i64);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pos {
    pos: (i64, i64),
    repetitions: Repetitions,
    heat_loss: i64,
    prev: Key,
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    //let input = include_str!("../example1");
    //let input = include_str!("../example2");

    let input = input.lines().map(|s| s.chars().collect_vec()).collect_vec();

    let mut pq = priority_queue::PriorityQueue::new();

    pq.push(
        Pos {
            pos: (0, 0),
            repetitions: Default::default(),
            heat_loss: 0,
            prev: Key::default(),
        },
        0i64,
    );

    let mut reached = HashMap::<Key, Pos>::new();
    let goal = (input[0].len() as i64 - 1, input.len() as i64 - 1);

    dbg!(&goal);

    let mut counter = 0;
    while let Some((current, _)) = pq.pop() {
        let (x, y) = current.pos;
        counter += 1;
        if counter % 10000 == 0 {
            dbg!(&pq.len());
        }

        let key = (current.pos, current.repetitions.clone());
        //let key = current.pos;
        match reached.entry(key.clone()) {
            std::collections::hash_map::Entry::Occupied(mut o) => {
                if o.get().heat_loss > current.heat_loss {
                    o.insert(current.clone());
                    unreachable!();
                } else {
                    continue;
                }
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(current.clone());
            }
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if (-dx, -dy) == current.repetitions.dir {
                continue;
            }

            let new_pos = (x + dx, y + dy);
            if new_pos.1 < 0
                || new_pos.1 >= input.len() as i64
                || new_pos.0 < 0
                || new_pos.0 >= input[0].len() as i64
            {
                continue;
            }
            let value = input[(y + dy) as usize][(x + dx) as usize] as u32 - '0' as u32;
            let heat_loss = current.heat_loss + value as i64;
            let mut repetitions = current.repetitions;
            //dbg!(&(x, y));
            //dbg!(&(dx, dy));
            //dbg!(&repetitions.repetitions);
            if repetitions.dir == (dx, dy) || repetitions.dir == (0, 0) {
                repetitions.repetitions += 1;
                if repetitions.repetitions + 1 > 10 {
                    //println!("too much rep");
                    continue;
                }
                //println!("Same dir ok");
            } else {
                if repetitions.repetitions + 1 < 4 {
                    //println!("too few rep");
                    continue;
                }
                //println!("Turning");
                repetitions.repetitions = 0;
            }
            repetitions.dir = (dx, dy);
            //if last_three.dir ==
            //if last_three.len() == 10 && last_three.iter().all(|&last| last == (dx, dy)) {
            //continue;
            //}
            //last_three.push_back((dx, dy));
            //if last_three.len() > 10 {
            //last_three.pop_front();
            //}
            pq.push(
                Pos {
                    pos: new_pos,
                    repetitions,
                    heat_loss,
                    prev: key.clone(),
                },
                -heat_loss,
            );
            //if new_pos == goal {
            //break;
            //}
        }
        //1188
    }

    let last = reached
        .iter()
        .filter_map(|((pos, reps), val)| (*pos == goal && reps.repetitions + 1 >= 4).then_some(val))
        .min_by_key(|val| val.heat_loss);
    //let last = &reached[&goal];
    let part1 = last.map(|val| val.heat_loss);
    dbg!(&part1);

    //if let Some(mut cur) = last {
        //let mut prev: Option<&Pos> = None;
        //let mut path = input.clone();
        //loop {
            //if let Some(prev) = prev {
                //if prev.pos == cur.pos {
                    //break;
                //}
                //path[cur.pos.1 as usize][cur.pos.0 as usize] =
                    //match (prev.pos.0 - cur.pos.0, prev.pos.1 - cur.pos.1) {
                        //(1, 0) => '>',
                        //(-1, 0) => '<',
                        //(0, 1) => 'V',
                        //(0, -1) => '^',
                        //_ => unreachable!(),
                    //};
            //}

            //prev = Some(cur);
            //cur = &reached[&cur.prev];
        //}
        //plot(&path);
    //}
    //1048

    //let mut reached_field = input.clone();
    //for pos in reached.values() {
    //reached_field[pos.pos.1 as usize][pos.pos.0 as usize] = '#';
    //}
    //plot(&reached_field);

    Ok(())
}
