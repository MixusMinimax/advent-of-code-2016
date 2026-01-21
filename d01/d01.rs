use std::collections::HashSet;
use std::iter::{once, repeat_n};
use std::str::FromStr;
use vecmath::{vec2_add, vec2_scale};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Turn {
    Straight,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Instruction {
    turn: Turn,
    dist: u32,
}

impl FromStr for Turn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Turn::Left),
            "R" => Ok(Turn::Right),
            _ => Err(()),
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(());
        }
        Ok(Instruction {
            turn: s[..1].parse()?,
            dist: s[1..].parse().map_err(|_| ())?,
        })
    }
}

fn parse_instructions(s: &str) -> Result<Vec<Instruction>, ()> {
    s.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|i| i.parse())
        .collect::<Result<_, _>>()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    dir: Direction,
    pos: [i32; 2],
}

impl Default for State {
    fn default() -> Self {
        Self {
            dir: Direction::North,
            pos: [0, 0],
        }
    }
}

fn turn(direction: Direction, turn: Turn) -> Direction {
    match (direction, turn) {
        (dir, Turn::Straight) => dir,
        (Direction::North, Turn::Right) => Direction::East,
        (Direction::East, Turn::Right) => Direction::South,
        (Direction::South, Turn::Right) => Direction::West,
        (Direction::West, Turn::Right) => Direction::North,
        (Direction::North, Turn::Left) => Direction::West,
        (Direction::East, Turn::Left) => Direction::North,
        (Direction::South, Turn::Left) => Direction::East,
        (Direction::West, Turn::Left) => Direction::South,
    }
}

impl Direction {
    fn vec(&self) -> [i32; 2] {
        match self {
            Direction::North => [0, 1],
            Direction::East => [1, 0],
            Direction::South => [0, -1],
            Direction::West => [-1, 0],
        }
    }
}

fn execute(mut state: State, ins: Instruction) -> State {
    state.dir = turn(state.dir, ins.turn);
    state.pos = vec2_add(state.pos, vec2_scale(state.dir.vec(), ins.dist as i32));
    state
}

fn dist_from_0(pos: [i32; 2]) -> i32 {
    pos[0].abs() + pos[1].abs()
}

fn main() {
    let input = include_str!("d01.txt");
    // let input = "R8, R4, R4, R8";
    let instructions = parse_instructions(input).unwrap();
    let final_state = instructions.iter().copied().fold(State::default(), execute);
    let dist = dist_from_0(final_state.pos);
    println!("Part1: {}", dist);

    let final_state = instructions
        .into_iter()
        // We have to do this because when paths cross, it also counts. Not just when we _land_
        // on an already visited place. So I just split the instruction into single-length steps.
        .flat_map(|ins| {
            once(Instruction { dist: 1, ..ins }).chain(repeat_n(
                Instruction {
                    turn: Turn::Straight,
                    dist: 1,
                },
                if ins.dist > 1 {
                    (ins.dist as i32 - 1) as usize
                } else {
                    0
                },
            ))
        })
        .scan(
            (HashSet::from([[0, 0]]), State::default()),
            |(visited, state), ins| {
                *state = execute(*state, ins);
                if visited.contains(&state.pos) {
                    return Some(Some(*state));
                }
                visited.insert(state.pos);
                Some(None)
            },
        )
        .find(Option::is_some)
        .unwrap()
        .unwrap();

    let dist = dist_from_0(final_state.pos);
    println!("Part2: {}", dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            "L123".parse(),
            Ok(Instruction {
                turn: Turn::Left,
                dist: 123,
            })
        );
        assert_eq!(
            "R69".parse(),
            Ok(Instruction {
                turn: Turn::Right,
                dist: 69,
            })
        );
        assert_eq!("X123".parse::<Instruction>(), Err(()));
        assert_eq!("L".parse::<Instruction>(), Err(()));
        assert_eq!("".parse::<Instruction>(), Err(()));
        assert_eq!("L123L".parse::<Instruction>(), Err(()));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse_instructions(""), Ok(vec![]));
        assert_eq!(
            parse_instructions("L1"),
            Ok(vec![Instruction {
                turn: Turn::Left,
                dist: 1
            }])
        );
        assert_eq!(
            parse_instructions("L1,R2"),
            Ok(vec![
                Instruction {
                    turn: Turn::Left,
                    dist: 1
                },
                Instruction {
                    turn: Turn::Right,
                    dist: 2
                }
            ])
        );
        assert_eq!(
            parse_instructions("   L1  ,  R2  "),
            Ok(vec![
                Instruction {
                    turn: Turn::Left,
                    dist: 1
                },
                Instruction {
                    turn: Turn::Right,
                    dist: 2
                }
            ])
        );
    }

    #[test]
    fn test_example1() {
        let instructions = parse_instructions("R2, L3").unwrap();
        let result = instructions.into_iter().fold(State::default(), execute);
        assert_eq!(result.pos, [2, 3]);
        assert_eq!(dist_from_0(result.pos), 5);
    }

    #[test]
    fn test_example2() {
        let instructions = parse_instructions("R2, R2, R2").unwrap();
        let result = instructions.into_iter().fold(State::default(), execute);
        assert_eq!(result.pos, [0, -2]);
        assert_eq!(dist_from_0(result.pos), 2);
    }

    #[test]
    fn test_example3() {
        let instructions = parse_instructions("R5, L5, R5, R3").unwrap();
        let result = instructions.into_iter().fold(State::default(), execute);
        assert_eq!(result.pos, [10, 2]);
        assert_eq!(dist_from_0(result.pos), 12);
    }
}
