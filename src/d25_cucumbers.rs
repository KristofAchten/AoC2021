use std::time::Instant;

use crate::get_input_for_day;

pub fn move_cucumbers() -> String {
    let now = Instant::now();
    let input = parse_input();

    let res1 = solve_until_halt(&input, 1);

    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", res1, "push the button", now.elapsed().as_millis());
}

fn solve_until_halt(input: &Vec<Vec<char>>, step: i64) -> i64 {
    let mut mut_input = input.clone();
    let num_of_rows = input.len();
    let num_of_cols = input[0].len();

    // East
    for y in 0..num_of_rows {
        for x in 0..num_of_cols {
            if input[y][x] == '>' {
                if input[y][(x + 1) % num_of_cols] == '.' {
                    mut_input[y][x] = '.';
                    mut_input[y][(x + 1) % num_of_cols] = '>';
                } else {
                    mut_input[y][x] = input[y][x].clone();
                }
            }
        }
    }

    // South
    let after_east_mov = mut_input.clone();
    for y in 0..num_of_rows {
        for x in 0..num_of_cols {
            if after_east_mov[y][x] == 'v' {
                if after_east_mov[(y + 1) % num_of_rows][x] == '.' {
                    mut_input[y][x] = '.';
                    mut_input[(y + 1) % num_of_rows][x] = 'v';
                } else {
                    mut_input[y][x] = input[y][x].clone();
                }
            }
        }
    }

    if input == &mut_input {
        return step;
    }

    return solve_until_halt(&mut_input.clone(), step + 1);
}

fn parse_input() -> Vec<Vec<char>> {
    get_input_for_day(25).lines()
        .map(|l| l.chars().collect())
        .collect()
}