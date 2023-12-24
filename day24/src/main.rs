use std::{fmt::Display, ops::RangeInclusive};

use glam::{dvec3, DMat3, DVec2, DVec3, Vec3Swizzles};
use indicatif::ProgressBar;
use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq)]
struct HailGrain {
    p: glam::DVec3,
    v: glam::DVec3,
}

#[derive(Clone, Debug, PartialEq)]
struct Aabb {
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
    z_range: RangeInclusive<i64>,
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
    fn intersects_xy(&self, other: &Self) -> Option<(f64, glam::DVec2)> {
        let x1 = self.p.xy();
        let x2 = self.p.xy() + self.v.xy();
        let x3 = other.p.xy();
        let x4 = other.p.xy() + other.v.xy();
        let d = (x1.x - x2.x) * (x3.y - x4.y) - (x1.y - x2.y) * (x3.x - x4.x);
        let t = ((x1.x - x3.x) * (x3.y - x4.y) - (x1.y - x3.y) * (x3.x - x4.x)) / d;
        let u = ((x1.x - x3.x) * (x1.y - x2.y) - (x1.y - x3.y) * (x1.x - x2.x)) / d;
        (t >= 0.0 && u >= 0.0 && d.abs() > 1e-3).then_some((t, self.p.xy() + t * self.v.xy()))
        //(d.abs() > 1e-3).then_some((t, self.p.xy() + t * self.v.xy()))
    }

    fn at_time(&self, time: f64) -> DVec3 {
        self.p + time * self.v
    }

    //fn intersects_xz(&self, other: &Self) -> Option<glam::DVec2> {
    //let x1 = self.p.xz();
    //let x2 = self.p.xz() + self.v.xz();
    //let x3 = other.p.xz();
    //let x4 = other.p.xz() + other.v.xz();
    //let d = (x1.x - x2.x) * (x3.y - x4.y) - (x1.y - x2.y) * (x3.x - x4.x);
    //let t = ((x1.x - x3.x) * (x3.y - x4.y) - (x1.y - x3.y) * (x3.x - x4.x)) / d;
    //let u = ((x1.x - x3.x) * (x1.y - x2.y) - (x1.y - x3.y) * (x1.x - x2.x)) / d;
    //(t >= 0.0 && u >= 0.0 && d.abs() > 1e-3).then_some(self.p.xz() + t * self.v.xz())
    //}

    //fn intersects_yz(&self, other: &Self) -> Option<glam::DVec2> {
    //let x1 = self.p.yz();
    //let x2 = self.p.yz() + self.v.yz();
    //let x3 = other.p.yz();
    //let x4 = other.p.yz() + other.v.yz();
    //let d = (x1.x - x2.x) * (x3.y - x4.y) - (x1.y - x2.y) * (x3.x - x4.x);
    //let t = ((x1.x - x3.x) * (x3.y - x4.y) - (x1.y - x3.y) * (x3.x - x4.x)) / d;
    //let u = ((x1.x - x3.x) * (x1.y - x2.y) - (x1.y - x3.y) * (x1.x - x2.x)) / d;
    //(t >= 0.0 && u >= 0.0 && d.abs() > 1e-3).then_some(self.p.yz() + t * self.v.yz())
    //}

    fn rotate(&self, rotation: &DMat3) -> Self {
        Self {
            p: *rotation * self.p,
            v: *rotation * self.v,
        }
    }
}

