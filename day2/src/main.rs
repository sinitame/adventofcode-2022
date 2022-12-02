use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let score = input
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
    println!("{:?}", score);
}
