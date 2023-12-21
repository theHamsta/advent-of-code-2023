use std::collections::HashMap;

use itertools::Itertools;

type Point2d = (i16, i16);

fn count_steps(
    input: &Vec<Vec<char>>,
    (x, y): Point2d,
    steps: i64,
    cache: &mut HashMap<(Point2d, i64), u64>,
) -> u64 {
    if x < 0
        || x >= input[0].len() as i16
        || y < 0
        || y >= input.len() as i16
        || input[y as usize][x as usize] == '#'
    {
        return 0;
    }
    if steps == 0 {
        dbg!(&(x, y));
        return 1;
    }
    if let Some(res) = cache.get(&((x, y), steps)) {
        return *res;
    }
    let mut sum = 0u64;
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        sum += count_steps(input, (x + dx, y + dy), steps - 1, cache);
    }
    cache.insert(((x, y), steps), sum);
    sum
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    let input = include_str!("../example1");
    //

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
    let part1 = count_steps(&input, start, 2, &mut cache);
    dbg!(&part1);

    Ok(())
}
