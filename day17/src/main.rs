use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;

#[allow(dead_code)]
fn plot(input: &[Vec<char>]) {
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            print!("{}", input[y][x]);
        }
        println!();
    }
    println!();
}

type PointInt = i16;
type Point2d = (PointInt, PointInt);

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Repetitions {
    repetitions: u32,
    dir: Point2d,
}

type Key = (Point2d, Repetitions);
//type Key = ((i64, i64), VecDeque<(i64, i64)>);
//type Key = (i64, i64);

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    heat_loss: i64,
    pos: Point2d,
    repetitions: Repetitions,
    prev: Key,
}

fn find_solution(
    input: &[Vec<char>],
    min_repetions: u32,
    max_repetitions: u32,
    plot_result: bool,
) -> Option<i64> {
    let mut pq = BinaryHeap::new();

    pq.push(Reverse(Pos {
        pos: (0, 0),
        repetitions: Default::default(),
        heat_loss: 0,
        prev: Key::default(),
    }));

    let mut reached = HashMap::<Key, Pos>::new();
    let goal = (input[0].len() as PointInt - 1, input.len() as PointInt - 1);

    while let Some(Reverse(current)) = pq.pop() {
        let (x, y) = current.pos;

        let key = (current.pos, current.repetitions);

        match reached.entry(key) {
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
        if current.pos == goal && current.repetitions.repetitions + 1 >= min_repetions {
            break;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if (-dx, -dy) == current.repetitions.dir {
                continue;
            }

            let new_pos = (x + dx, y + dy);
            if new_pos.1 < 0
                || new_pos.1 >= input.len() as PointInt
                || new_pos.0 < 0
                || new_pos.0 >= input[0].len() as PointInt
            {
                continue;
            }
            let value = input[(y + dy) as usize][(x + dx) as usize] as u32 - '0' as u32;
            let heat_loss = current.heat_loss + value as i64;
            let mut repetitions = current.repetitions;
            if repetitions.dir == (dx, dy) || repetitions.dir == (0, 0) {
                repetitions.repetitions += 1;
                if repetitions.repetitions + 1 > max_repetitions {
                    continue;
                }
            } else {
                if repetitions.repetitions + 1 < min_repetions {
                    continue;
                }
                repetitions.repetitions = 0;
            }
            repetitions.dir = (dx, dy);
            pq.push(Reverse(Pos {
                pos: new_pos,
                repetitions,
                heat_loss,
                prev: key,
            }));
        }
    }

    let last = reached
        .iter()
        .filter_map(|((pos, reps), val)| {
            (*pos == goal && reps.repetitions + 1 >= min_repetions).then_some(val)
        })
        .min_by_key(|val| val.heat_loss);
    if plot_result {
        if let Some(mut cur) = last {
            let mut prev: Option<&Pos> = None;
            let mut path = input.to_vec();
            loop {
                if let Some(prev) = prev {
                    if prev.pos == cur.pos {
                        break;
                    }
                    path[cur.pos.1 as usize][cur.pos.0 as usize] =
                        match (prev.pos.0 - cur.pos.0, prev.pos.1 - cur.pos.1) {
                            (1, 0) => '>',
                            (-1, 0) => '<',
                            (0, 1) => 'V',
                            (0, -1) => '^',
                            _ => unreachable!(),
                        };
                }

                prev = Some(cur);
                cur = &reached[&cur.prev];
            }
            plot(&path);
        }

        //let mut reached_field = input.to_vec();
        //for pos in reached.values() {
        //reached_field[pos.pos.1 as usize][pos.pos.0 as usize] = '#';
        //}
        //plot(&reached_field);
    }
    last.map(|val| val.heat_loss)
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    //let input = include_str!("../example1");
    //let input = include_str!("../example2");

    let input = input.lines().map(|s| s.chars().collect_vec()).collect_vec();

    let plot = false;
    let part1 = find_solution(&input, 0, 3, plot);
    dbg!(&part1);
    let part2 = find_solution(&input, 4, 10, plot);
    dbg!(&part2);

    Ok(())
}
