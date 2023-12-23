use bitvec_simd::BitVec;

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
            let old = max_goal;
            max_goal = max_goal.max(steps);
            if old != max_goal {
                dbg!(&max_goal);
            }
        }

        if !part2 {
            if prev.get_unchecked((y * width + x) as usize) {
                continue;
            }
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

        prev.set((y * width + x) as usize, true);

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0
                && nx < (input[0].len() as i16)
                && ny >= 0
                && ny < (input.len() as i16)
                && input[ny as usize][nx as usize] != '#'
                && !prev.get_unchecked((ny * width + nx) as usize)
            {
                pq.push((steps + 1, nx, ny, prev.clone()));
            }
        }
    }

    max_goal
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    //let input = include_str!("../example1");

    let input = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let start: Point2d = (input[0].iter().position(|&c| c == '.').unwrap() as i16, 0);

    let part1 = find_longest_path_bitvec(&input, start, false);
    dbg!(&part1);

    // took more than 1h26min to find the solution but wasn't done maximizing (but didn't add the
    // nx/ny optimization). Next idea would have been to take a maximum subset of the junctions to
    // form a round tour (when connecting end to start)
    // Current version finds maximum in 11min though not finished after 27min, should have really
    // used the condensed graph
    let part2 = find_longest_path_bitvec(&input, start, true);
    dbg!(&part2);

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
