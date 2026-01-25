use aoc2016::graph::a_star_rev;
use md5::{Digest, Md5};
use nom::FindSubstring;
use vecmath::{vec2_add, vec2_sub};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
struct State {
    pos: [i32; 2],
    path_taken: String,
}

fn get_shortest_path(input: &str) -> State {
    a_star_rev(
        &State {
            pos: [0, 0],
            path_taken: String::new(),
        },
        |s| s.pos == [3, 3],
        |s| {
            let hash =
                base16ct::lower::encode_string(&Md5::digest(format!("{}{}", input, s.path_taken)));
            ['U', 'D', 'L', 'R']
                .into_iter()
                .enumerate()
                .filter(|&(index, _)| "abcdef".find_substring(&hash[index..index + 1]).is_some())
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
                    (s, d)
                })
                .collect::<Vec<_>>()
        },
        |s| {
            let v = vec2_sub(s.pos, [3, 3]);
            (v[0].abs() + v[1].abs()) as i64
        },
        |_, _, _| 1,
    )
    .unwrap()
    .1
}

fn main() {
    let p = get_shortest_path("qtetzkpl");
    println!("{p:?}");
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
}
