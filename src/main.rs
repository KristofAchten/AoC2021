use std::fs;
use crate::d1_sonar_sweep::sonar_sweep;

mod d1_sonar_sweep;

fn main() {
    print!("Day 1 - 'Sonar Sweep' results: "); sonar_sweep();
}

pub fn get_input_for_day(day: i8) -> String {
    let path = format!("src/input/d{}_input.txt", day);
    return fs::read_to_string(path).expect("Unable to read file")
}

pub fn to_int_32(str: &String) -> i32 {
    return str.trim().parse::<i32>().unwrap();
}
