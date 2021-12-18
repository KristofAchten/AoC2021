use std::time::Instant;

use crate::{get_input_for_day, split_on};

const OPENING_CHARS: [&'static str; 4] = ["(", "{", "[", "<"];
const CLOSING_CHARS: [&'static str; 4] = [")", "}", "]", ">"];


pub fn syntax_fixer() -> String {
    let now = Instant::now();

    let input: Vec<Vec<String>> = parse_input();

    let (res1, legal_lines) = score_corrupted_lines(&input);
    let res2 = complete_lines(legal_lines);

    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", res1, res2, now.elapsed().as_millis());
}

fn score_corrupted_lines(lines: &Vec<Vec<String>>) -> (i32, Vec<Vec<String>>) {
    let mut sum = 0;
    let mut legal_lines = Vec::new();

    for line in lines {
        match get_illegal_string(line).as_str() {
            ")" => sum += 3,
            "]" => sum += 57,
            "}" => sum += 1197,
            ">" => sum += 25137,
            _ => legal_lines.push(line.clone().to_vec())
        }
    }

    return (sum, legal_lines);
}

fn get_illegal_string(line: &Vec<String>) -> String {
    let mut stack: Vec<&String> = Vec::new();

    for char in line {
        if OPENING_CHARS.contains(&char.as_str()) {
            stack.push(char);
        } else {
            let opening_char = stack.pop().unwrap();
            let idx1 = OPENING_CHARS.iter().position(|&r| r == opening_char).unwrap();
            let idx2 = CLOSING_CHARS.iter().position(|&r| r == char).unwrap();

            if idx1 != idx2 {
                return char.to_string();
            }
        }
    }

    return "".to_string();
}

fn complete_lines(lines: Vec<Vec<String>>) -> i64 {
    let mut scores: Vec<i64> = Vec::new();

    for line in lines {
        scores.push(calc_line_completion_score(&line));
    }

    scores.sort();

    return scores[scores.len() / 2];
}

fn calc_line_completion_score(line: &Vec<String>) -> i64 {
    let mut score: i64 = 0;
    let mut stack: Vec<&String> = Vec::new();

    for char in line {
        if OPENING_CHARS.contains(&char.as_str()) {
            stack.push(char);
        } else {
            stack.pop();
        }
    }

    while !stack.is_empty() {
        let val = stack.pop().unwrap().as_str();
        match val {
            "(" => score = (score * 5) + 1,
            "[" => score = (score * 5) + 2,
            "{" => score = (score * 5) + 3,
            "<" => score = (score * 5) + 4,
            &_ => {}
        }
    }

    return score;
}


fn parse_input() -> Vec<Vec<String>> {
    split_on(&get_input_for_day(10), "\r\n").into_iter()
        .map(|v| split_on(&v, "").into_iter().filter(|v| v != &"").collect())
        .collect()
}