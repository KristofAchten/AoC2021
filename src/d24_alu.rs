use std::time::Instant;

pub fn magic() -> String {
    let now = Instant::now();

    let (res1, res2) = solve();

    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", res1, res2, now.elapsed().as_millis());
}

fn solve() -> (i64, i64) {
    let mut res1 = 0;

    for x in (1111111..9999999).rev() {
        let nums: Vec<u32> = x.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        if let Some(res) = solve_for_digits(nums) {
            res1 = res;
            break;
        }
    }

    for x in 1111111..9999999 {
        let nums: Vec<u32> = x.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        if let Some(res) = solve_for_digits(nums) {
            return (res1, res);
        }
    }

    return (0, 0);
}

const NONDETER: [&'static Option<i64>; 14] = [&Some(15), &Some(8), &Some(2), &None, &Some(13), &Some(4), &Some(1), &None, &Some(5), &None, &None, &None, &None, &None];
const DETER: [&'static Option<i64>; 14] = [&None, &None, &None, &Some(9), &None, &None, &None, &Some(5), &None, &Some(7), &Some(12), &Some(10), &Some(1), &Some(11)];

fn solve_for_digits(digits: Vec<u32>) -> Option<i64> {
    if digits.contains(&0) {
        return None;
    }

    let mut z = 0;
    let mut res = Vec::new();
    let mut digits_idx = 0;

    for i in 0..14 {
        if let Some(val) = DETER[i] {
            let digit = (z % 26) - val;
            res.push(digit.clone());
            if digit < 1 || digit > 9 {
                return None;
            }
            z = (z as f64 / 26.0).floor() as i64;
        } else {
            let val = NONDETER[i].unwrap();
            z = z * 26 + (digits[digits_idx] as i64) + val;
            res.push(digits[digits_idx] as i64);
            digits_idx += 1;
        }
    }

    return Some(res.iter().map(|v| v.to_string()).collect::<Vec<String>>()
        .join("").trim().parse::<i64>().unwrap());
}