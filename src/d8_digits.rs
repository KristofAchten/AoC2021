use std::time::Instant;

use crate::{get_input_for_day, split_on, to_int_32};

struct SignalCase {
    patterns: Vec<String>,
    output: Vec<String>,
}

pub fn digits() -> String {
    let now = Instant::now();
    let signals: Vec<SignalCase> = parse_input();

    let res1 = calc_digits(&signals);
    let res2 = decode_digits(&signals);

    return format!("part 1 = {} ; part 2 = {} (time: {}ms)", res1, res2, now.elapsed().as_millis());
}

fn calc_digits(signals: &Vec<SignalCase>) -> usize {
    let mut cnt = 0;
    for signal in signals {
        cnt += signal.output.iter().map(|v| v.len()).filter(|v| v == &2 || v == &3 || v == &4 || v == &7).count();
    }

    return cnt;
}

fn decode_digits(signals: &Vec<SignalCase>) -> usize {
    let mut sum = 0;
    for signal in signals {
        let one = signal.patterns.iter().find(|v| v.len() == 2).unwrap();
        let four = signal.patterns.iter().find(|v| v.len() == 4).unwrap();

        let mut numbers = Vec::new();
        for seq_part in &signal.output {
            let len = seq_part.len();
            match len {
                2 => numbers.push("1".to_string()),
                3 => numbers.push("7".to_string()),
                4 => numbers.push("4".to_string()),
                5 => numbers.push(determine_len_5(one, four, &seq_part)),
                6 => numbers.push(determine_len_6(one, four, &seq_part)),
                7 => numbers.push("8".to_string()),
                _ => {}
            }
        }

        sum += to_int_32(&numbers.join(""));
    }

    return sum as usize;
}

fn determine_len_5(one: &String, four: &String, part: &String) -> String {
    let (overlap_with_one, overlap_with_four) = get_overlaps(one, four, part);

    return if overlap_with_one == 2 {
        "3".to_string()
    } else if overlap_with_four == 2 {
        "2".to_string()
    } else {
        "5".to_string()
    };
}

fn get_overlaps(one: &String, four: &String, part: &String) -> (usize, usize) {
    let one_chars: Vec<String> = split_on(one, "").into_iter().filter(|v| v != &"").collect();
    let four_chars: Vec<String> = split_on(four, "").into_iter().filter(|v| v != &"").collect();

    let mut part_chars_for_one: Vec<String> = split_on(part, "").into_iter().filter(|v| v != &"").collect();
    let mut part_chars_for_four: Vec<String> = part_chars_for_one.clone();

    part_chars_for_one.retain(|v| one_chars.contains(v));
    part_chars_for_four.retain(|v| four_chars.contains(v));

    let overlap_with_one = part_chars_for_one.len();
    let overlap_with_four = part_chars_for_four.len();

    return (overlap_with_one, overlap_with_four);
}

fn determine_len_6(one: &String, four: &String, part: &String) -> String {
    let (overlap_with_one, overlap_with_four) = get_overlaps(one, four, part);

    return if overlap_with_four == 4 {
        "9".to_string()
    } else if overlap_with_one == 2 {
        "0".to_string()
    } else {
        "6".to_string()
    };
}

fn parse_input() -> Vec<SignalCase> {
    let mut results = Vec::new();
    let lines = split_on(&get_input_for_day(8), "\r\n");

    for line in lines {
        let parts = split_on(&line, " | ");

        let patterns = split_on(&parts[0], " ");
        let output = split_on(&parts[1], " ");

        results.push(SignalCase { patterns, output });
    }

    return results;
}