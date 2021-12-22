use std::time::Instant;

use crate::{get_input_for_day, to_int_32};

struct DeterministicDice {
    value: i64,
    num_of_rolls: i64,
}

impl DeterministicDice {
    fn roll(&mut self) -> i64 {
        let res = self.value.clone();
        self.value = self.value % 100 + 1;
        self.num_of_rolls += 1;
        return res;
    }
}

pub fn roll_dice() -> String {
    let now = Instant::now();

    let input = parse_input();

    let res1 = roll_deterministic_die(&input);
    let res2 = solve(input.0, input.1, 0, 0);

    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", res1, res2.0.max(res2.1), now.elapsed().as_millis());
}

fn roll_deterministic_die((p1, p2): &(i64, i64)) -> i64 {
    let mut pos = (p1 - 1, p2 - 1);
    let mut score = (0, 0);
    let mut die = DeterministicDice { value: 1, num_of_rolls: 0 };

    loop {
        let move_p1 = die.roll() + die.roll() + die.roll();
        let new_pos_p1 = (pos.0 + move_p1) % 10;
        pos.0 = new_pos_p1;
        score.0 += new_pos_p1 + 1;

        if score.0 >= 1000 {
            break;
        }

        let move_p2 = die.roll() + die.roll() + die.roll();
        let new_pos_p2 = (pos.1 + move_p2) % 10;
        pos.1 = new_pos_p2;
        score.1 += new_pos_p2 + 1;

        if score.1 >= 1000 {
            break;
        }
    }

    return die.num_of_rolls * score.0.min(score.1);
}

fn solve(p1: i64, p2: i64, s1: i64, s2: i64) -> (i64, i64) {
    if s1 >= 21 {
        return (1, 0);
    } else if s2 >= 21 {
        return (0, 1);
    }

    let mut total_wins = (0, 0);
    for (roll, times) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let pos = (p1 + roll - 1) % 10 + 1;
        let score = s1 + pos;

        let (p2_wins, p1_wins) = solve(p2, pos, s2, score);
        total_wins.0 += p1_wins * times;
        total_wins.1 += p2_wins * times;
    }

    return total_wins;
}

fn parse_input() -> (i64, i64) {
    let input: Vec<i64> = get_input_for_day(21).lines()
        .map(|s| to_int_32(&s[28..].to_string()) as i64)
        .collect();

    return (input[0], input[1]);
}