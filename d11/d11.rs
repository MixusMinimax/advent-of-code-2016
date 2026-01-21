#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Floor<'i> {
    nr: u8,
    items: Vec<&'i str>,
}

mod parse {
    use super::Floor;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{space0, space1};
    use nom::combinator::{eof, map, not, value};
    use nom::multi::separated_list1;
    use nom::{IResult, Parser};
    use std::cmp;

    fn ordinal(s: &str) -> IResult<&str, u8> {
        alt((
            value(1, tag("first")),
            value(2, tag("second")),
            value(3, tag("third")),
            value(4, tag("fourth")),
            value(5, tag("fifth")),
            value(6, tag("sixth")),
            value(7, tag("seventh")),
            value(8, tag("eighth")),
            value(9, tag("ninth")),
            value(10, tag("tenth")),
            value(11, tag("eleventh")),
            value(12, tag("twelfth")),
        ))
        .parse(s)
    }

    fn item(s: &str) -> IResult<&str, &str> {
        let (s, _): (&str, _) = (tag("a"), space1).parse(s)?;
        let end = cmp::min(
            s.find(" and").unwrap_or(s.len()),
            cmp::min(
                s.find(",").unwrap_or(s.len()),
                s.find(".").unwrap_or(s.len()),
            ),
        );
        Ok((
            &s[end..],
            s[0..end].trim().trim_start_matches("a").trim_start(),
        ))
    }

    fn item_list(s: &str) -> IResult<&str, Vec<&str>> {
        alt((
            map((tag("nothing"), space1, tag("relevant"), space0), |(..)| {
                Vec::<&str>::new()
            }),
            map(
                (item, space1, tag("and"), space1, item, space0),
                |(a, _, _, _, b, _): (&str, _, _, _, &str, _)| vec![a, b],
            ),
            map(
                (
                    separated_list1((space0, tag(","), space0, not(tag("and"))), item),
                    space0,
                    tag(","),
                    space0,
                    tag("and"),
                    space1,
                    item,
                    space0,
                ),
                |(mut xs, _, _, _, _, _, x, _): (Vec<&str>, _, _, _, _, _, &str, _)| {
                    xs.push(x);
                    xs
                },
            ),
            map((item, space0), |(a, _)| vec![a]),
        ))
        .parse(s)
    }

    pub fn floor(s: &'_ str) -> IResult<&'_ str, Floor<'_>> {
        map(
            (
                space0,
                tag("The"),
                space1,
                ordinal,
                space1,
                tag("floor"),
                space1,
                tag("contains"),
                space1,
                item_list,
                tag("."),
                space0,
                eof,
            ),
            |(_, _, _, nr, _, _, _, _, _, items, ..)| Floor { nr, items },
        )
        .parse(s)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_item() {
            assert_eq!(item("a toilet"), Ok(("", "toilet")));
            assert_eq!(item("a toilet,"), Ok((",", "toilet")));
            assert_eq!(item("a toilet and"), Ok((" and", "toilet")));
            assert_eq!(item("a toilet "), Ok(("", "toilet")));
            assert_eq!(item("a toilet seat  ,  asd"), Ok((",  asd", "toilet seat")));
            assert_eq!(
                item("a  toilet seat  and  something else"),
                Ok((" and  something else", "toilet seat"))
            );
        }

        #[test]
        fn test_item_list() {
            assert_eq!(item_list("a dog"), Ok(("", vec!["dog"])));
            assert_eq!(item_list("a dog and a cat"), Ok(("", vec!["dog", "cat"])));
            assert_eq!(item_list("a dog, and a cat"), Ok(("", vec!["dog", "cat"])));
            assert_eq!(
                item_list("a dog, a cat, and a mouse"),
                Ok(("", vec!["dog", "cat", "mouse"]))
            );
        }

        #[test]
        fn test_floor() {
            assert_eq!(
                floor("The fourth floor contains nothing relevant."),
                Ok((
                    "",
                    Floor {
                        nr: 4,
                        items: vec![],
                    },
                )),
            );
            assert_eq!(
                floor("The third floor contains a lithium generator."),
                Ok((
                    "",
                    Floor {
                        nr: 3,
                        items: vec!["lithium generator"],
                    },
                )),
            );
            assert_eq!(
                floor(
                    "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.",
                ),
                Ok((
                    "",
                    Floor {
                        nr: 1,
                        items: vec![
                            "hydrogen-compatible microchip",
                            "lithium-compatible microchip",
                        ],
                    },
                )),
            );
            assert_eq!(
                floor(
                    "The first floor contains a strontium generator, a strontium-compatible microchip, a plutonium generator, and a plutonium-compatible microchip.",
                ),
                Ok((
                    "",
                    Floor {
                        nr: 1,
                        items: vec![
                            "strontium generator",
                            "strontium-compatible microchip",
                            "plutonium generator",
                            "plutonium-compatible microchip",
                        ],
                    },
                )),
            );
        }
    }
}

fn main() {
    let input = include_str!("d11.txt");
    let floors: Vec<_> = input.lines().map(|l| parse::floor(l).unwrap().1).collect();
    println!("{:?}", floors);
}
