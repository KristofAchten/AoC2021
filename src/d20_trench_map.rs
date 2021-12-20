use std::collections::HashSet;
use std::time::Instant;

use crate::{bin_string_to_dec, get_input_for_day, split_on};

pub fn enhance() -> String {
    let now = Instant::now();
    let (algo, mut points) = parse_input();

    enhance_for(&mut points, &algo, 2);
    let res1 = points.len();

    enhance_for(&mut points, &algo, 48);
    let res2 = points.len();

    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", res1, res2, now.elapsed().as_millis());
}

fn enhance_for(points: &mut HashSet<(isize, isize)>, algo: &Vec<char>, cnt: i8) {
    (0..cnt).for_each(|i| {
        step(points, algo, i % 2 == 0)
    })
}

fn step(points: &mut HashSet<(isize, isize)>, algo: &Vec<char>, is_even: bool) {
    let mut to_add = HashSet::new();
    let blinking = algo[0] == '#';

    let min_x = &points.iter().map(|(x, _)| x).min().unwrap().clone();
    let max_x = &points.iter().map(|(x, _)| x).max().unwrap().clone();
    let min_y = &points.iter().map(|(_, y)| y).min().unwrap().clone();
    let max_y = &points.iter().map(|(_, y)| y).max().unwrap().clone();

    for y in (min_y - 3)..=(max_y + 3) {
        for x in (min_x - 3)..=(max_x + 3) {
            let out_of_bounds = &x <= min_x || &x >= max_x || &y <= min_y || &y >= max_y;
            let point = (x, y);

            let idx = get_neighbour_index(&point, points, blinking && out_of_bounds && !is_even);

            if algo[idx] == '#' {
                to_add.insert(point.clone());
            }
        }
    }

    points.clear();
    points.extend(to_add);
}

fn get_neighbour_index((x, y): &(isize, isize), points: &HashSet<(isize, isize)>, blink: bool) -> usize {
    if blink {
        return 511;
    }

    let p1 = if points.contains(&(x - 1, y - 1)) { '#' } else { '.' };
    let p2 = if points.contains(&(x.clone(), y - 1)) { '#' } else { '.' };
    let p3 = if points.contains(&(x + 1, y - 1)) { '#' } else { '.' };
    let p4 = if points.contains(&(x - 1, y.clone())) { '#' } else { '.' };
    let p5 = if points.contains(&(x.clone(), y.clone())) { '#' } else { '.' };
    let p6 = if points.contains(&(x + 1, y.clone())) { '#' } else { '.' };
    let p7 = if points.contains(&(x - 1, y + 1)) { '#' } else { '.' };
    let p8 = if points.contains(&(x.clone(), y + 1)) { '#' } else { '.' };
    let p9 = if points.contains(&(x + 1, y + 1)) { '#' } else { '.' };

    let idx_str: String = String::from_iter([p1, p2, p3, p4, p5, p6, p7, p8, p9]);
    let binary_str = idx_str.replace(".", "0").replace("#", "1");

    return bin_string_to_dec(&binary_str) as usize;
}

fn parse_input() -> (Vec<char>, HashSet<(isize, isize)>) {
    let parts = split_on(&get_input_for_day(20), "\r\n\r\n");

    let algo = parts[0].chars().collect();

    let lines: Vec<&str> = parts[1].lines().collect();
    let mut result = HashSet::new();
    for y in 0..lines.len() {
        let line = lines[y];
        for x in 0..line.len() {
            let char = line.chars().nth(x).unwrap();
            if char == '#' {
                result.insert((x as isize, y as isize));
            }
        }
    }
    return (algo, result);
}