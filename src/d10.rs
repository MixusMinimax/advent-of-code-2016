use aoc2016::IndexMap;
use std::cmp;
use std::collections::VecDeque;

type Id = u32;
type Value = u32;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum OutputReference {
    Bot(Id),
    Output(Id),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Output {
    Unknown(OutputReference),
    Value(OutputReference, Value),
}

impl Output {
    fn set_value(&mut self, v: Value) -> (OutputReference, Value) {
        let (Output::Unknown(r) | Output::Value(r, _)) = *self;
        *self = Output::Value(r, v);
        (r, v)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Input {
    output: Output,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Bot {
    id: Id,
    low_output: Output,
    high_output: Output,
    tmp_value: Option<Value>,
}

mod parse {
    use super::{Bot, Id, Input, Output, OutputReference};
    use aoc2016::IndexMap;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::u32;
    use nom::combinator::map;
    use nom::{IResult, Parser};

    type NomError<S> = nom::Err<nom::error::Error<S>>;

    enum ParseLineResult {
        Input(Input),
        Bot(Bot),
    }

    fn parse_line(line: &str) -> IResult<&str, ParseLineResult> {
        fn output_reference(s: &str) -> IResult<&str, OutputReference> {
            alt((
                map((tag("bot "), u32), |(_, id)| OutputReference::Bot(id)),
                map((tag("output "), u32), |(_, id)| OutputReference::Output(id)),
            ))
            .parse(s)
        }

        alt((
            // value 2 goes to bot 156
            map(
                (tag("value "), u32, tag(" goes to "), output_reference),
                |(_, value, _, output)| {
                    ParseLineResult::Input(Input {
                        output: Output::Value(output, value),
                    })
                },
            ),
            // bot 84 gives low to bot 174 and high to bot 155
            map(
                (
                    tag("bot "),
                    u32,
                    tag(" gives low to "),
                    output_reference,
                    tag(" and high to "),
                    output_reference,
                ),
                |(_, id, _, low, _, high)| {
                    ParseLineResult::Bot(Bot {
                        id,
                        low_output: Output::Unknown(low),
                        high_output: Output::Unknown(high),
                        tmp_value: None,
                    })
                },
            ),
        ))
        .parse(line)
    }

    #[allow(clippy::type_complexity)]
    pub fn from_lines<'s>(
        lines: impl IntoIterator<Item = &'s str>,
    ) -> Result<(Vec<Input>, IndexMap<Id, Bot>), NomError<&'s str>> {
        let mut inputs = Vec::new();
        let mut bots = IndexMap::new();

        for line in lines {
            match parse_line(line)?.1 {
                ParseLineResult::Input(input) => inputs.push(input),
                ParseLineResult::Bot(bot) => bots.insert(bot.id, bot),
            }
        }

        Ok((inputs, bots))
    }
}

fn main() {
    // let input = include_str!("d10.sample.txt");
    let input = include_str!("d10.txt");
    let (inputs, mut bots) = parse::from_lines(input.lines()).unwrap();
    let mut processing: VecDeque<_> = inputs
        .iter()
        .map(|i| match i.output {
            Output::Value(r, v) => (r, v),
            _ => unreachable!(),
        })
        .collect();
    let mut outputs = IndexMap::new();
    while let Some((r, v)) = processing.pop_front() {
        match r {
            OutputReference::Bot(id) => {
                let b = bots
                    .get_mut(id)
                    .unwrap_or_else(|| panic!("Bot {id} not found"));
                if let Some(tmp) = b.tmp_value {
                    b.tmp_value = None;
                    assert_ne!(v, tmp);
                    let low = cmp::min(v, tmp);
                    let high = cmp::max(v, tmp);
                    processing.push_back(b.low_output.set_value(low));
                    processing.push_back(b.high_output.set_value(high));
                } else {
                    b.tmp_value = Some(v);
                }
            }
            OutputReference::Output(id) => outputs.insert(id, v),
        }
    }

    // println!("{bots:?}");
    // println!("{outputs:?}");

    let important_bot = bots.values().find(|bot| {
        matches!(
            bot,
            Bot {
                low_output: Output::Value(_, 17),
                high_output: Output::Value(_, 61),
                ..
            }
        )
    });

    println!("Important bot: {important_bot:?}");

    println!("Part2: {}", outputs[0] * outputs[1] * outputs[2]);
}
