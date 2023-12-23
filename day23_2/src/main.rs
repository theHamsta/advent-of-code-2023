use bitvec_simd::BitVec;
use petgraph::prelude::*;
use petgraph::Graph;

use itertools::Itertools;

type Point2d = (i16, i16);

fn find_longest_path_bitvec(input: &[Vec<char>], start: Point2d, part2: bool) -> u64 {
    let mut pq = Vec::new();

    let width = input[0].len() as i16;
    let height = input.len() as i16;
    let bitvector = BitVec::zeros((width * height) as usize);
    pq.push((0, start.0, start.1, bitvector));

    let mut max_goal = 0u64;


    while let Some((steps, x, y, mut prev)) = pq.pop() {
        if x < 0 || x >= input[0].len() as i16 || y < 0 || y >= input.len() as i16 {
            continue;
        }
        let cur = input[y as usize][x as usize];
        if cur == '#' {
            continue;
        }
        if y == input.len() as i16 - 1 {
            //goal_paths.push(visited.clone());
            max_goal = max_goal.max(steps);
            dbg!(&max_goal);
        }
        if prev.get_unchecked((y * width + x) as usize) {
            continue;
        }

        prev.set((y * width + x) as usize, true);

        if !part2 {
            match cur {
                '>' => {
                    if input[y as usize][x as usize + 1] != '#' {
                        pq.push((steps + 1, x + 1, y, prev.clone()));
                        continue;
                    }
                }
                '<' => {
                    if input[y as usize][x as usize - 1] != '#' {
                        pq.push((steps + 1, x - 1, y, prev.clone()));
                        continue;
                    }
                }
                'v' => {
                    if input[y as usize + 1][x as usize] != '#' {
                        pq.push((steps + 1, x, y + 1, prev.clone()));
                        continue;
                    }
                }
                '^' => {
                    if input[y as usize - 1][x as usize] != '#' {
                        pq.push((steps - 1, x, y - 1, prev.clone()));
                        continue;
                    }
                }
                '.' => (),
                a => panic!("Didn't expect to see {a}"),
            }
        }
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            pq.push((steps + 1, x + dx, y + dy, prev.clone()));
        }
    }

    max_goal
}

fn find_longest_path_graph(input: &[Vec<char>], start: Point2d, part2: bool) -> u64 {
    //let mut pq = BinaryHeap::new();
    let mut pq = Vec::new();

    pq.push((0, start.0, start.1, None));

    //let mut visited: HashMap<(i16, i16), (u64, Option<(i64, i64)>)> = HashMap::new();

    //let mut goal_paths = Vec::new();
    let mut max_goal = 0u64;
    //let mut cur_max = HashMap::new();

    let mut graph = Graph::<Point2d, ()>::new();

    while let Some((steps, x, y, prev)) = pq.pop() {
        if x < 0 || x >= input[0].len() as i16 || y < 0 || y >= input.len() as i16 {
            continue;
        }
        let cur = input[y as usize][x as usize];
        if cur == '#' {
            continue;
        }
        if y == input.len() as i16 - 1 {
            //goal_paths.push(visited.clone());
            max_goal = max_goal.max(steps);
            dbg!(&max_goal);
            dbg!(&max_goal);
        }
        if was_there_be_before(&graph, prev, (x, y)) {
            continue;
        }

        //match cur_max.entry((x, y)) {
        //std::collections::hash_map::Entry::Occupied(mut o) => {
        //if *o.get() < steps {
        //*o.get_mut() = steps;
        //} else {
        //if *o.get() > steps + 100 {
        //continue;
        //}
        //}
        //}
        //std::collections::hash_map::Entry::Vacant(v) => {
        //v.insert(steps);
        //}
        //}

        let node = graph.add_node((x, y));
        if let Some(prev) = prev {
            graph.add_edge(prev, node, ());
        }

        if !part2 {
            match cur {
                '>' => {
                    if input[y as usize][x as usize + 1] != '#' {
                        pq.push((steps + 1, x + 1, y, Some(node)));
                        continue;
                    }
                }
                '<' => {
                    if input[y as usize][x as usize - 1] != '#' {
                        pq.push((steps + 1, x - 1, y, Some(node)));
                        continue;
                    }
                }
                'v' => {
                    if input[y as usize + 1][x as usize] != '#' {
                        pq.push((steps + 1, x, y + 1, Some(node)));
                        continue;
                    }
                }
                '^' => {
                    if input[y as usize - 1][x as usize] != '#' {
                        pq.push((steps - 1, x, y - 1, Some(node)));
                        continue;
                    }
                }
                '.' => (),
                a => panic!("Didn't expect to see {a}"),
            }
        }
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            pq.push((steps + 1, x + dx, y + dy, Some(node)));
        }
    }

    max_goal
}

fn was_there_be_before(
    visited: &Graph<Point2d, ()>,
    cur: Option<NodeIndex>,
    needle: Point2d,
) -> bool {
    let Some(mut cur) = cur else {
        return false;
    };
    while let Some((_, next)) = visited
        .neighbors_directed(cur, Incoming)
        .detach()
        .next(visited)
    {
        if visited.node_weight(next) == Some(&needle) {
            return true;
        }
        cur = next;
    }
    false
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    //let input = include_str!("../example1");
    //5450

    let input = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let start: Point2d = (input[0].iter().position(|&c| c == '.').unwrap() as i16, 0);
    dbg!(&start);

    let max = find_longest_path_bitvec(&input, start, true);
    dbg!(&max);
    //let max = find_longest_path_graph(&input, start, true);
    //dbg!(&max);
    //let part1 = paths.iter().find_map(|(pos, (dist, _))| {
    //if pos.1 == input.len() as i16 - 1 {
    //Some(dist)
    //} else {
    //None
    //}
    //});
    //dbg!(&part1);

    //let longest = paths.iter().map(|paths|
    //paths.iter()
    //.find(|(pos, (_dist, _))| pos.1 == input.len() as i16 - 1)
    //.unwrap()).max_by_key(|paths);

    //let mut cur = longest.1;
    //let mut copy = input.clone();
    //copy[longest.0 .1 as usize][longest.0 .0 as usize] = '♡';
    //while let (_, Some(prev)) = cur {
    //copy[prev.1 as usize][prev.0 as usize] = '♡';
    //cur = &paths[&prev];
    //}
    //plot(&input);
    //plot(&copy);

    //let mut copy = input.clone();
    //for y in 0..input.len() {
    //for x in 0..input[0].len() {
    //if paths.contains_key(&(x as i16, y as i16)) {
    //copy[y][x] = '_';
    //}
    //}
    //}
    //plot(&copy);

    //let part2 = part2.0;
    //dbg!(&part2);

    Ok(())
}

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
