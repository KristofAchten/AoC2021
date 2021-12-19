use std::collections::HashSet;
use std::time::Instant;

use crate::{get_input_for_day, split_on, to_int_32};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn minus(&self, other: &Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn plus(&self, other: &Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn manhattan_dist(&self, other: &Point3D) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

pub fn scan(exec: bool) -> String {
    let now = Instant::now();
    if exec {
        let rotations = get_rotations();
        let input = parse_input();

        let (res1, res2) = reduce(&input, &rotations);

        return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", res1, res2, now.elapsed().as_millis());
    }

    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", 318, 12166, now.elapsed().as_millis());
}

fn get_rotations() -> Vec<Box<dyn Fn(&Point3D) -> Point3D>> {
    let mut rotations: Vec<Box<dyn Fn(&Point3D) -> Point3D>> = Vec::new();

    // order: x, y, z shifted
    for (first, second, third) in &[(1, 1, 1), (1, -1, -1), (-1, 1, -1), (-1, -1, 1)] {
        let fun1 = |point: &Point3D| Point3D { x: point.x * first.clone(), y: point.y * second.clone(), z: point.z * third.clone() };
        let fun2 = |point: &Point3D| Point3D { x: point.y * second.clone(), y: point.z * third.clone(), z: point.x * first.clone() };
        let fun3 = |point: &Point3D| Point3D { x: point.z * third.clone(), y: point.x * first.clone(), z: point.y * second.clone() };

        rotations.push(Box::from(fun1));
        rotations.push(Box::from(fun2));
        rotations.push(Box::from(fun3));
    }

    // order: x, z, y shifted
    for (first, second, third) in &[(1, 1, -1), (1, -1, 1), (-1, 1, 1), (-1, -1, -1)] {
        let fun4 = |point: &Point3D| Point3D { x: point.y * second.clone(), y: point.x * first.clone(), z: point.z * third.clone() };
        let fun5 = |point: &Point3D| Point3D { x: point.z * third.clone(), y: point.y * second.clone(), z: point.x * first.clone() };
        let fun6 = |point: &Point3D| Point3D { x: point.x * first.clone(), y: point.z * third.clone(), z: point.y * second.clone() };

        rotations.push(Box::from(fun4));
        rotations.push(Box::from(fun5));
        rotations.push(Box::from(fun6));
    }

    return rotations;
}


fn parse_input() -> Vec<Vec<Point3D>> {
    split_on(&get_input_for_day(19), "\r\n\r\n").into_iter()
        .map(|part| {
            let lines = part.lines().skip(1);
            lines.map(|line| {
                let parts = split_on(line, ",");
                Point3D {
                    x: to_int_32(&parts[0]) as i64,
                    y: to_int_32(&parts[1]) as i64,
                    z: to_int_32(&parts[2]) as i64,
                }
            }).collect()
        }).collect()
}

fn reduce(input: &Vec<Vec<Point3D>>, rotations: &Vec<Box<dyn Fn(&Point3D) -> Point3D>>) -> (usize, usize) {
    let mut input_mut = input.clone();
    let mut all_beacons = HashSet::new();
    for beacon in input_mut[0].clone() {
        all_beacons.insert(beacon.clone());
    }
    input_mut.remove(0);

    let mut distances = Vec::new();
    while !input_mut.is_empty() {
        let mut to_remove = Vec::new();
        for i in (0..input_mut.len()).rev() {
            let (reduced, dist) = reduction_step(&mut all_beacons, &input_mut[i], &rotations);
            if reduced {
                distances.push(dist);
                to_remove.push(i);
            }
        }
        to_remove.sort();
        to_remove.reverse();

        to_remove.iter().for_each(|&i| { let _ = input_mut.remove(i); });
    }

    let mut max = 0;
    for i in 0..distances.len() {
        for j in i..distances.len() {
            max = max.max(distances[i].manhattan_dist(&distances[j]))
        }
    }
    return (all_beacons.len(), max as usize);
}

fn reduction_step(beacons: &mut HashSet<Point3D>, input: &Vec<Point3D>, rotations: &Vec<Box<dyn Fn(&Point3D) -> Point3D>>) -> (bool, Point3D) {
    for rotation in rotations {
        let rotated: Vec<Point3D> = input.iter().map(|v| rotation(v).clone()).collect();
        let dist: Vec<Point3D> = beacons.iter().map(|p1| {
            rotated.iter().map(|p2| {
                p1.minus(p2)
            })
        }).flatten().collect();

        for d in dist {
            let shifted: Vec<Point3D> = rotated.iter().map(|p| p.plus(&d).clone()).collect();
            let match_cnt = shifted.iter().filter(|v| beacons.contains(v)).count();
            if match_cnt >= 12 {
                shifted.iter().for_each(|p| { let _ = beacons.insert(p.clone()); });
                return (true, d);
            }
        }
    }
    return (false, Point3D { x: 0, y: 0, z: 0 });
}
