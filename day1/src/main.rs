use std::{fs, path::PathBuf};

fn main() {
    let input = fs::read_to_string(PathBuf::from("input.txt")).unwrap();
    let calories = input.lines().fold(vec![0], |mut calories, line| {
        if let Ok(calory) = line.parse::<usize>() {
            let num_processed = calories.len();
            if num_processed > 0 {
                calories.get_mut(num_processed - 1).map(|it| *it += calory);
            }
        } else {
            calories.push(0)
        }
        calories
    });
    let elf_with_max_cal = calories
        .iter()
        .enumerate()
        .max_by(|x, y| x.1.cmp(y.1))
        .unwrap();
    println!("{}", elf_with_max_cal.1);
}
