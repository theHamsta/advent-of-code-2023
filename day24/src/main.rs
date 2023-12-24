use std::fmt::Display;

use glam::Vec3Swizzles;
use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq)]
struct HailGrain {
    p: glam::DVec3,
    v: glam::DVec3,
}

impl Display for HailGrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.02}, {:.02}, {:.02} @ {:.02}, {:.02}, {:.02}",
            self.p.x, self.p.y, self.p.z, self.v.x, self.v.y, self.v.z
        )
    }
}

impl HailGrain {
    //fn intersects_3d(&self, other: &Self) -> glam::DVec3 {
    //(other.v - self.v) / (self.p - other.p)
    //}
    fn intersects_2d(&self, other: &Self) -> Option<glam::DVec2> {
        let x1 = self.p;
        let x2 = self.p + self.v;
        let x3 = other.p;
        let x4 = other.p + other.v;
        let d = (x1.x - x2.x) * (x3.y - x4.y) - (x1.y - x2.y) * (x3.x - x4.x);
        let t = ((x1.x - x3.x) * (x3.y - x4.y) - (x1.y - x3.y) * (x3.x - x4.x)) / d;
        let u = ((x1.x - x3.x) * (x1.y - x2.y) - (x1.y - x3.y) * (x1.x - x2.x)) / d;
        (t >= 0.0 && u >= 0.0 && d.abs() > 1e-3).then_some(self.p.xy() + t * self.v.xy())
    }
}

fn num_intersections(points: &[HailGrain], x_range: (f64, f64), y_range: (f64, f64)) -> usize {
    points
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            let s = a.intersects_2d(b);
            //println!("{a} -> {b}");
            //dbg!(&s);
            s.map(|s| s.x >= x_range.0 && s.x <= x_range.1 && s.y >= y_range.0 && s.y <= y_range.1)
                .unwrap_or(false)
        })
        .count()
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input");
    //let input = include_str!("../example1");

    let regex = Regex::new(
        r"([-]?\d+),\s*([-]?\d+),\s*([-]?\d+)\s*@\s*([-]?\d+),\s*([-]?\d+),\s*([-]?\d+)",
    )
    .unwrap();

    let points = regex
        .captures_iter(input)
        .map(|c| HailGrain {
            p: glam::DVec3 {
                x: c[1].parse().unwrap(),
                y: c[2].parse().unwrap(),
                z: c[3].parse().unwrap(),
            },
            v: glam::DVec3 {
                x: c[4].parse().unwrap(),
                y: c[5].parse().unwrap(),
                z: c[6].parse().unwrap(),
            },
        })
        .collect_vec();

    //let start: Point2d = (input[0].iter().position(|&c| c == '.').unwrap() as i16, 0);

    //dbg!(&points);
    //let part1 = num_intersections(&points, (7., 27.), (7., 27.));
    let part1 = num_intersections(&points, (200000000000000., 400000000000000.), (200000000000000., 400000000000000.));
    dbg!(&part1);

    //let part2 = find_longest_path_bitvec(&input, start, true);
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
