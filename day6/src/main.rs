fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let characters = input.chars().collect::<Vec<_>>();
    let message_len_1 = 4;
    let decoder = |msg_len: usize, characters: &Vec<char>| {
        characters
            .windows(msg_len)
            .enumerate()
            .find(|(_idx, it)| {
                let set_len = it
                    .iter()
                    .skip(1)
                    .fold(vec![it[0]], |mut set, e| {
                        if !set.contains(e) {
                            set.push(*e);
                        }
                        set
                    })
                    .len();
                set_len == msg_len
            })
            .map(|(idx, _)| msg_len + idx)
            .unwrap()
    };
    println!("{}", (decoder)(message_len_1, &characters));
    let message_len_2 = 14;
    println!("{}", (decoder)(message_len_2, &characters));
}
