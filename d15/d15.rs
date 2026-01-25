use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
struct Disc {
    id: u32,
    pos_count: u32,
    start_pos: u32,
}

impl FromStr for Disc {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        || -> Option<_> {
            lazy_static! {
                // language=regexp
                static ref PAT: Regex = Regex::new(
                    r#"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+)."#
                ).unwrap();
            }
            let (_, [id, pos_count, start_pos]) = PAT.captures(s)?.extract();
            Some(Disc {
                id: id.parse().ok()?,
                pos_count: pos_count.parse().ok()?,
                start_pos: start_pos.parse().ok()?,
            })
        }()
        .ok_or(())
    }
}

fn get_first_time(discs: &[Disc]) -> u32 {
    for i in 0.. {
        if discs
            .iter()
            .all(|d| (d.start_pos + d.id + i) % d.pos_count == 0)
        {
            return i;
        }
    }
    panic!("didn't work");
}

fn main() {
    // let input = include_str!("d15.sample.txt");
    let input = include_str!("d15.txt");
    let mut discs = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Disc>, _>>()
        .unwrap();
    discs.sort_by_key(|d| d.id);
    let t = get_first_time(&discs);
    println!("Part1: {}", t);

    discs.push(Disc {
        id: discs.len() as u32 + 1,
        pos_count: 11,
        start_pos: 0,
    });
    let t = get_first_time(&discs);
    println!("Part2: {}", t);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            "Disc #1 has 17 positions; at time=0, it is at position 15.".parse(),
            Ok(Disc {
                id: 1,
                pos_count: 17,
                start_pos: 15,
            })
        );
    }
}
