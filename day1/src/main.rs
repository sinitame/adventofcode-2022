use std::{fs, path::PathBuf};

fn main() {
    let input = fs::read_to_string(PathBuf::from("input.txt")).unwrap();
    let mut calories = input.lines().fold(vec![(1, 0)], |mut calories, line| {
        if let Ok(calory) = line.parse::<usize>() {
            let num_processed = calories.len();
            if num_processed > 0 {
                calories.get_mut(num_processed - 1).map(|it| it.1 += calory);
            }
        } else {
            calories.push((calories.len(), 0))
        }
        calories
    });

    calories.sort_by(|x, y| x.1.cmp(&y.1));
    println!(
        "{}",
        calories.iter().rev().take(3).map(|it| it.1).sum::<usize>()
    );
}
