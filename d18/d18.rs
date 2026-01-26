use bitvec::prelude::*;
use itertools::Itertools;
use std::fmt;
use std::fmt::{Formatter, Write};
use std::iter::once;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Row {
    data: BitVec,
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Row {
            data: s
                .chars()
                .map(|c| match c {
                    '^' => Ok(true),
                    '.' => Ok(false),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.data
            .iter()
            .map(|c| if *c { '^' } else { '.' })
            .try_for_each(|c| f.write_char(c))
    }
}

fn next_row(prev: Row) -> Row {
    Row {
        data: once(false)
            .chain(prev.data)
            .chain(once(false))
            .tuple_windows()
            .map(|(left, center, right)| {
                matches!(
                    (left, center, right),
                    (true, true, false)
                        | (false, true, true)
                        | (true, false, false)
                        | (false, false, true)
                )
            })
            .collect(),
    }
}

struct RowIter {
    prev: Option<Row>,
    first: Option<Row>,
}

impl RowIter {
    fn new(row: Row) -> Self {
        Self {
            prev: None,
            first: Some(row),
        }
    }
}

impl Iterator for RowIter {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if let first @ Some(_) = self.first.take() {
            self.prev = first.clone();
            return first;
        }
        self.prev = Some(next_row(self.prev.take()?));
        self.prev.clone()
    }
}

impl Row {
    fn into_rows(self) -> impl Iterator<Item = Row> {
        RowIter::new(self)
    }
}

fn main() {
    let input = ".^^^^^.^^^..^^^^^...^.^..^^^.^^....^.^...^^^...^^^^..^...^...^^.^.^.......^..^^...^.^.^^..^^^^^...^.";
    let first_row: Row = input.parse().unwrap();
    let clear_count: usize = first_row
        .clone()
        .into_rows()
        .take(40)
        .inspect(|r| println!("{}", r))
        .map(|r| r.data.count_zeros())
        .sum();
    println!("Safe tiles(40): {}", clear_count);

    let clear_count: usize = first_row
        .into_rows()
        .take(400000)
        .map(|r| r.data.count_zeros())
        .sum();
    println!("Safe tiles(400000): {}", clear_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_display() {
        let s = ".^^.^.^^^^";
        let parsed = s.parse();
        assert_eq!(
            parsed,
            Ok(Row {
                data: bitvec![0, 1, 1, 0, 1, 0, 1, 1, 1, 1]
            })
        );
        assert_eq!(parsed.unwrap().to_string(), s);
    }

    #[test]
    fn test_next_row() {
        let r: Row = "..^^.".parse().unwrap();
        assert!(
            r.into_rows()
                .take(3)
                .map(|r| r.to_string())
                .eq("..^^.\n.^^^^\n^^..^".lines())
        );
    }
}
