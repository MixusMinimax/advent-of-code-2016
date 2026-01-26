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
    Reverse(usize, usize),
    Move(usize, usize),
}

fn parse_op(s: &str) -> Operation {
    lazy_static! {
        // language=regexp
        static ref PAT: Regex = Regex::new(
            r#"(?x)^(?:
                (?<si>swap\sposition\s(?<swapIndex0>\d+)\swith\sposition\s(?<swapIndex1>\d+))|
                (?<sc>swap\sletter\s(?<swapChar0>[a-z])\swith\sletter\s(?<swapChar1>[a-z]))|
                (?<r> rotate\s(?<rotateDir>left|right)\s(?<rotate0>\d+)\ssteps)|
                (?<rc>rotate\sbased\son\sposition\sof\sletter\s(?<rotateChar>[a-z]))|
                (?<re>reverse\spositions\s(?<reverse0>\s+)\sthrough\s(?<reverse1>\s+))|
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

fn main() {}
