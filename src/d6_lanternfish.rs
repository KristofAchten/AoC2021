use std::time::Instant;

use crate::{get_input_for_day, split_on, to_int_32};

pub fn calc_lanternfish() -> String {
    let now = Instant::now();

    let input: Vec<i32> = split_on(&get_input_for_day(6), ",").into_iter()
        .map(|v| to_int_32(&v))
        .collect();

    let mut buckets = Vec::new();

    for i in 0..=8 {
        buckets.push(input.iter().filter(|v| v == &&i).count())
    }

    let res_1 = run(&buckets, 80);
    let res_2 = run(&buckets, 256);

    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", res_1, res_2, now.elapsed().as_millis());
}

fn run(input: &Vec<usize>, steps: i32) -> usize {
    let result = (0..steps).into_iter()
        .fold(input.clone(), |mut res, _| {
            let new_fish = res[0];
            res.rotate_left(1);
            res[6] += new_fish;
            return res;
        });

    return result.into_iter().sum();
}