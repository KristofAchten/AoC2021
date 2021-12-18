use std::collections::HashSet;
use std::time::Instant;

use crate::{get_input_for_day, split_on, to_int_32};

pub fn find_smoke_flows() -> String {
    let now = Instant::now();

    let input = parse_input();
    let (res1, res2) = calc_minima_and_basins(&input);

    return format!("part 1 = {} ; part 2 = {} (time: {}ms)", res1, res2, now.elapsed().as_millis());
}

fn calc_minima_and_basins(input: &Vec<Vec<i32>>) -> (i32, i32) {
    let mut depth_calc = 0;
    let mut basin_size = Vec::new();

    for j in 0..input.len() {
        for i in 0..input[0].len() {
            let val = input[j][i];
            let up = if j > 0 { input[j - 1][i] } else { 9001 };
            let down = if j < input.len() - 1 { input[j + 1][i] } else { 9001 };
            let left = if i > 0 { input[j][i - 1] } else { 9001 };
            let right = if i < input[0].len() - 1 { input[j][i + 1] } else { 9001 };

            if val < up && val < down && val < left && val < right {
                basin_size.push(calc_basin_size((i, j), input));
                depth_calc += val + 1;
            }
        }
    }

    basin_size.sort();
    basin_size.reverse();

    return (depth_calc, basin_size[0] * basin_size[1] * basin_size[2]);
}

fn calc_basin_size(start: (usize, usize), input: &Vec<Vec<i32>>) -> i32 {
    let mut cnt = 0;
    let i = start.0;
    let j = start.1;

    let mut todo = HashSet::new();
    let mut seen = HashSet::new();

    todo.insert((i, j));

    while !todo.is_empty() {
        let pop = todo.clone().into_iter().next().unwrap();
        seen.insert(pop);
        todo.remove(&pop);

        if input[pop.1][pop.0] == 9 {
            continue;
        }

        cnt += 1;

        let relevant_neighbours: Vec<(usize, usize)> = calc_neighbours(input, pop.0, pop.1)
            .into_iter()
            .filter(|v| !seen.contains(v))
            .collect();

        for neighbour in relevant_neighbours {
            todo.insert(neighbour);
        }
    }
    return cnt;
}

fn calc_neighbours(input: &Vec<Vec<i32>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();

    if j > 0 { neighbours.push((i, j - 1)); }
    if j < input.len() - 1 { neighbours.push((i, j + 1)); }
    if i > 0 { neighbours.push((i - 1, j)); }
    if i < input[0].len() - 1 { neighbours.push((i + 1, j)); }

    return neighbours;
}

fn parse_input() -> Vec<Vec<i32>> {
    let input_lines = split_on(&get_input_for_day(9), "\r\n");

    let mut result = Vec::new();
    for line in input_lines {
        let mut vec_cur_line = Vec::new();
        split_on(&line, "").into_iter()
            .filter(|v| v != &"")
            .for_each(|v| vec_cur_line.push(to_int_32(&v)));

        result.push(vec_cur_line);
    }

    return result;
}