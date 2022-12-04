fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let assignments = input
        .lines()
        .map(|it| {
            it.split(",")
                .map(|it| {
                    it.split("-")
                        .map(|it| it.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let num_fully_contained = assignments
        .iter()
        .filter(|it| {
            let second_in_first = (it[1][0] >= it[0][0]) & (it[1][1] <= it[0][1]);
            let first_in_second = (it[1][0] <= it[0][0]) & (it[1][1] >= it[0][1]);
            second_in_first || first_in_second
        })
        .count();
    println!("{:?}", num_fully_contained);

    let num_overlaps = assignments
        .iter()
        .filter(|it| it[1][0].max(it[0][0]) <= it[0][1].min(it[1][1]))
        .count();
    println!("{:?}", num_overlaps);
}
