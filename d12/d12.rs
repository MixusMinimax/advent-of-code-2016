#![feature(assert_matches)]

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, i32, i64, space1};
use nom::combinator::{eof, map, value, verify};
use nom::{IResult, Parser};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

type Reg = char;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum RegOrLit {
    Reg(Reg),
    Lit(i64),
}

impl From<char> for RegOrLit {
    fn from(value: char) -> Self {
        RegOrLit::Reg(value)
    }
}

impl From<i64> for RegOrLit {
    fn from(value: i64) -> Self {
        RegOrLit::Lit(value)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Ins {
    /// Copy 0 into 1
    Cpy(RegOrLit, Reg),
    /// Increment the contents of register r by 1
    Inc(Reg),
    /// Decrement the contents of register r by 1
    Dec(Reg),
    /// Jump to offset o if register r is not 0
    Jnz(RegOrLit, i32),
}

impl fmt::Display for RegOrLit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RegOrLit::Reg(r) => write!(f, "{}", r),
            RegOrLit::Lit(r) => write!(f, "{}", r),
        }
    }
}

impl fmt::Display for Ins {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Ins::Cpy(a, b) => write!(f, "cpy {} {}", a, b),
            Ins::Inc(r) => write!(f, "inc {}", r),
            Ins::Dec(r) => write!(f, "dec {}", r),
            Ins::Jnz(r, o) => write!(f, "jnz {} {:+}", r, o),
        }
    }
}

impl FromStr for Ins {
    type Err = nom::Err<nom::error::Error<String>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn reg(s: &str) -> IResult<&str, char> {
            verify(anychar, |c| c.is_alphabetic()).parse(s)
        }

        fn reg_or_lit(s: &str) -> IResult<&str, RegOrLit> {
            alt((map(reg, RegOrLit::Reg), map(i64, RegOrLit::Lit))).parse(s)
        }

        alt((
            map(
                (tag("cpy"), space1, reg_or_lit, space1, reg),
                |(_, _, a, _, b)| Ins::Cpy(a, b),
            ),
            map(
                (
                    alt((
                        value(Ins::Inc as fn(Reg) -> Ins, tag("inc")),
                        value(Ins::Dec as fn(Reg) -> Ins, tag("dec")),
                    )),
                    space1,
                    reg,
                    eof,
                ),
                |(ins, _, r, _)| ins(r),
            ),
            map(
                (tag("jnz"), space1, reg_or_lit, space1, i32, eof),
                |(_, _, r, _, o, _)| Ins::Jnz(r, o),
            ),
        ))
        .parse(s)
        .map(|(_, o)| o)
        .map_err(<nom::Err<nom::error::Error<&str>>>::to_owned)
    }
}

fn parse_program(s: &str) -> Result<Vec<Ins>, nom::Err<nom::error::Error<String>>> {
    s.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::parse)
        .collect()
}

type Val = i64;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Cpu {
    program: Vec<Ins>,
    pc: i32,
    registers: HashMap<Reg, Val>,
}

impl FromStr for Cpu {
    type Err = nom::Err<nom::error::Error<String>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cpu {
            program: parse_program(s)?,
            pc: 0,
            registers: HashMap::new(),
        })
    }
}

impl Cpu {
    fn read(&self, r: Reg) -> Val {
        self.registers.get(&r).copied().unwrap_or_default()
    }

    fn write(&mut self, r: Reg, v: Val) {
        self.registers.insert(r, v);
    }
}

fn step(mut cpu: Cpu) -> (Cpu, bool) {
    if cpu.pc < 0 || cpu.pc >= cpu.program.len() as i32 {
        return (cpu, true);
    }
    let ins = cpu.program[cpu.pc as usize];
    cpu.pc += 1;
    match ins {
        Ins::Cpy(RegOrLit::Reg(a), b) => cpu.write(b, cpu.read(a)),
        Ins::Cpy(RegOrLit::Lit(a), b) => cpu.write(b, a),
        Ins::Inc(r) => cpu.write(r, cpu.read(r) + 1),
        Ins::Dec(r) => cpu.write(r, cpu.read(r) - 1),
        Ins::Jnz(r, o) => {
            if match r {
                RegOrLit::Reg(r) => cpu.read(r),
                RegOrLit::Lit(l) => l,
            } != 0
            {
                cpu.pc += o - 1
            }
        }
    }
    (cpu, false)
}

fn run(mut cpu: Cpu) -> Cpu {
    loop {
        let (cpu_next, halted) = step(cpu);
        if halted {
            break cpu_next;
        }
        cpu = cpu_next;
    }
}

fn main() {
    let cpu: Cpu = include_str!("input.asm").parse().unwrap();

    let result = run(cpu.clone());
    println!("Part1: a = {}", result.read('a'));

    let mut cpu = cpu;
    cpu.write('c', 1);
    let result = run(cpu);
    println!("Part2: a = {}", result.read('a'));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_parse() {
        assert_eq!("cpy 1 a".parse(), Ok(Ins::Cpy(RegOrLit::Lit(1), 'a')));
        assert_eq!("cpy x a".parse(), Ok(Ins::Cpy(RegOrLit::Reg('x'), 'a')));
        assert_eq!("inc c".parse(), Ok(Ins::Inc('c')));
        assert_eq!("dec c".parse(), Ok(Ins::Dec('c')));
        assert_eq!("jnz b -3".parse(), Ok(Ins::Jnz('b'.into(), -3)));

        assert_matches!("cpy 1".parse::<Ins>(), Err(_));
        assert_matches!("jnz abc".parse::<Ins>(), Err(_));
    }

    #[test]
    fn test_parse_program() {
        let program = "\
            cpy 41 a\n\
            inc a\n\
            inc a\n\
            dec a\n\
            jnz a 2\n\
            dec a\
        ";
        let expected = vec![
            Ins::Cpy(RegOrLit::Lit(41), 'a'),
            Ins::Inc('a'),
            Ins::Inc('a'),
            Ins::Dec('a'),
            Ins::Jnz('a'.into(), 2),
            Ins::Dec('a'),
        ];
        assert_eq!(parse_program(program), Ok(expected));
    }

    #[test]
    fn test_step() {
        let cpu = Cpu {
            program: vec![
                Ins::Cpy(RegOrLit::Lit(41), 'a'),
                Ins::Inc('a'),
                Ins::Inc('a'),
                Ins::Dec('a'),
                Ins::Jnz('a'.into(), 2),
                Ins::Dec('a'),
            ],
            ..Cpu::default()
        };

        let (cpu, halted) = step(cpu);
        assert!(!halted);
        assert_eq!(cpu.pc, 1);
        assert_eq!(cpu.read('a'), 41);
    }

    #[test]
    fn test_run() {
        let cpu = Cpu::from_str(
            r#"
                cpy 41 a
                inc a
                inc a
                dec a
                jnz a 2
                dec a
            "#,
        )
        .unwrap();
        let expected = Cpu {
            program: cpu.program.clone(),
            pc: 6,
            registers: HashMap::from([('a', 42)]),
        };
        assert_eq!(run(cpu), expected);
    }
}
