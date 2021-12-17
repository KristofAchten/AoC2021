use std::time::Instant;

use crate::{bin_string_to_dec, get_input_for_day};

pub fn decode_input() {
    let now = Instant::now();

    let input = get_input_for_day(16);
    let binary = to_binary(input);

    let (res1, _) = sum_versions(&binary);
    let (res2, _) = evaluate(&binary);

    println!("part 1 = {} ; part 2 = {} (time: {}ms)", res1, res2, now.elapsed().as_millis());
}

fn sum_versions(input: &String) -> (isize, String) {
    let version = bin_string_to_dec(&input[0..3]);
    let id = bin_string_to_dec(&input[3..6]);

    let mut remaining_bits = &input[6..];
    return if id == 4 {
        let (_, rest) = parse_literal(&remaining_bits);
        (version, rest.clone().to_string())
    } else {
        let mut version_sum = version;
        let op_thing = remaining_bits.chars().nth(0).unwrap();
        remaining_bits = &remaining_bits[1..];

        if op_thing == '0' {
            let length = bin_string_to_dec(&remaining_bits[..15]) as usize;
            remaining_bits = &remaining_bits[15..];

            let mut to_evaluate: String = remaining_bits[..length].to_string();
            remaining_bits = &remaining_bits[length..];

            while !to_evaluate.is_empty() {
                let (ver, rest) = sum_versions(&to_evaluate.to_string());
                version_sum += ver;
                to_evaluate = rest;
            }
            (version_sum, remaining_bits.to_string())
        } else {
            let num_of_subpackets = bin_string_to_dec(&remaining_bits[..11]);
            remaining_bits = &remaining_bits[11..];

            let mut to_evaluate: String = remaining_bits.clone().to_string();

            for _ in 0..num_of_subpackets {
                let (ver, rest) = sum_versions(&to_evaluate);
                version_sum += ver;
                to_evaluate = rest;
            }

            (version_sum, to_evaluate.to_string())
        }
    };
}

fn evaluate(input: &String) -> (isize, String) {
    let id = bin_string_to_dec(&input[3..6]);

    let remaining_bits = &input[6..];
    match id {
        0 => { return parse_op(&remaining_bits, &|v: Vec<isize>| v.into_iter().sum::<isize>() as isize); }
        1 => { return parse_op(&remaining_bits, &|v: Vec<isize>| v.into_iter().product::<isize>() as isize); }
        2 => { return parse_op(&remaining_bits, &|v: Vec<isize>| v.into_iter().min().unwrap()); }
        3 => { return parse_op(&remaining_bits, &|v: Vec<isize>| v.into_iter().max().unwrap()); }
        4 => { return parse_literal(&remaining_bits); }
        5 => { return parse_op(&remaining_bits, &|v: Vec<isize>| return if v[0] > v[1] { 1 } else { 0 }); }
        6 => { return parse_op(&remaining_bits, &|v: Vec<isize>| return if v[0] < v[1] { 1 } else { 0 }); }
        7 => { return parse_op(&remaining_bits, &|v: Vec<isize>| return if v[0] == v[1] { 1 } else { 0 }); }
        _ => { panic!() }
    }
}

fn parse_literal(input: &str) -> (isize, String) {
    let mut remaining_bits = input;
    let mut should_continue = true;
    let mut val_string = "".to_string();

    while should_continue {
        should_continue = remaining_bits.chars().nth(0).unwrap() == '1';
        val_string += &remaining_bits[1..5].to_string();
        remaining_bits = &remaining_bits[5..];
    }

    return (bin_string_to_dec(&val_string) as isize, remaining_bits.to_string());
}

fn parse_op(input: &str, f: &dyn Fn(Vec<isize>) -> isize) -> (isize, String) {
    let mut remaining_bits = input.clone();

    let op = remaining_bits.chars().nth(0).unwrap();
    remaining_bits = &remaining_bits[1..];

    let (results, rest) = evaluate_op(op, &remaining_bits);

    return (f(results), rest.to_string());
}

fn evaluate_op(op: char, input: &str) -> (Vec<isize>, String) {
    let mut remaining_bits = input.clone();
    let mut results: Vec<isize> = Vec::new();

    if op == '0' {
        let length = bin_string_to_dec(&remaining_bits[..15]) as usize;
        remaining_bits = &remaining_bits[15..];

        let mut to_evaluate: String = remaining_bits[..length].to_string();
        remaining_bits = &remaining_bits[length..];

        while !to_evaluate.is_empty() {
            let (val, rest) = evaluate(&to_evaluate.to_string());
            results.push(val);
            to_evaluate = rest;
        }

        (results, remaining_bits.to_string())
    } else {
        let num_of_subpackets = bin_string_to_dec(&remaining_bits[..11]);
        remaining_bits = &remaining_bits[11..];

        let mut to_evaluate: String = remaining_bits.clone().to_string();

        for _ in 0..num_of_subpackets {
            let (val, rest) = evaluate(&to_evaluate);
            results.push(val);
            to_evaluate = rest;
        }

        (results, to_evaluate.to_string())
    }
}

fn to_binary(input: String) -> String {
    input.chars().map(|c| char_to_binary(c)).collect()
}

fn char_to_binary(c: char) -> String {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }.to_string()
}