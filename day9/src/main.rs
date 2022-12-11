fn compute_next_position(
    cur_position: &(usize, usize),
    offsets: &mut (isize, isize),
) -> Option<((usize, usize), (isize, isize))> {
    if (offsets.0.abs() < 2) & (offsets.1.abs() < 2) {
        None
    } else {
        let mut new_position = cur_position.clone();
        new_position = (
            (new_position.0 as isize + offsets.0.signum()) as usize,
            (new_position.1 as isize + offsets.1.signum()) as usize,
        );
        let offset_change = (offsets.0.signum(), offsets.1.signum());
        offsets.0 -= offset_change.0;
        offsets.1 -= offset_change.1;
        Some((new_position, offset_change))
    }
}

fn compute_all_positions(input: &str, rope_size: usize) -> Vec<Vec<(usize, usize)>> {
    let mut offsets = vec![(0_isize, 0_isize); rope_size - 1];
    input.lines().fold(
        vec![vec![(0, 0)]; rope_size - 1],
        |mut all_positions, it| {
            let moves = it.split(" ").collect::<Vec<_>>();
            (0..moves[1].parse::<usize>().unwrap()).for_each(|_| {
                match moves[0] {
                    "R" => offsets[0].1 += 1,
                    "L" => offsets[0].1 -= 1,
                    "U" => offsets[0].0 += 1,
                    "D" => offsets[0].0 -= 1,
                    _ => unreachable!(),
                }

                all_positions.iter_mut().zip(offsets.iter_mut()).fold(
                    None,
                    |prev_offset: Option<(isize, isize)>, (positions, mut offset)| {
                        if let Some(prev_offset) = prev_offset {
                            offset.0 += prev_offset.0;
                            offset.1 += prev_offset.1;
                        }

                        if let Some((new_position, offset_change)) =
                            compute_next_position(positions.last().unwrap(), &mut offset)
                        {
                            positions.push(new_position);
                            return Some(offset_change);
                        }
                        None
                    },
                );
            });
            all_positions
        },
    )
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let rope_size = 10;
    let mut all_positions = compute_all_positions(&input, rope_size);

    let p1_positions = all_positions.get_mut(0).unwrap();
    p1_positions.sort();
    p1_positions.dedup();
    println!("{}", p1_positions.len());

    let p2_positions = all_positions.get_mut(rope_size - 2).unwrap();
    p2_positions.sort();
    p2_positions.dedup();
    println!("{}", p2_positions.len());
}
