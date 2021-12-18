use std::collections::HashMap;
use std::time::Instant;

use crate::{get_input_for_day, split_on};

pub fn extend_polym() -> String {
    let now = Instant::now();
    let (template, rules) = parse_input();

    let res1 = polymax(&template, &rules, 10);
    let res2 = polymax(&template, &rules, 40);

    return format!("part 1 = {} ; part 2 = {} (time: {}ms)", res1, res2, now.elapsed().as_millis());
}

fn parse_input() -> (String, HashMap<String, char>) {
    let parts = split_on(&get_input_for_day(14), "\r\n\r\n");
    let template = &parts[0];

    let mut rules = HashMap::new();

    for rule in split_on(&parts[1], "\r\n") {
        let parts = split_on(&rule, " -> ");
        rules.insert(parts[0].to_string(), parts[1].chars().nth(0).unwrap());
    }

    return (template.to_string(), rules);
}

fn polymax(template: &String, rules: &HashMap<String, char>, iterations: i32) -> i64 {
    let mut pair_map: HashMap<String, i64> = HashMap::new();

    for i in 0..template.len() - 1 {
        let sub_str = template[i..i + 2].to_string();
        if !pair_map.contains_key(&sub_str) {
            pair_map.insert(sub_str, 1);
        } else {
            pair_map.insert(sub_str.clone(), pair_map[&sub_str] + 1);
        }
    }

    (0..iterations).for_each(|_| {
        let mut new_pair_map = HashMap::new();
        for key in pair_map.keys() {
            let to_insert = rules[key];
            let (c1, c2) = (key.chars().nth(0).unwrap(), key.chars().nth(1).unwrap());

            let pair_one: String = vec![c1, to_insert].into_iter().collect();
            let pair_two: String = vec![to_insert, c2].into_iter().collect();

            new_pair_map.insert(pair_one.clone(), new_pair_map.get(&pair_one).or_else(|| Some(&0)).unwrap() + pair_map.get(key).unwrap());
            new_pair_map.insert(pair_two.clone(), new_pair_map.get(&pair_two).or_else(|| Some(&0)).unwrap() + pair_map.get(key).unwrap());
        }

        pair_map = new_pair_map;
    });

    let mut char_to_cnt: HashMap<char, i64> = HashMap::new();
    for (str, val) in &pair_map {
        let c = str.chars().nth(0).unwrap();
        char_to_cnt.insert(c, char_to_cnt.get(&c).or_else(|| Some(&0)).unwrap() + val.clone());
    }

    let last_char = template.chars().last().unwrap();
    char_to_cnt.insert(last_char, char_to_cnt.get(&last_char).or_else(|| Some(&0)).unwrap() + 1);

    return char_to_cnt.values().max().unwrap() - char_to_cnt.values().min().unwrap();
}