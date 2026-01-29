#![feature(assert_matches)]

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, i64, space0, space1};
use nom::combinator::{map, opt, recognize, rest, value, verify};
use nom::{IResult, Parser};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

type Reg = char;
type Val = i64;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum RegOrLit {
    Reg(Reg),
    Lit(Val),
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
    Cpy(RegOrLit, RegOrLit),
    /// Increment the contents of register r by 1
    Inc(Reg),
    /// Decrement the contents of register r by 1
    Dec(Reg),
    /// Jump to offset o if register r is not 0
    Jnz(RegOrLit, RegOrLit),
    /// Toggle instruction at offset
    Tgl(Reg),
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
            Ins::Tgl(r) => write!(f, "tgl {}", r),
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

        fn cmt(s: &str) -> IResult<&str, &str> {
            (space0, recognize(opt((tag(";"), rest))))
                .parse(s)
                .map(|(_, c)| c)
        }

        alt((
            map(
                (tag("cpy"), space1, reg_or_lit, space1, reg_or_lit, cmt),
                |(_, _, a, _, b, _)| Ins::Cpy(a, b),
            ),
            map(
                (
                    alt((
                        value(Ins::Inc as fn(Reg) -> Ins, tag("inc")),
                        value(Ins::Dec as fn(Reg) -> Ins, tag("dec")),
                        value(Ins::Tgl as fn(Reg) -> Ins, tag("tgl")),
                    )),
                    space1,
                    reg,
                    cmt,
                ),
                |(ins, _, r, _)| ins(r),
            ),
            map(
                (tag("jnz"), space1, reg_or_lit, space1, reg_or_lit, cmt),
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
        Ins::Cpy(RegOrLit::Reg(a), RegOrLit::Reg(b)) => cpu.write(b, cpu.read(a)),
        Ins::Cpy(RegOrLit::Lit(a), RegOrLit::Reg(b)) => cpu.write(b, a),
        Ins::Cpy(_, RegOrLit::Lit(_)) => {}
        Ins::Inc(r) => cpu.write(r, cpu.read(r) + 1),
        Ins::Dec(r) => cpu.write(r, cpu.read(r) - 1),
        Ins::Jnz(r, o) => {
            let o = match o {
                RegOrLit::Reg(r) => cpu.read(r),
                RegOrLit::Lit(l) => l,
            };
            if match r {
                RegOrLit::Reg(r) => cpu.read(r),
                RegOrLit::Lit(l) => l,
            } != 0
            {
                cpu.pc += (o - 1) as i32
            }
        }
        Ins::Tgl(r) => {
            let o = cpu.read(r);
            let i = cpu.pc as usize + o as usize - 1;
            if i < cpu.program.len() {
                let ins = &mut cpu.program[cpu.pc as usize + o as usize - 1];
                *ins = match *ins {
                    Ins::Cpy(a, b) => Ins::Jnz(a, b),
                    Ins::Inc(r) => Ins::Dec(r),
                    Ins::Dec(r) => Ins::Inc(r),
                    Ins::Jnz(a, b) => Ins::Cpy(a, b),
                    Ins::Tgl(r) => Ins::Inc(r),
                };
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
        // println!("{}", cpu_next.read('a'));
        cpu_next
            .registers
            .iter()
            .filter(|(_, v)| **v < 0)
            .for_each(|(r, v)| {
                println!("{r}={v}");
            });

        cpu = cpu_next;
    }
}

fn main() {
    let mut cpu: Cpu = include_str!("input.asm").parse().unwrap();

    // cpu.write('a', 7);
    // let result = run(cpu.clone());
    // println!("Part1: a = {}", result.read('a'));

    cpu.write('a', 12);
    let result = run(cpu);
    println!("Part2: a = {}", result.read('a'));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tgl_dec_inc() {
        let cpu = Cpu::from_str("cpy 1 a\ntgl a\ndec a").unwrap();
        let cpu = run(cpu);
        assert_eq!(
            cpu.program,
            [Ins::Cpy(1.into(), 'a'.into()), Ins::Tgl('a'), Ins::Inc('a')]
        );
    }

    #[test]
    fn test_tgl_inc_dec() {
        let cpu = Cpu::from_str("cpy 1 a\ntgl a\ninc a").unwrap();
        let cpu = run(cpu);
        assert_eq!(
            cpu.program,
            [Ins::Cpy(1.into(), 'a'.into()), Ins::Tgl('a'), Ins::Dec('a')]
        );
    }

    #[test]
    fn test_tgl_tgl_inc() {
        let cpu = Cpu::from_str("cpy 1 a\ntgl a\ntgl a").unwrap();
        let cpu = run(cpu);
        assert_eq!(
            cpu.program,
            [Ins::Cpy(1.into(), 'a'.into()), Ins::Tgl('a'), Ins::Inc('a')]
        );
    }

    #[test]
    fn test_tgl_cpy_jnz() {
        let cpu = Cpu::from_str("cpy 1 a\ntgl a\ncpy 1 a").unwrap();
        let cpu = run(cpu);
        assert_eq!(
            cpu.program,
            [
                Ins::Cpy(1.into(), 'a'.into()),
                Ins::Tgl('a'),
                Ins::Jnz(1.into(), 'a'.into()),
            ]
        );
    }

    #[test]
    fn test_tgl_jnz_cpy() {
        let cpu = Cpu::from_str("cpy 1 a\ntgl a\njnz 1 a").unwrap();
        let cpu = run(cpu);
        assert_eq!(
            cpu.program,
            [
                Ins::Cpy(1.into(), 'a'.into()),
                Ins::Tgl('a'),
                Ins::Cpy(1.into(), 'a'.into()),
            ]
        );
    }
}
