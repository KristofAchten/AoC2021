use std::cmp::{max, min};
use std::collections::HashMap;
use std::time::Instant;

use crate::{get_input_for_day, split_on, to_int_32};

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

struct Line {
    p1: Point,
    p2: Point,
}

pub fn draw_all_lines() -> String {
    let now = Instant::now();

    let lines = parse_input();
    let visited = HashMap::new();
    let visited_with_diags = HashMap::new();


    let nums = draw_lines(&lines, visited, false);
    let nums_with_diags = draw_lines(&lines, visited_with_diags, true);

    return format!("part 1 = {} ; part 2 = {} (time: {}ms)", nums, nums_with_diags, now.elapsed().as_millis());
}

fn draw_lines(lines: &Vec<Line>, mut visited: HashMap<Point, i32>, include_diags: bool) -> usize {
    for line in lines {
        let connected_points = find_connected_points(&line.p1, &line.p2, include_diags);
        for point in connected_points {
            if visited.contains_key(&point) {
                let val = visited.get(&point).unwrap() + 1;
                visited.insert(Point { x: point.x, y: point.y }, val);
            } else {
                visited.insert(Point { x: point.x, y: point.y }, 1);
            }
        }
    }

    return visited.into_iter().filter(|v| v.1 >= 2).count();
}

fn parse_input() -> Vec<Line> {
    let parts: Vec<String> = split_on(&get_input_for_day(5), "\r\n");
    let mut lines: Vec<Line> = Vec::new();

    for part in parts {
        let coords: Vec<i32> = split_on(&part, " -> ")
            .into_iter()
            .map(|v| split_on(&v, ","))
            .flatten()
            .map(|v| to_int_32(&v))
            .collect();

        let p1 = Point { x: coords[0], y: coords[1] };
        let p2 = Point { x: coords[2], y: coords[3] };

        lines.push(Line { p1, p2 })
    }

    return lines;
}

fn find_connected_points(p1: &Point, p2: &Point, include_diagonals: bool) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    if p1.x == p2.x {
        let x = p1.x;
        for y in min(p1.y, p2.y)..=max(p1.y, p2.y) {
            points.push(Point { x, y })
        }
    } else if p1.y == p2.y {
        let y = p1.y;
        for x in min(p1.x, p2.x)..=max(p1.x, p2.x) {
            points.push(Point { x, y })
        }
    } else if include_diagonals {
        let diff = (p1.x - p2.x).abs();
        let dx = if p1.x > p2.x { -1 } else { 1 };
        let dy = if p1.y > p2.y { -1 } else { 1 };
        for i in 0..=diff {
            points.push(Point { x: p1.x + (i * dx), y: p1.y + (i * dy) });
        }
    }
    return points;
}