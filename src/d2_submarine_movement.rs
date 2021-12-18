use std::time::Instant;

use crate::{get_input_for_day, split_on, to_int_32};

pub fn move_submarine() -> String{
    let now = Instant::now();

    let movements: Vec<String> = split_on(&get_input_for_day(2), "\n");
    let mut horizontal_pos = 0;
    let mut aim = 0;
    let mut depth = 0;

    for movement in &movements {
        let parts: Vec<String> = split_on(&movement, " ");
        let dir = &parts[0];
        let val = to_int_32(&parts[1]);

        match dir as &str {
            "up" => aim -= val,
            "down" => aim += val,
            "forward" => {
                horizontal_pos += val;
                depth += aim * val;
            }
            _ => {}
        }
    }

    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", horizontal_pos * aim, horizontal_pos * depth, now.elapsed().as_millis());
}