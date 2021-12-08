use std::fs;

use crate::d1_sonar_sweep::sonar_sweep;
use crate::d2_submarine_movement::move_submarine;
use crate::d3_binary_diagnostic::run_diagnostic;
use crate::d4_bingo::play_bingo;
use crate::d5_hydrothermal_venture::draw_all_lines;
use crate::d6_lanternfish::calc_lanternfish;
use crate::d7_whale::calc_alignment;
use crate::d8_digits::digits;

mod d1_sonar_sweep;
mod d2_submarine_movement;
mod d3_binary_diagnostic;
mod d4_bingo;
mod d5_hydrothermal_venture;
mod d6_lanternfish;
mod d7_whale;
mod d8_digits;

fn main() {
    print!("Day 1 - 'Sonar Sweep' results: ");
    sonar_sweep();
    print!("Day 2 - 'Submarine Movement' results: ");
    move_submarine();
    print!("Day 3 - 'Binary Diagnostics' results: ");
    run_diagnostic();
    print!("Day 4 - 'Bingo!' results: ");
    play_bingo();
    print!("Day 5 - 'Hydrothermal Ventures' results: ");
    draw_all_lines();
    print!("Day 6 - 'Lanternfish' results: ");
    calc_lanternfish();
    print!("Day 7 - 'Alignment' results: ");
    calc_alignment();
    print!("Day 8 - 'Seven segment displays' results: ");
    digits();
}

pub fn get_input_for_day(day: i8) -> String {
    let path = format!("src/input/d{}_input.txt", day);
    return fs::read_to_string(path).expect("Unable to read file");
}

pub fn to_int_32(str: &String) -> i32 {
    return str.trim().parse::<i32>().unwrap();
}

pub fn split_on(str: &str, on: &str) -> Vec<String> {
    return str.split(on).map(str::to_string).collect();
}