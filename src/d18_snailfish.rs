use std::fmt;
use std::fmt::Formatter;
use std::time::Instant;

use crate::{get_input_for_day, to_int_32};
use crate::d18_snailfish::SnailNum::{Lit, Pair};

#[derive(Clone)]
enum SnailNum {
    Lit(i64),
    Pair(Box<SnailNum>, Box<SnailNum>),
}

impl fmt::Display for SnailNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut str_builder = Vec::new();
        match self {
            Lit(val) => { str_builder.push(val.to_string()) }
            Pair(left, right) => {
                str_builder.push(format!("[{}, {}]", left, right));
            }
        }

        write!(f, "{}", str_builder.join(""))
    }
}

impl SnailNum {
    fn from_str(str: &str) -> (SnailNum, &str) {
        let first = str.chars().nth(0).unwrap();
        return if first == '[' {
            let (left, remainder) = SnailNum::from_str(&str[1..]);
            let (right, final_remainder) = SnailNum::from_str(remainder);
            let actual_remainder = if final_remainder.is_empty() { "" } else { &final_remainder[1..] };

            (Pair(Box::new(left), Box::new(right)), actual_remainder)
        } else {
            (Lit(to_int_32(&first.to_string()) as i64), &str[2..])
        };
    }

    fn explode(&mut self) -> bool {
        let (_, exploded) = SnailNum::explode_recursively(self, 0);
        exploded
    }

    fn split(&mut self) -> bool {
        match self {
            Lit(v) => {
                if *v >= 10 {
                    let left = Lit(*v / 2);
                    let right = Lit((*v + (2 - 1)) / 2);
                    *self = Pair(Box::from(left), Box::from(right));
                    return true;
                }
                false
            }
            Pair(left, right) => {
                {
                    let split = SnailNum::split(left);
                    return if split { split } else { SnailNum::split(right) };
                }
            }
        }
    }

    fn explode_recursively(num: &mut SnailNum, depth: i8) -> ((Option<i64>, Option<i64>), bool) {
        return match num {
            Lit(_) => { ((None, None), false) }
            Pair(left, right) => {
                if depth == 4 {
                    match (left.as_ref(), right.as_ref()) {
                        (Lit(left_val), Lit(right_val)) => {
                            let ret_pair = (Some(left_val.clone()), Some(right_val.clone()));
                            *num = Lit(0);
                            (ret_pair, true)
                        }
                        (_, _) => { panic!() }
                    }
                } else {
                    let (opt_res, left_exploded) = SnailNum::explode_recursively(left, depth + 1);
                    if left_exploded {
                        match &opt_res {
                            (x, Some(val_right)) => {
                                SnailNum::add_to_pair_lhs(right, val_right.clone());
                                ((*x, None), true)
                            }
                            _ => { (opt_res, left_exploded) }
                        }
                    } else {
                        let (opt_res_right, right_exploded) = SnailNum::explode_recursively(right, depth + 1);
                        if right_exploded {
                            match &opt_res_right {
                                (Some(val_left), x) => {
                                    SnailNum::add_to_pair_rhs(left, val_left.clone());
                                    ((None, *x), right_exploded)
                                }
                                _ => { (opt_res_right, right_exploded) }
                            }
                        } else {
                            ((None, None), false)
                        }
                    }
                }
            }
        };
    }

    fn add_to_pair_lhs(num: &mut Box<SnailNum>, val: i64) {
        match num.as_mut() {
            Lit(v) => { *v += val }
            Pair(l, _) => { SnailNum::add_to_pair_lhs(l, val); }
        }
    }

    fn add_to_pair_rhs(num: &mut Box<SnailNum>, val: i64) {
        match num.as_mut() {
            Lit(v) => { *v += val }
            Pair(_, r) => { SnailNum::add_to_pair_rhs(r, val); }
        }
    }
}

fn sum_up(nums: &mut Vec<SnailNum>) -> i64 {
    let final_num = nums.into_iter().reduce(|acc, num| {
        *acc = Pair(Box::from(acc.clone()), Box::from(num.clone()));
        fully_reduce(acc);
        acc
    }).unwrap();

    magnitude(&final_num)
}

fn fully_reduce(num: &mut SnailNum) {
    let mut reducing = true;

    while reducing {
        let mut exploding = true;
        while exploding {
            exploding = num.explode();
        }
        reducing = num.split();
    }
}

fn magnitude(num: &SnailNum) -> i64 {
    return match num {
        Lit(v) => { v.clone() as i64 }
        Pair(l, r) => { 3 * magnitude(l) + 2 * magnitude(r) }
    };
}

fn find_max(nums: &mut Vec<SnailNum>) -> i64 {
    let mut res = 0;

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let num1 = &nums[i];
            let num2 = &nums[j];

            let mut v1 = vec![num1.clone(), num2.clone()];
            let mut v2 = vec![num2.clone(), num1.clone()];

            let res1 = sum_up(&mut v1);
            let res2 = sum_up(&mut v2);

            res = res.max(res1);
            res = res.max(res2);
        }
    }

    return res;
}

pub fn do_math() -> String {
    let now = Instant::now();
    let mut snail_nums: Vec<SnailNum> = get_input_for_day(18)
        .lines()
        .into_iter()
        .map(|l| SnailNum::from_str(l).0)
        .collect();

    let mut snail_nums_copy = snail_nums.clone();

    let res1 = sum_up(&mut snail_nums);
    let res2 = find_max(&mut snail_nums_copy);

    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", res1, res2, now.elapsed().as_millis());
}