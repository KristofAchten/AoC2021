use std::fs;

pub fn sonar_sweep() {
    let input = fs::read_to_string("src/input/input.txt").expect("Unable to read file");
    let nums: Vec<String> = input.split("\n").map(str::to_string).collect();

    let mut prev_num = 90000;
    let mut cnt = 0;
    for s_of_num in &nums {
        let num = s_of_num.trim().parse::<i32>().unwrap();
        if num > prev_num {
            cnt += 1;
        }
        prev_num = num;
    }

    prev_num = 90000;
    let mut triple_cnt = 0;


    let mut i = 2;
    while i < nums.len() {
        let n1 = nums[i].trim().parse::<i32>().unwrap();
        let n2 = nums[i - 1].trim().parse::<i32>().unwrap();
        let n3 = nums[i - 2].trim().parse::<i32>().unwrap();

        let new = n1 + n2 + n3;
        if new > prev_num {
            triple_cnt += 1
        }
        prev_num = new;
        i += 1
    }

    println!("Result part 1: {} - Result part 2: {}", cnt, triple_cnt)
}