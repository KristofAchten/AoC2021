use std::time::Instant;

use crate::{get_input_for_day, split_on, to_int_32};

pub fn sonar_sweep() -> String {
    let now = Instant::now();

    let nums: Vec<String> = split_on(&get_input_for_day(1), "\n");

    let mut prev_num = 90000;
    let mut prev_threesome_num = 90000;
    let mut cnt = 0;
    let mut threesome_cnt = 0;
    let mut i = 0;

    while i < nums.len() {
        let n1 = to_int_32(&nums[i]);

        if n1 > prev_num {
            cnt += 1;
        }
        prev_num = n1;

        if i > 1 {
            let n2 = to_int_32(&nums[i - 1]);
            let n3 = to_int_32(&nums[i - 2]);
            let sum = n1 + n2 + n3;

            if sum > prev_threesome_num {
                threesome_cnt += 1
            }
            prev_threesome_num = sum
        }

        i += 1;
    }

    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", cnt, threesome_cnt, now.elapsed().as_millis())
}