use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Operation {
    SwapIndex(usize, usize),
    SwapChar(u8, u8),
    Rotate(Dir, usize),
    RotateChar(u8),
    /// Used for unscrambling
    RotateCharInverse(u8),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Operation {
    fn rev(self) -> Self {
        match self {
            Operation::SwapIndex(_, _) | Operation::SwapChar(_, _) | Operation::Reverse(_, _) => {
                self
            }
            Operation::Rotate(dir, by) => match dir {
                Dir::Left => Operation::Rotate(Dir::Right, by),
                Dir::Right => Operation::Rotate(Dir::Left, by),
            },
            Operation::RotateChar(c) => Operation::RotateCharInverse(c),
            Operation::RotateCharInverse(c) => Operation::RotateChar(c),
            Operation::Move(a, b) => Operation::Move(b, a),
        }
    }
}

fn parse_op(s: &str) -> Operation {
    lazy_static! {
        // language=regexp
        static ref PAT: Regex = Regex::new(
            r#"(?x)^(?:
                (?<si>swap\sposition\s(?<swapIndex0>\d+)\swith\sposition\s(?<swapIndex1>\d+))|
                (?<sc>swap\sletter\s(?<swapChar0>[a-z])\swith\sletter\s(?<swapChar1>[a-z]))|
                (?<r> rotate\s(?<rotateDir>left|right)\s(?<rotate0>\d+)\ssteps?)|
                (?<rc>rotate\sbased\son\sposition\sof\sletter\s(?<rotateChar>[a-z]))|
                (?<re>reverse\spositions\s(?<reverse0>\d+)\sthrough\s(?<reverse1>\d+))|
                (?<m> move\sposition\s(?<move0>\d+)\sto\sposition\s(?<move1>\d+))
            )$"#,
        )
        .unwrap();
    }
    let caps = PAT.captures(s).unwrap();
    if caps.name("si").is_some() {
        Operation::SwapIndex(
            caps["swapIndex0"].parse().unwrap(),
            caps["swapIndex1"].parse().unwrap(),
        )
    } else if caps.name("sc").is_some() {
        Operation::SwapChar(
            caps["swapChar0"].chars().next().unwrap() as u8,
            caps["swapChar1"].chars().next().unwrap() as u8,
        )
    } else if caps.name("r").is_some() {
        Operation::Rotate(
            if &caps["rotateDir"] == "left" {
                Dir::Left
            } else {
                Dir::Right
            },
            caps["rotate0"].parse().unwrap(),
        )
    } else if caps.name("rc").is_some() {
        Operation::RotateChar(caps["rotateChar"].chars().next().unwrap() as u8)
    } else if caps.name("re").is_some() {
        Operation::Reverse(
            caps["reverse0"].parse().unwrap(),
            caps["reverse1"].parse().unwrap(),
        )
    } else if caps.name("m").is_some() {
        Operation::Move(
            caps["move0"].parse().unwrap(),
            caps["move1"].parse().unwrap(),
        )
    } else {
        panic!();
    }
}

fn execute(mut s: Vec<u8>, op: Operation) -> Vec<u8> {
    let l = s.len();
    match op {
        Operation::SwapIndex(a, b) => s.swap(a, b),
        Operation::SwapChar(a, b) => s.iter_mut().for_each(|c| {
            if *c == a {
                *c = b
            } else if *c == b {
                *c = a
            }
        }),
        Operation::Rotate(dir, by) => match dir {
            Dir::Left => s.rotate_left(by % l),
            Dir::Right => s.rotate_right(by % l),
        },
        Operation::RotateChar(c) => {
            if let Some((i, _)) = s.iter().find_position(|x| **x == c) {
                let i = if i >= 4 { i + 2 } else { i + 1 };
                s.rotate_right(i % l)
            }
        }
        Operation::RotateCharInverse(c) => {
            s = (0..s.len())
                .filter_map(|off| {
                    let mut sc = s.clone();
                    sc.rotate_left(off);
                    if execute(sc.clone(), Operation::RotateChar(c)) == s {
                        Some(sc)
                    } else {
                        None
                    }
                })
                .next()
                .unwrap();
        }
        Operation::Reverse(a, b) => (&mut s)[a..=b].reverse(),
        Operation::Move(a, b) => {
            if a < b {
                (&mut s)[a..=b].rotate_left(1)
            } else {
                (&mut s)[b..=a].rotate_right(1)
            }
        }
    }
    s
}

fn main() {
    let input = include_str!("d21.txt");
    let operations: Vec<_> = input.lines().map(parse_op).collect();
    let result = operations
        .iter()
        .copied()
        .fold(b"abcdefgh".to_vec(), execute);
    println!("Part1: {}", unsafe { String::from_utf8_unchecked(result) });
    let result = operations
        .iter()
        .copied()
        .rev()
        .map(Operation::rev)
        .fold(b"fbgdceah".to_vec(), execute);
    println!("Part2: {}", unsafe { String::from_utf8_unchecked(result) });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_op("swap position 4 with position 0"),
            Operation::SwapIndex(4, 0)
        );
        assert_eq!(
            parse_op("swap letter d with letter b"),
            Operation::SwapChar(b'd', b'b')
        );
        assert_eq!(
            parse_op("reverse positions 0 through 4"),
            Operation::Reverse(0, 4)
        );
        assert_eq!(
            parse_op("rotate left 1 step"),
            Operation::Rotate(Dir::Left, 1)
        );
    }

    #[test]
    fn test_op() {
        let s = b"abcde".to_vec();
        let s = execute(s, parse_op("swap position 4 with position 0"));
        assert_eq!(s, b"ebcda");
        let s = execute(s, parse_op("swap letter d with letter b"));
        assert_eq!(s, b"edcba");
        let s = execute(s, parse_op("reverse positions 0 through 4"));
        assert_eq!(s, b"abcde");
        let s = execute(s, parse_op("rotate left 1 step"));
        assert_eq!(s, b"bcdea");
        let s = execute(s, parse_op("move position 1 to position 4"));
        assert_eq!(s, b"bdeac");
        let s = execute(s, parse_op("move position 3 to position 0"));
        assert_eq!(s, b"abdec");
        let s = execute(s, parse_op("rotate based on position of letter b"));
        assert_eq!(s, b"ecabd");
        let s = execute(s, parse_op("rotate based on position of letter d"));
        assert_eq!(s, b"decab");
    }

    #[test]
    fn test_inverse() {
        let s = b"decab".to_vec();
        let s = execute(s, parse_op("rotate based on position of letter d").rev());
        assert_eq!(s, b"ecabd");
        let s = execute(s, parse_op("rotate based on position of letter b").rev());
        assert_eq!(s, b"abdec");
        let s = execute(s, parse_op("move position 3 to position 0").rev());
        assert_eq!(s, b"bdeac");
    }
}
