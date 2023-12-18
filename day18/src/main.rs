use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use itertools::{Itertools, MinMaxResult};

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

fn inside_area_naive(field: &HashSet<Point2d>) -> u64 {
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

#[derive(Debug)]
struct Quad {
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
}

impl Quad {
    fn can_go_right(&self, lines: &[(Point2d, Point2d)]) -> bool {
        !lines.iter().copied().any(|(start, end)| {
            let range = start.1.min(end.1)..=start.1.max(end.1);
            *self.x_range.end() == start.0
                && *self.x_range.end() == end.0
                && range.contains(self.y_range.start())
                && range.contains(self.y_range.end())
        })
    }

    fn can_go_left(&self, lines: &[(Point2d, Point2d)]) -> bool {
        !lines.iter().copied().any(|(start, end)| {
            let range = start.1.min(end.1)..=start.1.max(end.1);
            *self.x_range.start() == start.0
                && *self.x_range.start() == end.0
                && range.contains(self.y_range.start())
                && range.contains(self.y_range.end())
        })
    }

    fn can_go_up(&self, lines: &[(Point2d, Point2d)]) -> bool {
        !lines.iter().copied().any(|(start, end)| {
            //println!("({},{}) -> ({},{})", start.0, start.1, end.0, end.1);
            let range = start.0.min(end.0)..=start.0.max(end.0);
            *self.y_range.start() == start.1
                && *self.y_range.start() == end.1
                && range.contains(self.x_range.start())
                && range.contains(self.x_range.end())
        })
    }

    fn can_go_down(&self, lines: &[(Point2d, Point2d)]) -> bool {
        !lines.iter().copied().any(|(start, end)| {
            let range = start.0.min(end.0)..=start.0.max(end.0);
            *self.y_range.end() == start.1
                && *self.y_range.end() == end.1
                && range.contains(self.x_range.start())
                && range.contains(self.x_range.end())
        })
    }

    fn distinct_ranges(&self) -> Vec<(RangeInclusive<i64>, RangeInclusive<i64>)> {
        let Self { x_range, y_range } = self;
        vec![
            // corners
            (
                (*x_range.start())..=(*x_range.start()),
                (*y_range.start())..=(*y_range.start()),
            ),
            (
                (*x_range.end())..=(*x_range.end()),
                (*y_range.start())..=(*y_range.start()),
            ),
            (
                (*x_range.start())..=(*x_range.start()),
                (*y_range.end())..=(*y_range.end()),
            ),
            (
                (*x_range.end())..=(*x_range.end()),
                (*y_range.end())..=(*y_range.end()),
            ),
            // borders
            (
                (*x_range.start() + 1)..=(*x_range.end() - 1),
                (*y_range.start())..=(*y_range.start()),
            ),
            (
                (*x_range.start() + 1)..=(*x_range.end() - 1),
                (*y_range.end())..=(*y_range.end()),
            ),
            (
                (*x_range.start())..=(*x_range.start()),
                (*y_range.start() + 1)..=(*y_range.end() - 1),
            ),
            (
                (*x_range.end())..=(*x_range.end()),
                (*y_range.start() + 1)..=(*y_range.end() - 1),
            ),
            // middle
            (
                (*x_range.start() + 1)..=(*x_range.end() - 1),
                (*y_range.start() + 1)..=(*y_range.end() - 1),
            ),
        ]
    }
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
    let quads = y_coords
        .iter()
        .copied()
        .tuple_windows()
        .cartesian_product(x_coords.iter().copied().tuple_windows())
        .map(|((yl, yh), (xl, xh))| Quad {
            x_range: xl..=xh,
            y_range: yl..=yh,
        })
        .collect_vec();

    //for l in lines.iter() {
    //println!("({},{}) -> ({},{})", l.0 .0, l.0 .1, l.1 .0, l.1 .1);
    //}
    //dbg!(&x_coords);
    //dbg!(&y_coords);

    let width = x_coords.len() as i64 - 1;
    let height = y_coords.len() as i64 - 1;
    assert_eq!(quads.len() as i64, width * height);
    let mut visited = HashMap::new();

    // Take any tile that is guaranteed inside
    let start = lines[0].0;
    let start_tile = quads
        .iter()
        .position(|q| *q.x_range.start() == start.0 && *q.y_range.start() == start.1)
        .expect("did not prepare for puzzle input where line[0].start is x_max or y_max");

    let mut stack = vec![(
        (start_tile as i64 % width, start_tile as i64 / width),
        (0, 0),
    )];

    // Now let's traverse all remaining quads without intersecting a line (which would lead us into
    // outside)
    while let Some(((x, y), dir)) = stack.pop() {
        if x < 0 || x >= width || y < 0 || y >= height {
            continue;
        }
        if visited.contains_key(&(x, y)) {
            continue;
        };
        visited.insert((x, y), dir);
        let quad = &quads[(y * width + x) as usize];

        if quad.can_go_right(lines) {
            stack.push(((x + 1, y), (1, 0)));
        }
        if quad.can_go_left(lines) {
            stack.push(((x - 1, y), (-1, 0)));
        }
        if quad.can_go_down(lines) {
            stack.push(((x, y + 1), (0, 1)));
        }
        if quad.can_go_up(lines) {
            stack.push(((x, y - 1), (0, -1)));
        }

        //for y in 0..height {
        //for x in 0..width {
        //if visited.contains_key(&(x, y)) {
        //match visited[&(x, y)] {
        //(0, 0) => print!("#"),
        //(0, 1) => print!("v"),
        //(1, 0) => print!(">"),
        //(0, -1) => print!("^"),
        //(-1, 0) => print!("<"),
        //_ => unreachable!(),
        //}
        //} else {
        //print!(".");
        //}
        //}
        //println!();
        //}
        //println!();
    }

    let ret = visited
        .keys()
        .map(|(x, y)| &quads[(y * width + x) as usize])
        .flat_map(|q| q.distinct_ranges())
        .unique()
        .map(|(rx, ry)| {
            (rx.end() + 1 - rx.start()).clamp(0, i64::max_value())
                * (ry.end() + 1 - ry.start()).clamp(0, i64::max_value())
        })
        .sum();
    ret
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    //let input = include_str!("../example1");

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
        cur.dir = match t.color & 0xF {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => unreachable!(),
        };
        let start = cur.pos;
        cur.pos = (
            cur.pos.0 + cur.dir.0 * (t.color >> 4) as i64,
            cur.pos.1 + cur.dir.1 * (t.color >> 4) as i64,
        );
        let end = cur.pos;
        lines.push((start, end));
    }
    //   952408144115
    //303815444000044
    let part1 = inside_area_naive(&field);
    dbg!(&part1);

    let part2 = inside_area_lines(&lines);
    dbg!(&part2);

    // This is when you don't read the instructions
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
            cur.pos.0 + cur.dir.0 * (t.color) as i64,
            cur.pos.1 + cur.dir.1 * (t.color) as i64,
        );
        let end = cur.pos;
        lines.push((start, end));
    }
    let part3 = inside_area_lines(&lines);
    dbg!(&part3);

    Ok(())
}
