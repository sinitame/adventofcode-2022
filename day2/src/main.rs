use std::fs;

fn main() {
    let input = fs::read_to_string("test.txt").unwrap();
    let score_1 = input
        .lines()
        .map(|it| {
            let vals = it.chars().map(|it| it as usize).collect::<Vec<_>>();
            let outcome = (vals[2] - vals[0]) % 3;
            let choice_point = vals[2] - 87;
            let round_score = if outcome == 0 {
                6 + choice_point
            } else if outcome == 2 {
                3 + choice_point
            } else {
                0 + choice_point
            };
            round_score as usize
        })
        .sum::<usize>();
    let score_2 = input
        .lines()
        .map(|it| {
            let vals = it.chars().map(|it| it as usize).collect::<Vec<_>>();
            let strategy = vals[2] - 88;
            let oponent_choice = vals[0] - 64;
            if strategy == 0 {
                (oponent_choice + 1) % 3 + 1
            } else if strategy == 1 {
                3 + oponent_choice
            } else {
                (oponent_choice + 1) % 3 + 6
            }
        })
        .sum::<usize>();
    println!("{:?}", score_1);
    println!("{:?}", score_2);
}
