use std::collections::HashMap;
use std::time::Instant;

use crate::{get_input_for_day, split_on};

struct Cave {
    connections: Vec<String>,
    is_small: bool,
}

pub fn iterate_paths() {
    let now = Instant::now();
    let caves = parse_input();

    let res1 = count_all_paths(&caves, &"start".to_string(), &mut Vec::new());
    let res2 = count_all_paths_alternative(&caves, &"start".to_string(), &mut HashMap::new(), false);

    println!("part 1 = {} ; part 2 = {} (time: {}ms)", res1, res2, now.elapsed().as_millis());
}

fn count_all_paths(caves: &HashMap<String, Cave>, val: &String, visited: &mut Vec<String>) -> i64 {
    if val == "end" {
        return 1;
    }

    let cave = caves.get(val).unwrap();
    if cave.is_small && visited.contains(&val) {
        return 0;
    }

    visited.push(val.to_string());
    let mut sum_of_paths = 0;
    for connection in &cave.connections {
        sum_of_paths += count_all_paths(caves, connection, visited);
    }
    visited.retain(|x| x != val);

    return sum_of_paths;
}

fn count_all_paths_alternative(caves: &HashMap<String, Cave>, val: &String, visited: &mut HashMap<String, i32>, has_multi_visit: bool) -> i64 {
    if val == "end" {
        return 1;
    }

    let cave = caves.get(val).unwrap();
    let num_of_visits = visited.get(val).unwrap_or(&0).clone();
    let is_relevant_second_visit = cave.is_small && num_of_visits >= 1;

    if is_relevant_second_visit && (has_multi_visit || val == "start") {
        return 0;
    }

    visited.insert(val.to_string(), num_of_visits.clone() + 1);
    let mut sum_of_paths = 0;
    for connection in &cave.connections {
        sum_of_paths += count_all_paths_alternative(caves, connection, visited, has_multi_visit || is_relevant_second_visit);
    }
    visited.insert(val.to_string(), num_of_visits);

    return sum_of_paths;
}

fn parse_input() -> HashMap<String, Cave> {
    let lines = split_on(&get_input_for_day(12), "\r\n");
    let mut caves: HashMap<String, Cave> = HashMap::new();

    for line in &lines {
        let parts = split_on(&line, "-");

        for part in &parts {
            let mut connections: Vec<String> = get_connections(part, &lines);
            if caves.contains_key(part) {
                let cur_connections = &caves.get(part).unwrap().connections;
                for cur_connection in cur_connections {
                    if !connections.contains(cur_connection) {
                        connections.push(cur_connection.to_string());
                    }
                }
            }
            caves.insert(part.to_string(), Cave { connections, is_small: part.to_lowercase() == part.to_string() });
        }
    }

    return caves;
}

fn get_connections(val: &String, lines: &Vec<String>) -> Vec<String> {
    lines.into_iter()
        .map(|line| split_on(line, "-"))
        .filter(|line| line.contains(val))
        .flatten()
        .filter(|v| v != val)
        .collect()
}
