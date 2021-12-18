use std::time::Instant;

use crate::{get_input_for_day, split_on, to_int_32};

pub fn flash_octopi() -> String {
    let now = Instant::now();

    let mut input = parse_input();

    let res1 = calc_flashes(&mut input, 100);
    let res2 = find_full_pulse(&mut input, 100);

    return format!("part 1 = {} ; part 2 = {} (time: {}ms)", res1, res2, now.elapsed().as_millis());
}

fn calc_flashes(input: &mut Vec<Vec<i32>>, steps: i32) -> i32 {
    let mut flash_cnt = 0;

    (0..steps).into_iter().for_each(|_| {
        flash_cnt += step(input);
    });

    return flash_cnt;
}

fn find_full_pulse(input: &mut Vec<Vec<i32>>, start_cnt: i32) -> i32 {
    let mut step_cnt = start_cnt;
    let total_cells = (input[0].len() * input.len()) as i32;

    let mut flashes = 0i32;
    while flashes != total_cells {
        step_cnt += 1;
        flashes = step(input);
    }

    return step_cnt;
}

fn step(input: &mut Vec<Vec<i32>>) -> i32 {
    let mut total_flashes = 0;

    // Increase value by one for each cell
    increase_by_one(input);

    // Flash the shit out of shit
    let mut flashes = flash(input);
    while flashes > 0 {
        total_flashes += flashes;
        flashes = flash(input);
    }

    return total_flashes;
}

fn flash(input: &mut Vec<Vec<i32>>) -> i32 {
    let mut flashes = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] > 9 {
                flashes += 1;
                input[i][j] = 0;

                let neighbours = get_neighbours((i, j), input[0].len(), input.len());
                for neighbour in neighbours {
                    let (x, y) = neighbour;
                    let val = input[x][y];
                    if val != 0 {
                        input[x][y] += 1;
                    }
                }
            }
        }
    }

    return flashes;
}

fn increase_by_one(input: &mut Vec<Vec<i32>>) {
    for line in input {
        for val in line {
            *val += 1;
        }
    }
}

fn get_neighbours(point: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    let (i, j) = point;

    if i > 0 {
        neighbours.push((i - 1, j));
        if j > 0 {
            neighbours.push((i - 1, j - 1));
        }

        if j < width - 1 {
            neighbours.push((i - 1, j + 1))
        }
    }

    if i < height - 1 {
        neighbours.push((i + 1, j));
        if j > 0 {
            neighbours.push((i + 1, j - 1));
        }

        if j < width - 1 {
            neighbours.push((i + 1, j + 1))
        }
    }

    if j > 0 {
        neighbours.push((i, j - 1));
    }

    if j < width - 1 {
        neighbours.push((i, j + 1))
    }

    return neighbours;
}

fn parse_input() -> Vec<Vec<i32>> {
    split_on(&get_input_for_day(11), "\r\n").into_iter()
        .map(|v| split_on(&v, "").into_iter().filter(|v| v != &"").map(|v| to_int_32(&v)).collect())
        .collect()
}