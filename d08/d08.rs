use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::fmt::{Formatter, Write};
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction {
    Rect(u32, u32),
    RotateRow(u32, u32),
    RotateCol(u32, u32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            // language=regexp
            static ref PAT: Regex = Regex::new(
                r#"(?ix)
                    rect\s+(?<w>\d+)x(?<h>\d+)| # rect 2x1
                    rotate\s+(?:
                        row\s+y=(?<y>\d+)|      # rotate row y=0 by 5
                        column\s+x=(?<x>\d+)    # rotate column x=0 by 1
                    )\s+by\s+(?<by>\d+)
                "#
            ).unwrap();
        }

        || -> Option<_> {
            let cap = PAT.captures(s.trim())?;
            match (
                (cap.name("w"), cap.name("h")),
                cap.name("y"),
                cap.name("x"),
                cap.name("by"),
            ) {
                ((Some(w), Some(h)), ..) => Some(Self::Rect(
                    w.as_str().parse().ok()?,
                    h.as_str().parse().ok()?,
                )),
                (_, Some(y), _, Some(by)) => Some(Self::RotateRow(
                    y.as_str().parse().ok()?,
                    by.as_str().parse().ok()?,
                )),
                (_, _, Some(x), Some(by)) => Some(Self::RotateCol(
                    x.as_str().parse().ok()?,
                    by.as_str().parse().ok()?,
                )),
                _ => None,
            }
        }()
        .ok_or(())
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Screen {
    width: u32,
    height: u32,
    values: Vec<bool>,
}

impl Screen {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            values: vec![false; (width * height) as usize],
        }
    }

    fn idx(&self, x: u32, y: u32) -> usize {
        y as usize * self.width as usize + x as usize
    }

    fn rect(mut self, width: u32, height: u32) -> Self {
        for row in 0..height {
            for col in 0..width {
                let idx = self.idx(col, row);
                self.values[idx] = true;
            }
        }
        self
    }

    fn rot_row(mut self, y: u32, by: u32) -> Self {
        let by = match by {
            0 => return self,
            1.. => by % self.width,
        };
        let span = self.idx(0, y)..self.idx(0, y + 1);
        self.values[span].rotate_right(by as usize);
        self
    }

    fn rot_col(mut self, x: u32, by: u32) -> Self {
        let by = match by {
            0 => return self,
            1.. => by % self.height,
        };
        let mut col: Vec<_> = (0..self.height)
            .map(|y| self.idx(x, y))
            .map(|i| self.values[i])
            .collect();
        col.rotate_right(by as usize);
        for (y, v) in col.into_iter().enumerate() {
            let i = self.idx(x, y as u32);
            self.values[i] = v;
        }
        self
    }

    fn execute(self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Rect(w, h) => self.rect(w, h),
            Instruction::RotateRow(y, by) => self.rot_row(y, by),
            Instruction::RotateCol(x, by) => self.rot_col(x, by),
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut start_nl = false;
        for l in self.values.chunks(self.width as usize) {
            if start_nl {
                writeln!(f)?;
            } else {
                start_nl = true;
            }
            l.iter()
                .map(|&b| if b { '#' } else { '.' })
                .try_for_each(|c| f.write_char(c))?;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("d08.txt");
    let result = input
        .lines()
        .map(|s| s.parse().unwrap())
        .fold(Screen::new(50, 6), Screen::execute);
    let lit_count = result.values.iter().cloned().filter(|e| *e).count();
    println!("Lit count: {}", lit_count);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        use Instruction::*;
        assert_eq!("rect 4x5".parse(), Ok(Rect(4, 5)));
        assert_eq!("rotate row y=2 by 5".parse(), Ok(RotateRow(2, 5)));
        assert_eq!("rotate column x=17 by 1".parse(), Ok(RotateCol(17, 1)));
    }

    fn screen(s: &str) -> Screen {
        let mut width = 0u32;
        let values: Vec<_> = s
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .inspect(|l| width = l.len() as u32)
            .flat_map(str::chars)
            .map(|c| c == '#')
            .collect();
        Screen {
            width,
            height: values.len() as u32 / width,
            values,
        }
    }

    #[test]
    fn test_parse_screen() {
        assert_eq!(
            screen("##..\n##..\n....\n"),
            Screen {
                width: 4,
                height: 3,
                values: vec![
                    true, true, false, false, true, true, false, false, false, false, false, false
                ],
            }
        );
    }

    #[test]
    fn test_rect() {
        assert_eq!(
            Screen::new(4, 4).rect(2, 2),
            screen("##..\n##..\n....\n....\n")
        );
    }

    #[test]
    fn test_rot_row() {
        assert_eq!(
            screen("##...\n.#..#\n.....\n").rot_row(1, 3),
            screen("##...\n..#.#\n.....\n")
        );
        assert_eq!(
            screen("##...\n.#..#\n.....\n").rot_row(1, 3 + 5),
            screen("##...\n..#.#\n.....\n")
        );
    }

    #[test]
    fn test_rot_col() {
        assert_eq!(
            screen(
                r"
                    #....
                    .#..#
                    .....
                    .....
                    .#...
                ",
            )
            .rot_col(1, 3 + 5),
            screen(
                r"
                    #....
                    ....#
                    .#...
                    .....
                    .#...
                ",
            ),
        );
    }

    #[test]
    fn execute_instructions() {
        assert_eq!(
            [
                "rect 3x2",
                "rotate column x=1 by 1",
                "rotate row y=0 by 4",
                "rotate column x=1 by 1",
            ]
            .into_iter()
            .map(str::parse)
            .map(Result::unwrap)
            .fold(Screen::new(7, 3), Screen::execute),
            screen(".#..#.#\n#.#....\n.#.....\n")
        );
    }
}
