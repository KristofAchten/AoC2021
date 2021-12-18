use std::time::Instant;

use crate::{get_input_for_day, split_on, to_int_32};

pub fn calc_alignment() -> String {
    let now = Instant::now();

    let mut input: Vec<i32> = split_on(&get_input_for_day(7), ",").into_iter()
        .map(|v| to_int_32(&v))
        .collect();

    input.sort();

    let med = input[input.len() / 2];
    let avg = input.iter().sum::<i32>() / input.len() as i32;

    let mut total_med = 0;
    let mut total_avg = 0;
    for num in input {
        let med_diff = (num - med).abs();
        total_med += med_diff;

        let avg_diff = (num - avg).abs();
        total_avg += (avg_diff * (avg_diff + 1)) / 2;
    }

    return format!("part 1 = {} ; part 2 = {} (time: {}ms)", total_med, total_avg, now.elapsed().as_millis());
}