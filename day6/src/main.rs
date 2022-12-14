fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let characters = input.chars().collect::<Vec<_>>();
    let message_len_1 = 4;
    let decoder = |msg_len: usize, characters: &Vec<char>| {
        characters
            .windows(msg_len)
            .enumerate()
            .find(|(_idx, it)| {
                let mut set = vec![it[0]];
                let set_len = it
                    .iter()
                    .skip(1)
                    .map_while(|e| {
                        if !set.contains(e) {
                            set.push(*e);
                            Some(set.len())
                        } else {
                            None
                        }
                    })
                    .last()
                    .unwrap_or(0);
                set_len == msg_len
            })
            .map(|(idx, _)| msg_len + idx)
            .unwrap()
    };
    println!("{}", (decoder)(message_len_1, &characters));
    let message_len_2 = 14;
    println!("{}", (decoder)(message_len_2, &characters));
}
