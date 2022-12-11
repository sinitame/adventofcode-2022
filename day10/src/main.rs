fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut last_value = 1;
    let cycles = input
        .lines()
        .map(|it| {
            let instruction = it.split(" ").collect::<Vec<_>>();
            if instruction[0] == "addx" {
                let cycle_values = vec![last_value; 2];
                last_value += instruction[1].parse::<isize>().unwrap();
                cycle_values
            } else {
                vec![last_value]
            }
        })
        .flatten()
        .collect::<Vec<isize>>();

    let signal_strength = [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|idx| cycles[idx - 1] * *idx as isize)
        .sum::<isize>();
    println!("{}", signal_strength);

    let decoded_signal = cycles
        .iter()
        .enumerate()
        .map(|(idx, value)| {
            if (value - (idx % 40) as isize).abs() < 2 {
                "#"
            } else {
                "."
            }
        })
        .collect::<Vec<_>>();

    decoded_signal.chunks(40).for_each(|it| {
        println!("{}", it.join(""));
    })
}
