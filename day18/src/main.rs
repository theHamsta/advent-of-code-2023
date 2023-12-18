use std::collections::{HashMap, HashSet};

use geo::{Area, Intersects, Line};
use geo_types::{LineString, Polygon};
use itertools::{Itertools, MinMaxResult};
use line_intersection::LineInterval;
use rayon::iter::IntoParallelIterator;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Trench {
    dir: char,
    distance: u64,
    color: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
struct State {
    pos: Point2d,
    dir: Point2d,
    last_char: char,
}

type Point2d = (i64, i64);

fn inside_area(field: &HashSet<Point2d>) -> u64 {
    let MinMaxResult::MinMax(min_x, max_x): itertools::MinMaxResult<i64> =
        field.iter().copied().map(|(x, _)| x).minmax()
    else {
        unreachable!()
    };
    let MinMaxResult::MinMax(min_y, max_y) = field.iter().copied().map(|(_, y)| y).minmax() else {
        unreachable!()
    };

    let start: Point2d = (min_x - 1, min_y - 1);
    let mut visited = HashSet::new();

    let mut stack = vec![start];

    while let Some((x, y)) = stack.pop() {
        if x < min_x - 1 || x > (max_x + 1) || y < min_y - 1 || y > (max_y + 1) {
            continue;
        }
        if field.contains(&(x, y)) || visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        stack.push((x + 1, y));
        stack.push((x - 1, y));
        stack.push((x, y + 1));
        stack.push((x, y - 1));
    }
    let outside = visited.len() as i64;

    let total = (max_x - min_x + 3) * (max_y - min_y + 3);
    let calculations = total - outside;

    let mut sum = 0u64;
    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            if visited.contains(&(x, y)) {
                //print!("#");
            } else {
                //print!(".");
                sum += 1;
            }
        }
        //println!();
    }
    assert_eq!(calculations, sum as i64);
    sum
}

