use std::time::Instant;

use crate::{get_input_for_day, split_on, to_int_32};

pub fn run_diagnostic() -> String {
    let now = Instant::now();

    let binary_nums: Vec<String> = split_on(&get_input_for_day(3), "\n");
    let values_vac = transpose(&binary_nums);

    let mcv = to_most_common_value(&values_vac, binary_nums.len());

    let lcv = to_least_common_value(&values_vac, binary_nums.len());

    let gamma = isize::from_str_radix(&mcv.join(""), 2).unwrap();
    let epsilon = isize::from_str_radix(&lcv.join(""), 2).unwrap();

    let mut i = 0;

    let mut oxygen_data = binary_nums.clone();
    let mut co2_data = oxygen_data.clone();

    while oxygen_data.len() > 1 {
        let transposed_data = transpose(&oxygen_data);
        let mcv = to_most_common_value(&transposed_data, oxygen_data.len());
        let mcv_val = &mcv[i];
        oxygen_data = oxygen_data.into_iter()
            .filter(|v| &v.chars().nth(i).unwrap().to_string() == mcv_val)
            .collect();
        i += 1;
    }

    i = 0;
    while co2_data.len() > 1 {
        let transposed_data = transpose(&co2_data);
        let mcv = to_least_common_value(&transposed_data, co2_data.len());
        let mcv_val = &mcv[i];
        co2_data = co2_data.into_iter()
            .filter(|v| &v.chars().nth(i).unwrap().to_string() == mcv_val)
            .collect();
        i += 1;
    }

    let oxygen_binary = &mut oxygen_data[0];
    let co2_binary = &mut co2_data[0];

    oxygen_binary.remove(oxygen_binary.len() - 1);
    co2_binary.remove(co2_binary.len() - 1);

    let oxygen = isize::from_str_radix(&oxygen_binary, 2).unwrap();
    let co2 = isize::from_str_radix(&co2_binary, 2).unwrap();

    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", gamma * epsilon, oxygen * co2, now.elapsed().as_millis());
}

fn to_most_common_value(values_vac: &Vec<Vec<i32>>, size: usize) -> Vec<String> {
    values_vac.iter()
        .map(|v| most_common_value(v, size))
        .collect::<Vec<String>>()
}

fn to_least_common_value(values_vac: &Vec<Vec<i32>>, size: usize) -> Vec<String> {
    values_vac.iter()
        .map(|v| least_common_value(v, size))
        .collect::<Vec<String>>()
}

fn transpose(binary_nums: &Vec<String>) -> Vec<Vec<i32>> {
    let mut values_vac: Vec<Vec<_>> = Vec::new();

    for num in binary_nums {
        let vals: Vec<i32> = split_on(num.trim(), "").iter()
            .filter(|v| v != &"")
            .map(|v| to_int_32(&v))
            .collect();


        for i in 0..vals.len() {
            let init: bool = values_vac.len() <= i;
            let val: i32 = vals[i];
            if init {
                values_vac.push(vec![val]);
            } else {
                values_vac[i].push(val);
            }
        }
    }
    values_vac
}

fn most_common_value(vec: &Vec<i32>, size: usize) -> String {
    let more_zeros = vec.into_iter().filter(|v| v == &&to_int_32(&"0".to_string())).count() > size / 2;
    return if more_zeros { "0".to_string() } else { "1".to_string() };

}

fn least_common_value(vec: &Vec<i32>, size: usize) -> String {
    let more_zeros = vec.iter().filter(|v| v == &&to_int_32(&"0".to_string())).count() > size / 2;
    return if more_zeros { "1".to_string() } else { "0".to_string() };
}