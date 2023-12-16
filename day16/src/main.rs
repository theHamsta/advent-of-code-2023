use std::collections::{HashMap, HashSet};

use itertools::Itertools;
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
struct RayState {
    pos: (i64, i64),
    dir: (i64, i64),
}

fn trace_ray(
    input: &Vec<Vec<char>>,
    state: RayState,
    energized: &mut HashSet<RayState>, /*, cache: &HashMap<_>*/
) -> u64 {
    let (x, y) = state.pos;
    if x < 0 || x >= input[0].len() as i64 || y < 0 || y >= input.len() as i64 {
        return 0;
    }
    if !energized.insert(state) {
        return 0;
    }
    match (input[y as usize][x as usize], state.dir) {
        ('|', (1, 0) | (-1, 0)) => {
            trace_ray(
                input,
                RayState {
                    pos: (state.pos.0, state.pos.1 + 1),
                    dir: (0, 1),
                },
                energized,
            ) + trace_ray(
                input,
                RayState {
                    pos: (state.pos.0, state.pos.1 - 1),
                    dir: (0, -1),
                },
                energized,
            ) + 1
        }
        ('-', (0, 1) | (0, -1)) => {
            trace_ray(
                input,
                RayState {
                    pos: (state.pos.0 + 1, state.pos.1),
                    dir: (1, 0),
                },
                energized,
            ) + trace_ray(
                input,
                RayState {
                    pos: (state.pos.0 - 1, state.pos.1),
                    dir: (-1, 0),
                },
                energized,
            ) + 1
        }
        ('\\', dir) => {
            let (nx, ny) = match dir {
                (1, 0) => (0, 1),
                (-1, 0) => (0, -1),
                (0, 1) => (1, 0),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            };
            trace_ray(
                input,
                RayState {
                    pos: (state.pos.0 + nx, state.pos.1 + ny),
                    dir: (nx, ny),
                },
                energized,
            ) + 1
        }
        ('/', dir) => {
            let (nx, ny) = match dir {
                (1, 0) => (0, -1),
                (-1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (0, -1) => (1, 0),
                _ => unreachable!(),
            };
            trace_ray(
                input,
                RayState {
                    pos: (state.pos.0 + nx, state.pos.1 + ny),
                    dir: (nx, ny),
                },
                energized,
            ) + 1
        }
        _ => trace_ray(
            input,
            RayState {
                pos: (x + state.dir.0, y + state.dir.1),
                dir: state.dir,
            },
            energized,
        ),
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    //let input = include_str!("../example1");

    let input = input.lines().map(|s| s.chars().collect_vec()).collect_vec();

    plot(&input);

    let mut energized = HashSet::new();
    let _ = trace_ray(
        &input,
        RayState {
            pos: (0, 0),
            dir: (1, 0),
        },
        &mut energized,
    );

    //let mut energized_map = input.clone();

    let part1 = energized
        .iter()
        .map(|state| state.pos)
        .unique()
        //.inspect(|pos| energized_map[pos.1 as usize][pos.0 as usize] = '#')
        .count();
    dbg!(&part1);

    let part2 =
    // all possible states
    //(0..(input[0].len() as i64)).into_par_iter()
    //.cartesian_product(0..(input.len() as i64))
    //.cartesian_product([(1, 0), (-1, 0), (0, 1), (0, -1)].iter())
    (0..input[0].len() as i64).map(|x| RayState {
        pos: (x, 0),
        dir: (0, 1)})
        .chain(
            (0..input[0].len() as i64).map(|x| RayState {
                pos: (x, input.len() as i64-1),
                dir: (0, -1)
            }))
        .chain(
            (0..input.len() as i64).map(|y| RayState {
                pos: (0, y),
                dir: (1, 0)
            }))
        .chain(
            (0..input.len() as i64).map(|y| RayState {
                pos: (input[0].len() as i64-1, y),
                dir: (-1, 0)
            }))
        .map(|state| {
            let mut energized = HashSet::new();
            let _ = trace_ray(&input, state, &mut energized);

            let part1 = energized
                .iter()
                .map(|state| state.pos)
                .unique()
                .count();
            part1
        }).max();
    dbg!(&part2);

    //plot(&energized_map);

    Ok(())
}
