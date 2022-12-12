#[derive(Default)]
pub struct Monkey {
    pub id: usize,
    pub op: Option<Box<dyn Fn(usize) -> usize>>,
    pub test: Option<Box<dyn Fn(usize) -> bool>>,
    pub outcomes: [usize; 2],
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (monkeys, mut states) = input.lines().enumerate().fold(
        (Vec::new(), Vec::new()),
        |(mut monkeys, mut states), (idx, it)| {
            if idx % 7 == 0 {
                let monkey = it.split(" ").collect::<Vec<_>>();
                let id = monkey[1]
                    .split(":")
                    .find_map(|it| it.parse::<usize>().ok())
                    .unwrap();
                monkeys.push(Monkey {
                    id,
                    ..Default::default()
                })
            }

            if idx % 7 == 1 {
                let items = it
                    .split(": ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .split(", ")
                    .map(|it| it.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                states.push(items);
            }

            if idx % 7 == 2 {
                let operation = it
                    .split("= ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .split(" ")
                    .collect::<Vec<_>>();
                let op: Box<dyn Fn(usize) -> usize> = if operation[1] == "*" {
                    match (operation[0], operation[2]) {
                        ("old", "old") => Box::new(|x: usize| -> usize { x * x }),
                        ("old", op2) => {
                            let val2 = op2.parse::<usize>().unwrap();
                            Box::new(move |x: usize| -> usize { x * val2 })
                        }
                        (op1, "old") => {
                            let val1 = op1.parse::<usize>().unwrap();
                            Box::new(move |x: usize| -> usize { x * val1 })
                        }
                        (op1, op2) => {
                            let (val1, val2) =
                                (op1.parse::<usize>().unwrap(), op2.parse::<usize>().unwrap());
                            Box::new(move |_x: usize| -> usize { val1 * val2 })
                        }
                    }
                } else if operation[1] == "+" {
                    match (operation[0], operation[2]) {
                        ("old", "old") => Box::new(|x: usize| -> usize { x + x }),
                        ("old", op2) => {
                            let val2 = op2.parse::<usize>().unwrap();
                            Box::new(move |x: usize| -> usize { x + val2 })
                        }
                        (op1, "old") => {
                            let val1 = op1.parse::<usize>().unwrap();
                            Box::new(move |x: usize| -> usize { x + val1 })
                        }
                        (op1, op2) => {
                            let (val1, val2) =
                                (op1.parse::<usize>().unwrap(), op2.parse::<usize>().unwrap());
                            Box::new(move |_x: usize| -> usize { val1 + val2 })
                        }
                    }
                } else {
                    unreachable!()
                };
                monkeys.last_mut().map(|it| it.op = Some(op));
            }

            if idx % 7 == 3 {
                let div_number = it.split(" ").last().unwrap().parse::<usize>().unwrap();
                monkeys.last_mut().map(|it| {
                    it.test = Some(Box::new(move |x: usize| -> bool { x % div_number == 0 }))
                });
            }

            if idx % 7 == 4 {
                let outcome_true = it.split(" ").last().unwrap().parse::<usize>().unwrap();
                monkeys.last_mut().map(|it| it.outcomes[1] = outcome_true);
            }
            if idx % 7 == 5 {
                let outcome_true = it.split(" ").last().unwrap().parse::<usize>().unwrap();
                monkeys.last_mut().map(|it| it.outcomes[0] = outcome_true);
            }
            (monkeys, states)
        },
    );

    let round = 20;
    let mut scores = vec![0; monkeys.len()];
    for _ in 0..round {
        for (idx, monkey) in monkeys.iter().enumerate() {
            while let Some(item) = states[idx].pop() {
                scores[idx] += 1;
                let res = (monkey.op.as_ref().unwrap())(item);
                let res = res / 3;
                let next_monkey = monkey.outcomes[(monkey.test.as_ref().unwrap())(res) as usize];
                states[next_monkey].push(res);
            }
        }
    }

    scores.sort();
    scores.reverse();
    println!("{}", scores[0] * scores[1]);
}
