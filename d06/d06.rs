use aoc2016::AlphabetMap;

enum Mode {
    MostCommon,
    LeastCommon,
}

fn correct_error<'s>(words: impl IntoIterator<Item = &'s str>, mode: Mode) -> String {
    let mut initialized = false;
    let mut letter_counts = Vec::new();
    for word in words {
        let word = word.trim();
        if word.is_empty() {
            continue;
        }
        if !initialized {
            letter_counts = vec![AlphabetMap::<usize>::new(); word.len()];
            initialized = true;
        } else if letter_counts.len() != word.len() {
            panic!("encountered words of unequal length");
        }

        for (i, c) in word.chars().enumerate() {
            letter_counts[i][c] += 1;
        }
    }

    let key_fn: fn(&_) -> _ = match mode {
        Mode::MostCommon => |&(ch, co)| (usize::MAX - co, ch),
        Mode::LeastCommon => |&(ch, co)| (if co == 0 { usize::MAX } else { co }, ch),
    };

    letter_counts
        .into_iter()
        .map(|map| map.into_iter().min_by_key(key_fn).unwrap().0)
        .collect()
}

fn main() {
    // let input = include_str!("d06.sample.txt");
    let input = include_str!("d06.txt");
    let message = correct_error(input.lines(), Mode::MostCommon);
    println!("Part1: {}", message);

    let message = correct_error(input.lines(), Mode::LeastCommon);
    println!("Part2: {}", message);
}
