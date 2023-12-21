use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Point2d = (i16, i16);

fn count_steps(
    input: &Vec<Vec<char>>,
    (x, y): Point2d,
    steps: i64,
    cache: &mut HashMap<(Point2d, i64), HashSet<Point2d>>,
) -> HashSet<Point2d> {
    if x < 0
        || x >= input[0].len() as i16
        || y < 0
        || y >= input.len() as i16
        || input[y as usize][x as usize] == '#'
    {
        return HashSet::new();
    }
    if steps == 0 {
        let mut rtn = HashSet::new();
        rtn.insert((x, y));

        return rtn;
    }
    if let Some(res) = cache.get(&((x, y), steps)) {
        return res.clone();
    }
    let mut points = HashSet::new();
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        /*sum +=*/
        points.extend(count_steps(input, (x + dx, y + dy), steps - 1, cache));
    }
    cache.insert(((x, y), steps), points.clone());
    points
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    //let input = include_str!("../example1");

    let input = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let mut start = None;
    'outer: for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 'S' {
                start = Some((x as i16, y as i16));
                break 'outer;
            }
        }
    }
    let start = start.unwrap();

    let mut cache = HashMap::new();
    let points = count_steps(&input, start, 64, &mut cache);
    let part1 = points.len();
    dbg!(&part1);

    Ok(())
}
