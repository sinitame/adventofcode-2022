use std::{fs, path::PathBuf};

fn main() {
    let input = fs::read_to_string(PathBuf::from("input.txt")).unwrap();

    // Get the total of calories per elf
    let mut calories = input.lines().fold(vec![(1, 0)], |mut calories, line| {
        let num_processed = calories.len();
        if let Ok(calory) = line.parse::<usize>() {
            if num_processed > 0 {
                calories.get_mut(num_processed - 1).map(|it| it.1 += calory);
            }
        } else {
            calories.push((num_processed, 0))
        }
        calories
    });

    // Sort the list of calories per calory amount
    calories.sort_by(|x, y| x.1.cmp(&y.1));

    // Print the sum of top3 calories amout
    println!(
        "{}",
        calories.iter().rev().take(3).map(|it| it.1).sum::<usize>()
    );
}
