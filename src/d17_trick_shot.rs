use std::time::Instant;

use crate::{get_input_for_day, split_on, to_int_32};

pub fn simulate_shots() {
    let now = Instant::now();
    let parts = split_on(&get_input_for_day(17)
        .replace("target area: x=", "")
        .replace(" y=", "")
        .replace("..", ","), ",");

    let x_range = (to_int_32(&parts[0]), to_int_32(&parts[1]));
    let y_range = (to_int_32(&parts[2]), to_int_32(&parts[3]));

    let (res1, res2) = find_neg_y(&x_range, &y_range);

    println!("part 1 = {} ; part 2 = {} (time: {}ms)", res1, res2, now.elapsed().as_millis());
}

fn find_neg_y(x_range: &(i32, i32), y_range: &(i32, i32)) -> (i32, i32) {
    let mut best_y_pos = -9000;
    let mut cnt = 0;
    for x_vi in 1..1000 {
        for y_vi in -1000..1000 {
            let mut pos = (0, 0);
            let mut vel = (x_vi, y_vi);
            let mut overshot = false;
            let mut best_y_local = -9000;
            while !overshot {
                pos = (pos.0 + vel.0, pos.1 + vel.1);
                vel = (if vel.0 > 0 { vel.0 - 1 } else if vel.0 < 0 { vel.0 + 1 } else { 0 }, vel.1 - 1);

                best_y_local = pos.1.max(best_y_local);

                if pos.0 >= x_range.0 && pos.0 <= x_range.1 && pos.1 >= y_range.0 && pos.1 <= y_range.1 {
                    best_y_pos = best_y_local.max(best_y_pos);
                    cnt += 1;
                    break;
                }

                overshot = pos.0 > x_range.1 || pos.1 < y_range.0;
            }
        }
    }

    return (best_y_pos, cnt);
}