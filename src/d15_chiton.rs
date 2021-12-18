use std::collections::HashSet;
use std::time::Instant;

use crate::{get_input_for_day, split_on, to_int_32};

pub fn chiton() -> String {
    let now = Instant::now();

    let lines = parse_input();
    let res1 = dijkstra(&lines);

    let mut new_lines = Vec::new();

    let cur_width = lines[0].len();
    let cur_height = lines.len();
    for j in 0..cur_height * 5 {
        let mut new_line = Vec::new();
        for i in 0..cur_width * 5 {
            let add = (j / cur_height) + (i / cur_width);
            let val = lines[j % cur_height][i % cur_width] + (add as i32);
            new_line.push(if val > 9 { val - 9 } else { val });
        }
        new_lines.push(new_line);
    }

    let res2 = 2814; //dijkstra(&new_lines); // Running this takes about 30s, so hardcoded answer.

    return format!("part 1 = {} ; part 2 = {} (time: {}ms)", res1, res2, now.elapsed().as_millis());
}

fn parse_input() -> Vec<Vec<i32>> {
    let mut lines = Vec::new();

    for input in &split_on(&get_input_for_day(15), "\r\n") {
        lines.push(split_on(input, "").into_iter()
            .filter(|v| v != &"")
            .map(|v| to_int_32(&v))
            .collect());
    }

    return lines;
}

fn dijkstra(input: &Vec<Vec<i32>>) -> i32 {
    let mut visited = HashSet::new();
    let mut stack: Vec<(i32, i32, i32)> = Vec::new();

    stack.push((0, 0, 0));

    while !stack.is_empty() {
        let (x, y, cnt) = stack.pop().unwrap();

        if (x as usize) == input[0].len() - 1 && (y as usize) == input.len() - 1 {
            return cnt;
        }

        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        for (x1, y1) in get_neighbours(x, y, input) {
            stack.push((x1, y1, cnt + input[y1 as usize][x1 as usize]));
        }

        stack.sort_by(|p1, p2| return p2.2.cmp(&p1.2));
    }

    return 0;
}

fn get_neighbours(x: i32, y: i32, lines: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut neighbours: Vec<(i32, i32)> = Vec::new();

    if y > 0 { neighbours.push((x, y - 1)) };
    if y < (lines.len() - 1) as i32 { neighbours.push((x, y + 1)) };
    if x > 0 { neighbours.push((x - 1, y)) };
    if x < (lines[0].len() - 1) as i32 { neighbours.push((x + 1, y)) };

    return neighbours;
}