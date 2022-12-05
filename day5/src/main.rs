fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let clean_stack_values = input
        .lines()
        .map_while(|it| {
            let vals = it.chars().collect::<Vec<_>>();
            if vals[1] != '1' {
                let int = vals.chunks(4).fold(Vec::new(), |mut clean_line, it| {
                    let val = if it[1] != ' ' { Some(it[1]) } else { None };
                    clean_line.push(val);
                    clean_line
                });
                Some(int)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let reversed_intructions = input
        .lines()
        .rev()
        .map_while(|it| {
            let instruction = it.split(" ").collect::<Vec<_>>();
            if instruction[0] == "move" {
                Some((
                    instruction[1].parse::<usize>().unwrap(),
                    instruction[3].parse::<usize>().unwrap(),
                    instruction[5].parse::<usize>().unwrap(),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut stacks = vec![Vec::new(); clean_stack_values[0].len()];
    clean_stack_values.iter().rev().for_each(|it| {
        it.iter().enumerate().for_each(|(idx, val)| {
            if let Some(value) = val {
                stacks[idx].push(value);
            }
        })
    });

    let mut stacks_1 = stacks.clone();
    for &(num, from, to) in reversed_intructions.iter().rev() {
        for _ in 0..num {
            let element = stacks_1[from - 1].pop().unwrap();
            stacks_1[to - 1].push(element);
        }
    }

    println!(
        "{:?}",
        stacks_1
            .iter()
            .map(|it| *it.last().unwrap())
            .collect::<String>()
    );

    for &(num, from, to) in reversed_intructions.iter().rev() {
        let num_elements = stacks[from - 1].len();
        let (begin, end) = stacks[from - 1].split_at(num_elements - num);
        let (begin_vec, end_vec) = (begin.to_vec(), end.to_vec());
        stacks[to - 1].extend(end_vec);
        stacks[from - 1] = begin_vec;
    }
    println!(
        "{:?}",
        stacks
            .iter()
            .map(|it| *it.last().unwrap())
            .collect::<String>()
    );
}
