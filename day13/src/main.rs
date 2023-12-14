use std::collections::HashSet;

use itertools::Itertools;

fn calc_reflection_sum(input: &[Vec<char>]) -> (u64, Vec<u64>, Vec<u64>) {
    let mut sum = 0u64;
    let mut x_reflections = Vec::new();
    let mut y_reflections = Vec::new();

    for y in 0..(input.len() as i64) {
        let mut is_symetric = true;
        let mut has_one_comparision = false;
        'outer: for y_ in 1..(input.len() as i64) {
            let low_y = y - y_ + 1;
            let high_y = y + y_;
            if low_y < 0 || high_y >= (input.len() as i64) {
                continue;
            }
            for x in 0..(input[0].len() as i64) {
                has_one_comparision = true;
                if input[low_y as usize][x as usize] != input[high_y as usize][x as usize] {
                    is_symetric = false;
                    break 'outer;
                }
            }
        }
        if is_symetric && has_one_comparision {
            sum += (y as u64 + 1) * 100;
            y_reflections.push(y as u64 + 1);
        }
    }

    for x in 0..(input[0].len() as i64) {
        let mut is_symetric = true;
        let mut has_one_comparision = false;
        'outer: for x_ in 1..(input.len() as i64) {
            let low_x = x - x_ + 1;
            let high_x = x + x_;
            if low_x < 0 || high_x >= (input[0].len() as i64) {
                continue;
            }
            for y in 0..(input.len() as i64) {
                has_one_comparision = true;
                if input[y as usize][low_x as usize] != input[y as usize][high_x as usize] {
                    is_symetric = false;
                    break 'outer;
                }
            }
        }
        if is_symetric && has_one_comparision {
            sum += x as u64 + 1;

            x_reflections.push(x as u64 + 1);
        }
    }
    (sum, x_reflections, y_reflections)
}

fn main() -> anyhow::Result<()> {
    let raw_input = include_str!("../input");
    //let raw_input = include_str!("../example1");

    let input = raw_input
        .split("\n\n")
        .map(|l| {
            l.lines()
                .map(|l| l.chars().collect_vec())
                .collect::<Vec<Vec<char>>>()
        })
        .collect_vec();

    let part1: u64 = input.iter().map(|i| calc_reflection_sum(i).0).sum();
    dbg!(&part1);

    let part2: u64 = input
        .clone()
        .iter_mut()
        .map(|i| {
            let (_, orig_x, orig_y) = calc_reflection_sum(i);

            for y in 0..i.len() {
                for x in 0..i[0].len() {
                    let orig = i[y][x];
                    i[y][x] = match i[y][x] {
                        '#' => continue,
                        '.' => '#',
                        _ => unreachable!(),
                    };
                    let (_, new_x, new_y) = calc_reflection_sum(i);
                    //dbg!(&new_x);
                    //dbg!(&new_y);
                    let is_smudge = (!new_x.is_empty() || !new_y.is_empty())
                        && (new_x != orig_x || new_y != orig_y);

                    if is_smudge {
                        let old_x: HashSet<_> = orig_x.iter().collect();
                        let old_y: HashSet<_> = orig_y.iter().collect();
                        let mut sum: u64 = new_x.iter().filter(|x| !old_x.contains(x)).sum();
                        sum += new_y.iter().filter(|y| !old_y.contains(y)).sum::<u64>() * 100;
                        return sum;
                    }

                    i[y][x] = orig;
                }
            }

            println!();
            panic!("No smudge found")
        })
        .sum();
    dbg!(&part2);
    //61669

    Ok(())
}
