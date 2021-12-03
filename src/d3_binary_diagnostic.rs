use crate::{get_input_for_day, split_on, to_int_32};

pub fn run_diagnostic() {
    let binary_nums: Vec<String> = split_on(&get_input_for_day(3), "\n");
    let mut values_vac: Vec<Vec<_>> = Vec::new();

    for num in &binary_nums {
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

    let gamma = isize::from_str_radix(&values_vac.iter()
        .map(|v| to_most_common_value(v))
        .collect::<Vec<String>>().join(""), 2).unwrap();
    let epsilon = isize::from_str_radix(&values_vac.iter()
        .map(|v| to_least_common_value(v))
        .collect::<Vec<String>>().join(""), 2).unwrap();

    println!("part 1 = {} ; part 2 = {}", gamma * epsilon, "Todo");
}

fn to_most_common_value(vec: &Vec<i32>) -> String {
    let more_zeros = vec.iter().filter(|v| v == &&to_int_32(&"0".to_string())).count() > 500;
    return if more_zeros { "0".to_string() } else { "1".to_string() }

}

fn to_least_common_value(vec: &Vec<i32>) -> String {
    let more_zeros = vec.iter().filter(|v| v == &&to_int_32(&"0".to_string())).count() > 500;
    return if more_zeros { "1".to_string() } else { "0".to_string() }
}