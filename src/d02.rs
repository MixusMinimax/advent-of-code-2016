use std::fmt::Write;
use vecmath::vec2_add;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse_dirs(s: &str) -> impl Iterator<Item = Direction> {
    s.chars().map(|c| match c {
        'U' => Direction::Up,
        'R' => Direction::Right,
        'D' => Direction::Down,
        'L' => Direction::Left,
        _ => panic!("unexpected input"),
    })
}

impl Direction {
    fn vec(&self) -> [i32; 2] {
        match self {
            Direction::Up => [0, -1],
            Direction::Right => [1, 0],
            Direction::Down => [0, 1],
            Direction::Left => [-1, 0],
        }
    }
}

fn move_numpad(pos: [i32; 2], dir: Direction) -> [i32; 2] {
    let [x, y] = vec2_add(pos, dir.vec());
    [x.clamp(0, 2), y.clamp(0, 2)]
}

fn move_fingy(start: [i32; 2], directions: impl IntoIterator<Item = Direction>) -> [i32; 2] {
    directions.into_iter().fold(start, move_numpad)
}

fn number_at([x, y]: [i32; 2]) -> i32 {
    y * 3 + x + 1
}

fn get_code<'s>(lines: impl IntoIterator<Item = &'s str>) -> impl Iterator<Item = i32> {
    lines.into_iter().scan([1, 1], |fingy, l| {
        *fingy = move_fingy(*fingy, parse_dirs(l));
        Some(number_at(*fingy))
    })
}

fn to_string(code: impl IntoIterator<Item = i32>) -> String {
    code.into_iter().fold(String::new(), |mut s, i| {
        write!(&mut s, "{}", i).unwrap();
        s
    })
}

fn main() {
    let input = include_str!("d02.txt");
    let code = to_string(get_code(input.lines()));
    println!("Part1: {}", code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        use Direction::*;
        assert_eq!(parse_dirs("").collect::<Vec<_>>(), vec![]);
        assert_eq!(parse_dirs("L").collect::<Vec<_>>(), vec![Left]);
        assert_eq!(
            parse_dirs("LRUD").collect::<Vec<_>>(),
            vec![Left, Right, Up, Down]
        );
    }

    #[test]
    fn test_code() {
        let instructions = ["ULL", "RRDDD", "LURDL", "UUUUD"];
        let code: Vec<_> = get_code(instructions).collect();
        assert_eq!(code, [1, 9, 8, 5]);
        assert_eq!(to_string(code), "1985");
    }
}