fn num_intersections_xy(points: &[HailGrain], x_range: (f64, f64), y_range: (f64, f64)) -> usize {
    points
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            let s = a.intersects_xy(b);
            //println!("{a} -> {b}");
            //dbg!(&s);
            s.map(|(_, s)| {
                s.x >= x_range.0 && s.x <= x_range.1 && s.y >= y_range.0 && s.y <= y_range.1
            })
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

    let grains = regex
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

    //dbg!(&points)Part1 24;
    //let part1 = num_intersections(&points, (7., 27.), (7., 27.));
    let part1 = num_intersections_xy(
        &grains,
        (200000000000000., 400000000000000.),
        (200000000000000., 400000000000000.),
    );
    dbg!(&part1);

    let search_max = 5000i64;

    let unit_z = dvec3(0.0, 0.0, 1.0);
    let unit_x = dvec3(1.0, 0.0, 0.0);
    let bar = ProgressBar::new((search_max * 2 * 2 * 2 * search_max * search_max) as u64);
    let mut max_intersections = 0u64;
    let (times, real_intersections, dir) = (0i64..)
        .find_map(|h| {
            println!("{h}");
            for dx in -h..=h {
                for dy in -(h - dx.abs())..=(h - dx.abs()) {
                    'outer: for dz in -(h - dx.abs() - dy.abs())..=(h - dx.abs() - dy.abs()) {
                        bar.inc(1);
                        let dir = dvec3(dx as f64, dy as f64, dz as f64);
                        if dir.xy().length() < 1e-3 {
                            continue;
                        }
                        if dir.xz().length() < 1e-3 {
                            continue;
                        }
                        let dir = dir.normalize();
                        let rotation_z =
                            DMat3::from_rotation_z(unit_x.xy().angle_between(dir.xy())).transpose();
                        let y_is_zero = rotation_z * dir;
                        debug_assert!(y_is_zero.y.abs() < 1e-3);

                        let rotation_x =
                            DMat3::from_rotation_y(unit_z.xz().angle_between(y_is_zero.xz()));
                        let xy_is_zero = rotation_x * y_is_zero;
                        debug_assert!(xy_is_zero.x.abs() < 1e-3);
                        debug_assert!(xy_is_zero.y.abs() < 1e-3);
                        let rotation = rotation_x * rotation_z;

                        let dir_rotated = rotation * dir;
                        debug_assert!(dir_rotated.x.abs() < 1e-3);
                        debug_assert!(dir_rotated.y.abs() < 1e-3);

                        //let mut times = Vec::new();
                        //let mut real_intersections = Vec::new();
                        let mut times = None;
                        let mut real_intersections = None;
                        let mut common_intersection: Option<DVec2> = None;
                        let mut counter = 0u64;
                        for (a, b) in grains.iter().tuple_windows() {
                            let arot = a.rotate(&rotation);
                            let brot = b.rotate(&rotation);
                            if let Some((t, intersection)) = arot.intersects_xy(&brot) {
                                //dbg!(&intersection);
                                //times.push(t);
                                //real_intersections.push(a.at_time(t));
                                if times.is_none() {
                                    times = Some(t);
                                    real_intersections = Some(a.at_time(t));
                                }

                                if let Some(common_intersection) = common_intersection {
                                    if (common_intersection - intersection).length_squared() > 1e-3
                                    {
                                        //println!("intersection not match");
                                        continue 'outer;
                                    }
                                } else {
                                    common_intersection = Some(intersection);
                                }
                            } else {
                                let an = a.v.normalize();
                                let bn = b.v.normalize();
                                // parallel / antiparallel
                                if (an - bn).length() < 1e-3 || (an + bn).length() < 1e-3 {
                                    //continue;
                                } else {
                                    //println!("no interseciton");
                                    continue 'outer;
                                }
                            }
                            counter += 1;
                            if counter > max_intersections {
                                max_intersections = counter;
                                println!("{max_intersections}/{}", grains.len() - 1);
                            }
                        }
                        //common_intersection.map(|c| (rotation.transpose() * c, (dx, dy, dz)))
                        return Some((times.unwrap(), real_intersections.unwrap(), (dx, dy, dz)));
                    }
                }
            }
            None
        })
        .unwrap();
    let pos = real_intersections - times * dvec3(dir.0 as f64, dir.1 as f64, dir.2 as f64);
    dbg!(&dir);
    //dbg!(&real_intersections);
    //dbg!(&times);

    let part2 = pos.x.round() + pos.y.round() + pos.z.round();
    dbg!(&part2);
    //let MinMaxResult::MinMax(min_x, max_x) = grains.iter().map(|a| a.p.x).minmax() else {
    //unreachable!()
    //};
    //let MinMaxResult::MinMax(min_y, max_y) = grains.iter().map(|a| a.p.y).minmax() else {
    //unreachable!()
    //};
    //let MinMaxResult::MinMax(min_z, max_z) = grains.iter().map(|a| a.p.z).minmax() else {
    //unreachable!()
    //};
    //let min_x = min_x as i64;
    //let max_x = max_x as i64;
    //let min_y = min_y as i64;
    //let max_y = max_y as i64;
    //let min_z = min_z as i64;
    //let max_z = max_z as i64;

    //let aabb = Aabb {
    //x_range: min_x..=max_x,
    //y_range: min_y..=max_y,
    //z_range: min_z..=max_z,
    //};
    //dbg!(&aabb);
    //dbg!(aabb.x_range.count() as i128 * aabb.y_range.count() as i128 * aabb.z_range.count() as i128);

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
