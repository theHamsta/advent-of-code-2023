use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type PointInt = i64;
type Point2d = (PointInt, PointInt);

fn count_steps(
    input: &Vec<Vec<char>>,
    (x, y): Point2d,
    steps: i64,
    cache: &mut HashMap<(Point2d, i64), HashSet<Point2d>>,
    infinite: bool,
) -> HashSet<Point2d> {
    let y_lookup = y.rem_euclid(input.len() as i64);
    let x_lookup = x.rem_euclid(input[0].len() as i64);

    if infinite {
        if input[y_lookup as usize][x_lookup as usize] == '#' {
            return HashSet::new();
        }
    } else if x < 0
        || x >= input[0].len() as i64
        || y < 0
        || y >= input.len() as i64
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
        let lx = x - x_lookup;
        let ly = y - y_lookup;
        return res.iter().map(|(sx, sy)| (sx + lx, sy + ly)).collect();
        //return res.clone();
    }
    let mut points = HashSet::new();
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        points.extend(count_steps(
            input,
            (x + dx, y + dy),
            steps - 1,
            cache,
            infinite,
        ));
    }
    let lx = x - x_lookup;
    let ly = y - y_lookup;
    cache.insert(
        ((x, y), steps),
        points.iter().map(|(sx, sy)| (sx - lx, sy - ly)).collect(),
    );
    points
}



fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    let input = include_str!("../example1");

    let input = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let mut start = None;
    'outer: for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 'S' {
                start = Some((x as i64, y as i64));
                break 'outer;
            }
        }
    }
    let start = start.unwrap();

    //let mut cache = HashMap::new();
    //let points = count_steps(&input, start, 64, &mut cache, false);
    //let part1 = points.len();
    //dbg!(&part1);

    let mut cache = HashMap::new();
    let points = count_steps(&input, start, 500, &mut cache, true);
    let part2 = points.len();
    dbg!(&part2);

    Ok(())
}
