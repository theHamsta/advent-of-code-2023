use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn is_allowed(delta: (i64, i64), cur: char) -> bool {
    match (delta, cur) {
        ((0, -1), '|') => true,
        ((0, -1), '7') => true,
        ((0, -1), 'F') => true,
        ((0, 1), '|') => true,
        ((0, 1), 'L') => true,
        ((0, 1), 'J') => true,
        ((1, 0), '-') => true,
        ((1, 0), '7') => true,
        ((1, 0), 'J') => true,
        ((-1, 0), '-') => true,
        ((-1, 0), 'L') => true,
        ((-1, 0), 'F') => true,
        _ => false,
    }
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");
    //let raw_input = include_str!("../example2");
    //let raw_input = include_str!("../example3");

    let input = raw_input
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect::<Vec<Vec<char>>>();

    let starting_position = input
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter()
                .enumerate()
                .find_map(|(x, c)| (*c == 'S').then_some((x as i64, y as i64)))
        })
        .unwrap();

    let mut stack = VecDeque::new();
    stack.push_front((starting_position, 0u64, None::<(i64, i64)>));
    let mut max_dist = (starting_position, 0u64);
    let mut visited: HashMap<(i64, i64), (u64, Vec<Option<(i64, i64)>>)> = HashMap::new();

    while let Some(((x, y), dist, prev)) = stack.pop_front() {
        if !(y >= 0 && y < input.len() as i64 && x >= 0 && x < input[0].len() as i64) {
            continue;
        }
        let cur = input[y as usize][x as usize];
        if let Some(prev) = prev {
            let delta = (x - prev.0, y - prev.1);
            if !is_allowed(delta, cur) {
                dbg!(&prev);
                dbg!(&cur);
                continue;
            }
        }

        let was_visited = visited.get_mut(&(x, y));
        if let Some(visited) = was_visited {
            if visited.0 > dist {
                panic!("Expected to always go the shortest route via breadth-first");
            }
            visited.1.push(prev);

            continue;
        }

        let next_dist = dist + 1;

        match cur {
            'S' => {
                stack.push_back(((x - 1, y), next_dist, Some((x, y))));
                stack.push_back(((x + 1, y), next_dist, Some((x, y))));
                stack.push_back(((x, y - 1), next_dist, Some((x, y))));
                stack.push_back(((x, y + 1), next_dist, Some((x, y))));
            }
            //| is a vertical pipe connecting north and south.
            '|' => {
                stack.push_back(((x, y + 1), next_dist, Some((x, y))));
                stack.push_back(((x, y - 1), next_dist, Some((x, y))));
            }
            //- is a horizontal pipe connecting east and west.
            '-' => {
                stack.push_back(((x + 1, y), next_dist, Some((x, y))));
                stack.push_back(((x - 1, y), next_dist, Some((x, y))));
            }
            //L is a 90-degree bend connecting north and east.
            'L' => {
                stack.push_back(((x, y - 1), next_dist, Some((x, y))));
                stack.push_back(((x + 1, y), next_dist, Some((x, y))));
            }
            //J is a 90-degree bend connecting north and west.
            'J' => {
                stack.push_back(((x, y - 1), next_dist, Some((x, y))));
                stack.push_back(((x - 1, y), next_dist, Some((x, y))));
            }
            //7 is a 90-degree bend connecting south and west.
            '7' => {
                stack.push_back(((x - 1, y), next_dist, Some((x, y))));
                stack.push_back(((x, y + 1), next_dist, Some((x, y))));
            }
            //F is a 90-degree bend connecting south and east.
            'F' => {
                stack.push_back(((x + 1, y), next_dist, Some((x, y))));
                stack.push_back(((x, y + 1), next_dist, Some((x, y))));
            }
            //. is ground; there is no pipe in this tile.
            _ => continue,
        }
        visited.insert((x, y), (dist, vec![prev]));
        if dist > max_dist.1 {
            max_dist = ((x, y), dist);
        }
    }
    let part1 = visited.values().map(|v| v.0).max().unwrap();
    dbg!(&part1);
    assert_eq!(max_dist.1, part1);

    let mut copy = input.clone();
    let mut border_starters = Vec::new();
    for y in 0..copy.len() {
        for x in 0..copy[0].len() {
            if !visited.contains_key(&(x as i64, y as i64)) {
                copy[y][x] = '.';

                if x == 0 || x == copy[0].len() - 1 || y == 0 || y == copy.len() - 1 {
                    border_starters.push((x as i64, y as i64));
                }
            } else {
                copy[y][x] = 'V';
            }
        }
    }

    let mut outside = HashSet::new();

    let mut stack = border_starters;
    while let Some((x, y)) = stack.pop() {
        if !(y >= 0 && y < input.len() as i64 && x >= 0 && x < input[0].len() as i64) {
            continue;
        }

        let cur = copy[y as usize][x as usize];
        if outside.contains(&(x, y)) || visited.contains_key(&(x, y)) {
            continue;
        }

        match cur {
            '.' => {
                stack.push((x - 1, y));
                stack.push((x + 1, y));
                stack.push((x, y - 1));
                stack.push((x, y + 1));
            }
            _ => {
                continue;
            }
        }

        outside.insert((x, y));
    }
    // 498
    let total = copy[0].len() * copy.len();
    let inside = total - visited.len() - outside.len();
    let part2 = inside;

    for y in 0..copy.len() {
        for x in 0..copy[0].len() {
            if outside.contains(&(x as i64, y as i64)) {
                copy[y][x] = 'O';
            }
        }
    }

    dbg!(&part2);
    for y in 0..copy.len() {
        for x in 0..copy[0].len() {
            print!("{}", copy[y][x]);
        }
        println!();
    }
    println!();

    let mut count = 0;
    for y in 0..copy.len() {
        for x in 0..copy[0].len() {
            if copy[y][x] == '.' {
                count += 1;
            }
        }
    }
    for y in 0..copy.len() {
        for x in 0..copy[0].len() {
            if copy[y][x] == 'V' {
                count += 1;
                print!("{}", input[y][x]);
            } else {
                print!("{}", copy[y][x]);
            }
        }
        println!();
    }
    println!("{count}");

    Ok(())
}

//fn parts_fit(delta: (dx,dy), cur: char, prev_2: Option<char>) -> bool {
//match (delta, cur, prev) {
//((-1,0), ''

//}
//}
