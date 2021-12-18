use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::time::Instant;

use crate::{get_input_for_day, split_on, to_int_32};

struct Board {
    width: i32,
    height: i32,
    points: Vec<(i32, i32)>,
}

#[allow(unused_must_use)]
impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut board_vec = vec![vec![" "; self.width as usize]; self.height as usize];

        for (x, y) in &self.points {
            board_vec[y.clone() as usize][x.clone() as usize] = "X";
        }

        for line in board_vec {
            for val in line {
                write!(f, "{}", val);
            }
            writeln!(f);
        }

        write!(f, "")
    }
}

pub fn fold_away() -> String {
    let now = Instant::now();
    let (mut board, instructions) = parse_input();

    let first_instruction = &instructions[0];
    step(&mut board, first_instruction);

    let unique_points: HashSet<(i32, i32)> = HashSet::from_iter(board.points.clone());
    let res1 = unique_points.len();

    for i in 1..instructions.len() {
        step(&mut board, &instructions[i]);
    }

    return format!("part 1 = {} ; part 2 = {} (time: {}ms)\n\n{}", res1, "printed below", now.elapsed().as_millis(), board);
}

fn step(board: &mut Board, instruction: &String) {
    let parts = split_on(&instruction.replace("fold along ", ""), "=");
    let axis = &parts[0];
    let value = to_int_32(&parts[1]);

    match (axis.as_str(), value) {
        ("x", v) => {
            board.points.iter_mut().for_each(|(x, _)| {
                if *x > v { *x = v - (*x - v) }
            });
            board.width = v;
        }
        ("y", v) => {
            board.points.iter_mut().for_each(|(_, y)| {
                if *y > v { *y = v - (*y - v) }
            });
            board.height = v;
        }
        _ => {}
    }
}

fn parse_input() -> (Board, Vec<String>) {
    let points_and_instructions = split_on(&get_input_for_day(13), "\r\n\r\n");

    let mut points: Vec<(i32, i32)> = Vec::new();
    for line in split_on(&points_and_instructions[0], "\r\n") {
        let parts = split_on(&line, ",");
        points.push((to_int_32(&parts[0]), to_int_32(&parts[1])));
    }

    let max_x = points.iter().map(|&(x, _)| x).max();
    let max_y = points.iter().map(|&(_, y)| y).max();
    let board = Board {
        width: max_x.unwrap() + 1,
        height: max_y.unwrap() + 1,
        points,
    };
    let instructions = split_on(&points_and_instructions[1], "\r\n");

    return (board, instructions);
}