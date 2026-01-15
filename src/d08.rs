use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Instruction {
    Rect(u32, u32),
    RotateRow(u32, i32),
    RotateCol(u32, i32),
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

    fn idx(&self, x: u32, y: u32) -> u32 {
        y * self.width + x
    }

    fn rect(mut self, width: u32, height: u32) -> Self {
        for row in 0..height {
            for col in 0..width {
                let idx = self.idx(col, row);
                self.values[idx as usize] = true;
            }
        }
        self
    }
}

fn main() {}

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
}
