use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn tilt(input: &mut Vec<Vec<char>>, direction: (i64, i64)) {
    let y_iter = (0..input.len()).map(|x| x as i64);
    let x_iter = (0..input[0].len()).map(|x| x as i64);

    let it: Box<dyn Iterator<Item = (i64, i64)>> = match direction {
        (1, 0) => Box::new(y_iter.cartesian_product(x_iter.rev())),
        (-1, 0) => Box::new(y_iter.cartesian_product(x_iter)),
        (0, 1) => Box::new(y_iter.rev().cartesian_product(x_iter)),
        (0, -1) => Box::new(y_iter.cartesian_product(x_iter)),
        _ => unreachable!(),
    };

    for (y, x) in it.into_iter() {
        //println!("{y} {x}");
        if input[y as usize][x as usize] == 'O' {
            let mut shifted = (x, y);
            let mut last = (x, y);
            loop {
                shifted = (shifted.0 + direction.0, shifted.1 + direction.1);
                let (sx, sy) = shifted;
                if (sx < 0 || sx >= input[0].len() as i64) || (sy < 0 || sy >= input.len() as i64) {
                    break;
                }
                if input[shifted.1 as usize][shifted.0 as usize] != '.' {
                    break;
                }

                input[shifted.1 as usize][shifted.0 as usize] = 'O';
                input[last.1 as usize][last.0 as usize] = '.';
                last = shifted;
            }
            //println!("Shifted from {x},{y} to {},{}", last.0, last.1);
        }
    }
}

fn plot(input: &Vec<Vec<char>>) {
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            print!("{}", input[y][x]);
        }
        println!();
    }
    println!();
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    let raw_input = include_str!("../example1");

    let input = raw_input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .map(|l| l.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec();
    //plot(&input);
    //println!();
    let mut shifted = input.clone();

    for shifted in shifted.iter_mut() {
        tilt(shifted, (0, -1));
    }
    //plot(&shifted);

    let mut sum = 0;
    for shifted in shifted.iter() {
        for y in 0..shifted.len() {
            for x in 0..shifted[0].len() {
                if shifted[y][x] == 'O' {
                    sum += shifted.len() - y;
                }
            }
        }
    }
    let part1 = sum;
    dbg!(&part1);

    let mut shifted = input.clone();

    let num_iterations = 3;
    //let num_iterations = 1000000000;
    for shifted in shifted.iter_mut() {
        plot(shifted);
        let before_it = shifted.clone();
        for i in 0..num_iterations {
            tilt(shifted, (0, -1));
            tilt(shifted, (-1, 0));
            tilt(shifted, (0, 1));
            tilt(shifted, (1, 0));
            plot(shifted);
        }
        if before_it == *shifted {
            break;
        }
    }
    //plot(&shifted);

    let mut sum = 0;
    for shifted in shifted.iter() {
        for y in 0..shifted.len() {
            for x in 0..shifted[0].len() {
                if shifted[y][x] == 'O' {
                    sum += shifted.len() - y;
                }
            }
        }
    }

    let part2 = sum;
    dbg!(&part2);
    Ok(())
}
