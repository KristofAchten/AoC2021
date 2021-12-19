use std::fmt;
use std::fmt::Formatter;
use std::time::Instant;

use crate::{get_input_for_day, to_int_32};
use crate::d18_snailfish::SnailNum::{Lit, Pair};

enum SnailNum {
    Lit(i32),
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
            (Lit(to_int_32(&first.to_string())), &str[2..])
        };
    }
}


pub fn do_math() -> String {
    let now = Instant::now();
    let snail_nums: Vec<SnailNum> = get_input_for_day(18)
        .lines()
        .into_iter()
        .map(|l| SnailNum::from_str(l).0)
        .collect();

    for v in snail_nums {
        println!("{}", v);
    }


    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", "Todo", "Todo", now.elapsed().as_millis());
}