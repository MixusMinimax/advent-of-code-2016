use crate::part2::parse_vertical;

fn triangle_is_valid([a, b, c]: [i32; 3]) -> bool {
    a + b > c && a + c > b && b + c > a
}

fn parse_horizontal(line: &str) -> Option<[i32; 3]> {
    let mut it = line.split_whitespace().map(str::parse).map(Result::ok);
    let res = Some([it.next()??, it.next()??, it.next()??]);
    if it.next().is_some() { None } else { res }
}

mod part2 {
    use crate::parse_horizontal;

    struct ParseVerticalTrianglesIterator<Lines> {
        lines: Lines,
        col2: Option<[i32; 3]>,
        col3: Option<[i32; 3]>,
    }

    impl<'s, Lines> Iterator for ParseVerticalTrianglesIterator<Lines>
    where
        Lines: Iterator<Item = &'s str>,
    {
        type Item = [i32; 3];

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(col2) = self.col2.take() {
                return Some(col2);
            }
            if let Some(col3) = self.col3.take() {
                return Some(col3);
            }

            let r1 = parse_horizontal(self.lines.next()?)?;
            let r2 = parse_horizontal(self.lines.next()?)?;
            let r3 = parse_horizontal(self.lines.next()?)?;

            self.col2 = Some([r1[1], r2[1], r3[1]]);
            self.col3 = Some([r1[2], r2[2], r3[2]]);

            Some([r1[0], r2[0], r3[0]])
        }
    }

    pub fn parse_vertical<'s>(
        i: impl IntoIterator<Item = &'s str>,
    ) -> impl Iterator<Item = [i32; 3]> {
        ParseVerticalTrianglesIterator {
            lines: i.into_iter(),
            col2: None,
            col3: None,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_vertical() {
            // I know technically it goes down in columns, but we just do three rows at a time.
            // The difference is the order of the parsed vectors, but since this assignment
            // only requires the count of valid triangles, we don't need to temporarily allocate
            // an entire vector of triangles. Only three at a time need to be parsed.
            // This means that the code would work over a .lines() iterator of a file that is
            // Terabytes large. Technically.
            let input = [
                "101 301 501",
                "102 302 502",
                "103 303 503",
                "201 401 601",
                "202 402 602",
                "203 403 603",
            ];
            assert_eq!(
                parse_vertical(input).collect::<Vec<_>>(),
                [
                    [101, 102, 103],
                    [301, 302, 303],
                    [501, 502, 503],
                    [201, 202, 203],
                    [401, 402, 403],
                    [601, 602, 603],
                ]
            );
        }
    }
}

fn main() {
    let input = include_str!("d03.txt");
    let valid_count = input
        .lines()
        .filter_map(parse_horizontal)
        .filter(|t| triangle_is_valid(*t))
        .count();
    println!("Part1: {}", valid_count);

    let valid_count = parse_vertical(input.lines())
        .filter(|t| triangle_is_valid(*t))
        .count();
    println!("Part2: {}", valid_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_horizontal("5 10 25"), Some([5, 10, 25]));
    }

    #[test]
    fn test_valid() {
        assert!(!triangle_is_valid([5, 10, 25]));
        assert!(triangle_is_valid([5, 7, 9]));
    }
}
