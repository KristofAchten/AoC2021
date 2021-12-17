use std::time::Instant;

use crate::{bin_string_to_dec, get_input_for_day};

pub fn decode_input() {
    let now = Instant::now();

    let input = get_input_for_day(16);
    let binary = to_binary(input);

    let (res1, _) = sum_versions(&binary);

    println!("part 1 = {} ; part 2 = {} (time: {}ms)", res1, "TODO", now.elapsed().as_millis());
}

fn sum_versions(input: &String) -> (isize, String) {
    let version = bin_string_to_dec(&input[0..3]);
    let id = bin_string_to_dec(&input[3..6]);

    let mut remaining_bits = &input[6..];
    return if id == 4 {
        let mut should_continue = true;
        while should_continue {
            should_continue = remaining_bits.chars().nth(0).unwrap() == '1';
            remaining_bits = &remaining_bits[5..];
        }
        (version, remaining_bits.clone().to_string())
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