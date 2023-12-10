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

type Point = (i64, i64);
type PointSet = HashSet<(i64, i64)>;
type PointVec = Vec<(i64, i64)>;

fn floodfill(
    copy: &[Vec<char>],
    input: &[Vec<char>],
    visited: &HashMap<Point, (u64, Vec<Option<Point>>)>,
    start_set: PointVec,
) -> PointSet {
    let mut outside = HashSet::new();

    let mut stack = start_set;
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
    outside
        .iter()
        .copied()
        .filter(|(x, y)| input[*y as usize][*x as usize] == '.')
        .collect()
}

fn backtrack(
    input: &[Vec<char>],
    visited: &HashMap<(i64, i64), (u64, Vec<Option<(i64, i64)>>)>,
    start: (i64, i64),
) -> (PointVec, PointVec) {
    let mut cur = start;
    let mut left_neightbors = Vec::new();
    let mut right_neightbors = Vec::new();

    while let Some(prev) = visited[&cur].1[0] {
        let (x, y) = cur;
        let cur_char = input[y as usize][x as usize];
        let delta = (x - prev.0, y - prev.1);
        let (dx, dy) = delta;
        let right = (x - dy, y + dx);
        let left = (x + dy, y - dx);

        left_neightbors.push(left);
        right_neightbors.push(right);
        match (dx, dy, cur_char) {
            (-1, 0, 'L') => left_neightbors.push((x + dx, y + dy)),
            (-1, 0, 'F') => right_neightbors.push((x + dx, y + dy)),
            (1, 0, '7') => left_neightbors.push((x + dx, y + dy)),
            (1, 0, 'J') => right_neightbors.push((x + dx, y + dy)),
            (0, -1, 'F') => left_neightbors.push((x + dx, y + dy)),
            (0, -1, '7') => right_neightbors.push((x + dx, y + dy)),
            (0, 1, 'J') => left_neightbors.push((x + dx, y + dy)),
            (0, 1, 'L') => right_neightbors.push((x + dx, y + dy)),
            _ => (),
        }

        cur = prev;
    }
    (left_neightbors, right_neightbors)
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");
    //let raw_input = include_str!("../example2");
    //let raw_input = include_str!("../example3");
    //let raw_input = include_str!("../example4");

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

    // well, not sure if they really left, right or vice versa
    let mut right_neightbors = Vec::new();
    let mut left_neightbors = Vec::new();

    while let Some(((x, y), dist, prev)) = stack.pop_front() {
        if !(y >= 0 && y < input.len() as i64 && x >= 0 && x < input[0].len() as i64) {
            continue;
        }
        let cur = input[y as usize][x as usize];
        if let Some(prev) = prev {
            let delta = (x - prev.0, y - prev.1);
            if !is_allowed(delta, cur) {
                continue;
            }

            // right_neightbors would be more correct but I'm gonna assume that there at least one
            // - | at inner tiles
            if cur == '-' || cur == '|' {
                let (dx, dy) = delta;
                let left = (x - dy, y + dx);
                left_neightbors.push(left);
                let right = (x + dy, y - dx);
                right_neightbors.push(right);
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
            _ => {
                continue;
            }
        }
        visited.insert((x, y), (dist, vec![prev]));
        if dist > max_dist.1 {
            max_dist = ((x, y), dist);
        }
    }
    let part1 = visited.values().map(|v| v.0).max().unwrap();
    dbg!(&part1);
    assert_eq!(max_dist.1, part1);
    let loop_ends = &visited[&max_dist.0];
    assert_eq!(loop_ends.1.len(), 2);
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

    // change copy/copy to copy/input to just count '.' enclosed by loops
    let (a, b) = backtrack(&input, &visited, visited[&max_dist.0].1[0].unwrap());
    let (b2, a2) = backtrack(&input, &visited, visited[&max_dist.0].1[1].unwrap());

    let outside = floodfill(&copy, &copy, &visited, border_starters.clone());
    let mut left = floodfill(&copy, &copy, &visited, a.clone());
    let mut right = floodfill(&copy, &copy, &visited, b.clone());
    left.extend(floodfill(&copy, &copy, &visited, a2.clone()));
    right.extend(floodfill(&copy, &copy, &visited, b2.clone()));

    for y in 0..copy.len() {
        for x in 0..copy[0].len() {
            if left.contains(&(x as i64, y as i64)) && right.contains(&(x as i64, y as i64)) {
                print!("X");
            } else if left.contains(&(x as i64, y as i64)) {
                print!("A");
            } else if right.contains(&(x as i64, y as i64)) {
                print!("B");
            } else if visited.contains_key(&(x as i64, y as i64)) {
                print!("V");
            } else {
                print!("{}", input[y][x]);
            }
        }
        println!();
    }
    dbg!(&left.len());
    dbg!(&right.len());

    let part2 = if outside.intersection(&left).next().is_some() {
        assert!(outside.intersection(&right).next().is_none());
        right.len()
    } else {
        assert!(outside.intersection(&left).next().is_none());
        right.len()
    };
    dbg!(&part2);

    Ok(())
}
