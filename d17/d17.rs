use aoc2016::graph::bfs;
use md5::{Digest, Md5};
use nom::FindSubstring;
use vecmath::vec2_add;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
struct State {
    pos: [i32; 2],
    path_taken: String,
}

fn get_all_paths(input: &str) -> impl IntoIterator<Item = State> {
    bfs(
        State {
            pos: [0, 0],
            path_taken: String::new(),
        },
        |s| s.pos == [3, 3],
        move |s| {
            let hash =
                base16ct::lower::encode_string(&Md5::digest(format!("{}{}", input, s.path_taken)));
            ['U', 'D', 'L', 'R']
                .into_iter()
                .enumerate()
                .filter(|&(index, _)| "bcdef".find_substring(&hash[index..index + 1]).is_some())
                .map(|(_, d)| d)
                .filter(|&d| match d {
                    'U' => s.pos[1] != 0,
                    'D' => s.pos[1] != 3,
                    'L' => s.pos[0] != 0,
                    'R' => s.pos[0] != 3,
                    _ => unreachable!(),
                })
                .map(|d| {
                    let mut s = s.clone();
                    s.path_taken.push(d);
                    s.pos = vec2_add(
                        s.pos,
                        match d {
                            'U' => [0, -1],
                            'D' => [0, 1],
                            'L' => [-1, 0],
                            'R' => [1, 0],
                            _ => unreachable!(),
                        },
                    );
                    s
                })
                .collect::<Vec<_>>()
        },
    )
}

fn get_shortest_path(input: &str) -> State {
    get_all_paths(input).into_iter().next().unwrap()
}

fn get_longest_path(input: &str) -> State {
    get_all_paths(input)
        .into_iter()
        .max_by_key(|s| s.path_taken.len())
        .unwrap()
}

fn main() {
    let input = "qtetzkpl";

    let p = get_shortest_path(input);
    println!("Part1: {p:?}");

    let p = get_longest_path(input);
    println!("Part2: {}", p.path_taken.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_shortest_path() {
        assert_eq!(get_shortest_path("ihgpwlah").path_taken, "DDRRRD");
        assert_eq!(get_shortest_path("kglvqrro").path_taken, "DDUDRLRRUDRD");
        assert_eq!(
            get_shortest_path("ulqzkmiv").path_taken,
            "DRURDRUDDLLDLUURRDULRLDUUDDDRR"
        );
    }

    #[test]
    fn test_get_longest_path() {
        assert_eq!(get_longest_path("ihgpwlah").path_taken.len(), 370);
        assert_eq!(get_longest_path("kglvqrro").path_taken.len(), 492);
        assert_eq!(get_longest_path("ulqzkmiv").path_taken.len(), 830);
    }
}
