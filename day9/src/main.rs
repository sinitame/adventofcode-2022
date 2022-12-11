fn compute_next_position(
    cur_position: &(usize, usize),
    offsets: &mut (isize, isize),
) -> Option<(usize, usize)> {
    if (offsets.0.abs() < 2) & (offsets.1.abs() < 2) {
        None
    } else {
        let mut new_position = cur_position.clone();
        new_position = (
            (new_position.0 as isize + offsets.0.signum()) as usize,
            (new_position.1 as isize + offsets.1.signum()) as usize,
        );
        offsets.0 -= offsets.0.signum();
        offsets.1 -= offsets.1.signum();
        Some(new_position)
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut offsets = (0, 0);
    let cur_position = (0, 0);
    let mut positions = input.lines().fold(vec![cur_position], |mut positions, it| {
        let moves = it.split(" ").collect::<Vec<_>>();
        (0..moves[1].parse::<usize>().unwrap()).for_each(|_| {
            match moves[0] {
                "R" => offsets.1 += 1,
                "L" => offsets.1 -= 1,
                "U" => offsets.0 += 1,
                "D" => offsets.0 -= 1,
                _ => unreachable!(),
            }
            if let Some(new_position) =
                compute_next_position(positions.last().unwrap(), &mut offsets)
            {
                positions.push(new_position);
            }
        });
        positions
    });
    positions.sort();
    positions.dedup();
    println!("{}", positions.len());
}
