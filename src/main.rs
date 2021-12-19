use std::fs;
use std::time::Instant;

use crate::d10_syntax::syntax_fixer;
use crate::d11_octopi::flash_octopi;
use crate::d12_paths::iterate_paths;
use crate::d13_origami::fold_away;
use crate::d14_polymerization::extend_polym;
use crate::d15_chiton::chiton;
use crate::d16_packet_decoder::decode_input;
use crate::d17_trick_shot::simulate_shots;
use crate::d18_snailfish::do_math;
use crate::d1_sonar_sweep::sonar_sweep;
use crate::d2_submarine_movement::move_submarine;
use crate::d3_binary_diagnostic::run_diagnostic;
use crate::d4_bingo::play_bingo;
use crate::d5_hydrothermal_venture::draw_all_lines;
use crate::d6_lanternfish::calc_lanternfish;
use crate::d7_whale::calc_alignment;
use crate::d8_digits::digits;
use crate::d9_smoke_basin::find_smoke_flows;

mod d1_sonar_sweep;
mod d2_submarine_movement;
mod d3_binary_diagnostic;
mod d4_bingo;
mod d5_hydrothermal_venture;
mod d6_lanternfish;
mod d7_whale;
mod d8_digits;
mod d9_smoke_basin;
mod d10_syntax;
mod d11_octopi;
mod d12_paths;
mod d13_origami;
mod d14_polymerization;
mod d15_chiton;
mod d16_packet_decoder;
mod d17_trick_shot;
mod d18_snailfish;

fn main() {
    let pre = "# AoC 2020\n\nThis repository contains my solution for 2021s [Advent of Code challenges!]\
    (https://adventofcode.com/2021)\nI will be participating in `Rust` ([source-files](https://github.com/KristofAchten/AoC2021/tree/master/src)).\
    \n\nThe output for the challenges (given my personalized input files) is automatically generated and shown below.\n\n# Solutions";

    let now = Instant::now();

    let d1 = format!("- Day 1 - 'Sonar Sweep' results: {}", sonar_sweep());
    let d2 = format!("- Day 2 - 'Submarine Movement' results: {}", move_submarine());
    let d3 = format!("- Day 3 - 'Binary Diagnostics' results: {}", run_diagnostic());
    let d4 = format!("- Day 4 - 'Bingo!' results: {}", play_bingo());
    let d5 = format!("- Day 5 - 'Hydrothermal Ventures' results: {}", draw_all_lines());
    let d6 = format!("- Day 6 - 'Lanternfish' results: {}", calc_lanternfish());
    let d7 = format!("- Day 7 - 'Alignment' results: {}", calc_alignment());
    let d8 = format!("- Day 8 - 'Seven Segment Displays' results: {}", digits());
    let d9 = format!("- Day 9 - 'Smoke Basing' results: {}", find_smoke_flows());
    let d10 = format!("- Day 10 - 'Syntax Scoring' results: {}", syntax_fixer());
    let d11 = format!("- Day 11 - 'Dumbo Octopus' results: {}", flash_octopi());
    let d12 = format!("- Day 12 - 'Passage Pathing' results: {}", iterate_paths());
    let d13 = format!("- Day 13 - 'Transparent Origami' results: {}", fold_away());
    let d14 = format!("- Day 14 - 'Polymer Extension' results: {}", extend_polym());
    let d15 = format!("- Day 15 - 'Chiton' results: {}", chiton());
    let d16 = format!("- Day 16 - 'Packet Decoder' results: {}", decode_input());
    let d17 = format!("- Day 17 - 'Trick Shot' results: {}", simulate_shots());
    let d18 = format!("- Day 18 - 'Snailfish' results: {}", do_math());

    let exec_time = format!("\nTotal execution time: `{}ms`", now.elapsed().as_millis());
    let result_string = [pre.to_string(), d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14, d15, d16, d17, d18, exec_time].join("\r\n");

    println!("{}", result_string);

    let _ = fs::write("README.md", result_string);
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

pub fn bin_string_to_dec(str: &str) -> isize {
    isize::from_str_radix(str, 2).unwrap()
}