fn inside_area_lines(lines: &[(Point2d, Point2d)]) -> i64 {
    let x_coords = lines
        .iter()
        .copied()
        .map(|((x1, _), _)| x1)
        .unique()
        .sorted()
        .collect_vec();
    let y_coords = lines
        .iter()
        .copied()
        .map(|((_, y1), _)| y1)
        .unique()
        .sorted()
        .collect_vec();
    dbg!(&x_coords);

    let start: Point2d = (-1, -1);
    let mut visited = HashMap::new();

    let mut stack = vec![(start, start)];

    while let Some(((x, y), (last_x, last_y))) = stack.pop() {
        if x < -1 || x > x_coords.len() as i64 || y < -1 || y > y_coords.len() as i64 {
            continue;
        }
        if visited.contains_key(&(x, y)) {
            continue;
        }
        if lines.iter().any(|(start, end)| {
            let line1 = geo::geometry::Line {
                start: (*start).into(),
                end: (*end).into(),
            };
            let line2 = geo::geometry::Line {
                start: (
                    if last_x == -1 {
                        x_coords[0] - 1
                    } else if last_x == x_coords.len() as i64 {
                        *x_coords.last().unwrap() + 1
                    } else {
                        x_coords[last_x as usize] + 1
                    },
                    if last_y == -1 {
                        y_coords[0] - 1
                    } else if last_y == y_coords.len() as i64 {
                        *y_coords.last().unwrap() + 1
                    } else {
                        y_coords[last_y as usize] + 1
                    },
                )
                    .into(),
                end: (
                    if x == -1 {
                        x_coords[0] - 1
                    } else if x == x_coords.len() as i64 {
                        *x_coords.last().unwrap() + 1
                    } else {
                        x_coords[x as usize] + 1
                    },
                    if y == -1 {
                        y_coords[0] - 1
                    } else if y == y_coords.len() as i64 {
                        *y_coords.last().unwrap() + 1
                    } else {
                        y_coords[y as usize] + 1
                    },
                )
                    .into(),
            };
            line1.intersects(&line2)
        }) {
            continue;
        }
        let area =
            if x >= 0 && x + 1 < x_coords.len() as i64 && y >= 0 && y + 1 < y_coords.len() as i64 {
                let dx = x_coords[x as usize + 1] - x_coords[x as usize];
                let dy = y_coords[y as usize + 1] - y_coords[y as usize];
                dx * dy
            } else {
                0
            };
        visited.insert((x, y), area);
        stack.push(((x + 1, y), (x, y)));
        stack.push(((x - 1, y), (x, y)));
        stack.push(((x, y + 1), (x, y)));
        stack.push(((x, y - 1), (x, y)));
        //for y in (-1)..=(y_coords.len() as i64) {
        //for x in (-1)..=(x_coords.len() as i64) {
        //if visited.contains_key(&(x, y)) {
        //print!(".");
        //} else {
        //print!("#");
        //}
        //}
        //println!();
        //}
        //println!();
    }

    for y in (-1)..=(y_coords.len() as i64) {
        for x in (-1)..=(x_coords.len() as i64) {
            if visited.contains_key(&(x, y)) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }

    let outside_area: i64 = visited.values().copied().sum();
    let total =
        (*x_coords.last().unwrap() - x_coords[0]) * (*y_coords.last().unwrap() - y_coords[0]);

    (total - outside_area) / 4
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    let input = include_str!("../example1");

    let regex = regex::Regex::new(r"(\w) (\d+) [(]#(\w+)[)]").unwrap();
    let input = input
        .lines()
        .map(|s| {
            let cap = regex.captures(s).unwrap();
            Trench {
                dir: cap[1].chars().next().unwrap(),
                distance: cap[2].parse().unwrap(),
                color: u32::from_str_radix(&cap[3], 16).unwrap(),
            }
        })
        .collect_vec();

    let mut field = HashSet::new();

    let mut cur = State::default();
    for t in input.iter() {
        cur.dir = match t.dir {
            'R' => (1, 0),
            'D' => (0, 1),
            'L' => (-1, 0),
            'U' => (0, -1),
            _ => unreachable!(),
        };
        for _ in 0..t.distance {
            cur.pos = (cur.pos.0 + cur.dir.0, cur.pos.1 + cur.dir.1);
            field.insert(cur.pos);
        }
    }

    let mut lines = Vec::new();
    let mut cur = State::default();
    for t in input.iter() {
        cur.dir = match t.dir {
            'R' => (1, 0),
            'D' => (0, 1),
            'L' => (-1, 0),
            'U' => (0, -1),
            _ => unreachable!(),
        };
        let start = cur.pos;
        cur.pos = (
            cur.pos.0 + cur.dir.0 * (t.distance + 1) as i64,
            cur.pos.1 + cur.dir.1 * (t.distance + 1) as i64,
        );
        let end = cur.pos;
        lines.push((start, end));
    }

    let part1 = inside_area(&field);
    dbg!(&part1);

    let part2 = inside_area_lines(&lines);
    dbg!(&part2);

    //let mut inside_field = HashMap::new();

    //let mut area = 0u64;
    //for y in min_y..=(max_y + 1) {
    //let mut inside = false;
    //cur.last_char = ' ';
    //for x in min_x..=(max_x + 1) {
    //if field.contains_key(&(x, y)) {
    //*inside_field.entry((x, y)).or_insert(0) += 1;
    //cur.last_char = '#';
    //print!("#");
    //} else {
    //if cur.last_char == '#' {
    //inside = !inside;
    //}
    //if inside {
    //area += 1;
    //*inside_field.entry((x, y)).or_insert(0) += 1;
    //print!(".");
    //} else {
    //print!(" ");
    //}
    //cur.last_char = ' ';
    //}
    //}
    //println!();
    //}
    //for x in min_x..=(max_x + 1) {
    //let mut inside = false;
    //cur.last_char = ' ';
    //for y in min_y..=(max_y + 1) {
    //if field.contains_key(&(x, y)) {
    //*inside_field.entry((x, y)).or_insert(0) += 1;
    //cur.last_char = '#';
    //print!("#");
    //} else {
    //if cur.last_char == '#' {
    //inside = !inside;
    //}
    //if inside {
    //area += 1;
    //*inside_field.entry((x, y)).or_insert(0) += 1;
    //print!(".");
    //} else {
    //print!(" ");
    //}
    //cur.last_char = ' ';
    //}
    //}
    //println!();
    //}
    //let part1 = inside_field.values().filter(|&&v| v == 2).count();
    //dbg!(&part1);
    //49246
    //48326

    Ok(())
}
