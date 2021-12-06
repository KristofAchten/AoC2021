use std::time::Instant;
use crate::{get_input_for_day, split_on, to_int_32};

struct Game {
    draw: Vec<i32>,
    boards: Vec<Board>,
}

struct Board {
    values: Vec<Vec<Square>>,
    has_won: bool,
}

#[derive(Debug)]
struct Square {
    value: i32,
    crossed: bool,
}

pub fn play_bingo() {
    let now = Instant::now();

    let (first_win, last_win) = play_game(parse_input());
    println!("part 1 = {} ; part 2 = {} (time: {}ms)", first_win, last_win, now.elapsed().as_millis());
}

fn play_game(mut game: Game) -> (i32, i32) {
    let mut first_result = 0;
    let mut last_result = 0;

    for num in game.draw {
        for mut board in &mut game.boards {
            let has_won = board.has_won;

            play_num(board, num);

            let (is_winner, result) = is_winner(&mut board, num);

            if is_winner {
                if first_result == 0 {
                    first_result = result;
                };

                if !has_won {
                    last_result = result;
                }
            }
        }
    }

    return (first_result, last_result);
}

fn parse_input() -> Game {
    let parts: Vec<String> = split_on(&get_input_for_day(4), "\r\n\r\n");
    let draw: Vec<i32> = split_on(&parts[0], ",").into_iter()
        .map(|v| to_int_32(&v))
        .collect();
    let mut boards: Vec<Board> = Vec::new();
    for i in 1..parts.len() {
        boards.push(parse_board(&parts[i]))
    }

    return Game { draw, boards };
}

fn parse_board(str: &String) -> Board {
    let lines: Vec<String> = split_on(str, "\r\n");
    let mut values: Vec<Vec<Square>> = Vec::new();

    for line in lines {
        values.push(split_on(&line, " ").into_iter()
            .filter(|v| v != &"".to_string())
            .map(|v| to_int_32(&v))
            .map(|v| Square { value: v, crossed: false })
            .collect())
    }

    return Board { values, has_won: false };
}

fn play_num(board: &mut Board, num: i32) {
    for line in &mut board.values {
        for square in line {
            if square.value == num {
                square.crossed = true;
            }
        }
    }
}

fn is_winner(board: &mut Board, num: i32) -> (bool, i32) {
    // Rows
    for line in &board.values {
        if line.iter().all(|l| l.crossed == true) {
            board.has_won = true;
            return (true, calc_result(board, num));
        }
    }

    // Columns
    for i in 0..board.values[0].len() {
        let mut found = true;
        for j in 0..board.values.len() {
            if board.values[j][i].crossed == false {
                found = false;
            }
        }
        if found {
            board.has_won = true;
            return (true, calc_result(board, num));
        }
    }

    return (false, 0);
}

fn calc_result(board: &Board, num: i32) -> i32 {
    let mut sum = 0;
    for line in &board.values {
        for val in line {
            if val.crossed == false {
                sum += val.value;
            }
        }
    }

    return sum * num;
}

#[allow(dead_code)]
fn print_game(game: &Game) {
    println!("Drawn nums: {:?}", game.draw);
    for board in &game.boards {
        print_board(&board, false);
        println!("\n");
    }
}

#[allow(dead_code)]
fn print_board(board: &Board, show_crossed: bool) {
    for line in &board.values {
        if show_crossed {
            println!("{:?}", line);
        } else {
            println!("{:?}", line.into_iter().map(|l| l.value).collect::<Vec<i32>>())
        };
    }
}
