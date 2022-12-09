fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = input
        .lines()
        .map(|it| {
            it.chars()
                .map(|it| it.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let mut visible_trees = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if (i == 0) || (j == 0) || (i == (grid.len() - 1)) || (j == (grid[0].len() - 1)) {
                visible_trees.push(true);
            } else {
                let visible_from_top = (0..i).all(|id| grid[id][j] < grid[i][j]);
                let visible_from_down = (i + 1..grid.len()).all(|id| grid[id][j] < grid[i][j]);
                let visible_from_left = (0..j).all(|id| grid[i][id] < grid[i][j]);
                let visible_from_right = (j + 1..grid[0].len()).all(|id| grid[i][id] < grid[i][j]);
                if visible_from_top || visible_from_down || visible_from_left || visible_from_right
                {
                    visible_trees.push(true)
                } else {
                    visible_trees.push(false)
                }
            }
        }
    }
    let num_visible_trees = visible_trees.iter().map(|it| *it as usize).sum::<usize>();
    println!("{}", num_visible_trees);

    let mut scenic_scores = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let (score_to_top, _) =
                (0..i)
                    .rev()
                    .fold((0, true), |(mut count, mut should_continue), id| {
                        if ((grid[id][j] <= grid[i][j]) || id == 0) & should_continue {
                            count += 1;
                            if grid[id][j] == grid[i][j] {
                                should_continue = false;
                            }
                        }
                        (count, should_continue)
                    });
            let (score_to_down, _) =
                ((i + 1)..grid.len()).fold((0, true), |(mut count, mut should_continue), id| {
                    if ((grid[id][j] <= grid[i][j]) || id == grid.len() - 1) & should_continue {
                        count += 1;
                        if grid[id][j] == grid[i][j] {
                            should_continue = false;
                        }
                    }
                    (count, should_continue)
                });
            let (score_to_left, _) =
                (0..j)
                    .rev()
                    .fold((0, true), |(mut count, mut should_continue), id| {
                        if ((grid[i][id] <= grid[i][j]) || id == 0) & should_continue {
                            count += 1;
                            if grid[i][id] == grid[i][j] {
                                should_continue = false;
                            }
                        }
                        (count, should_continue)
                    });
            let (score_to_right, _) =
                ((j + 1)..grid[0].len()).fold((0, true), |(mut count, mut should_continue), id| {
                    if ((grid[i][id] <= grid[i][j]) || id == grid[0].len() - 1) & should_continue {
                        count += 1;
                        if grid[i][id] == grid[i][j] {
                            should_continue = false;
                        }
                    }
                    (count, should_continue)
                });
            scenic_scores.push(score_to_top * score_to_left * score_to_down * score_to_right);
        }
    }

    println!("{}", scenic_scores.iter().max().unwrap());
}
