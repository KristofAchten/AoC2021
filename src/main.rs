use std::fs;
use crate::d1_sonar_sweep::sonar_sweep;
use crate::d2_submarine_movement::move_submarine;
use crate::d3_binary_diagnostic::run_diagnostic;
use crate::d4_bingo::play_bingo;

mod d1_sonar_sweep;
mod d2_submarine_movement;
mod d3_binary_diagnostic;
mod d4_bingo;

fn main() {
    print!("Day 1 - 'Sonar Sweep' results: "); sonar_sweep();
    print!("Day 2 - 'Submarine Movement' results: "); move_submarine();
    print!("Day 3 - 'Binary Diagnostics' results: "); run_diagnostic();
    print!("Day 4 - 'Bingo!' results: "); play_bingo();
}

pub fn get_input_for_day(day: i8) -> String {
    let path = format!("src/input/d{}_input.txt", day);
    return fs::read_to_string(path).expect("Unable to read file")
}

pub fn to_int_32(str: &String) -> i32 {
    return str.trim().parse::<i32>().unwrap();
}

pub fn split_on(str: &str, on: &str) -> Vec<String> {
    return str.split(on).map(str::to_string).collect();
}