fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let items = input
        .lines()
        .map(|it| {
            it.chars()
                .map(|it| it as u8)
                .map(|it| it.checked_sub(96).unwrap_or_else(|| it - 38))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum_of_priorities_1 = items
        .iter()
        .map(|rucksack| {
            let rucksack_len = rucksack.len();
            *rucksack
                .iter()
                .take(rucksack_len / 2)
                .find(|it| rucksack[(rucksack_len / 2)..].contains(it))
                .unwrap() as usize
        })
        .sum::<usize>();
    println!("{}", sum_of_priorities_1);

    let sum_of_priorities_2 = items
        .chunks(3)
        .map(|rucksack_chunck| {
            *rucksack_chunck[0]
                .iter()
                .filter(|it| rucksack_chunck[1].contains(it))
                .find(|it| rucksack_chunck[2].contains(it))
                .unwrap() as usize
        })
        .sum::<usize>();
    println!("{}", sum_of_priorities_2);
}
