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

fn get_code<'s>(
    lines: impl IntoIterator<Item = &'s str>,
    start: [i32; 2],
    move_numpad: impl Fn([i32; 2], Direction) -> [i32; 2] + Copy,
    number_at: impl Fn([i32; 2]) -> char,
) -> impl Iterator<Item = char> {
    lines.into_iter().scan(start, move |fingy, l| {
        *fingy = parse_dirs(l).fold(*fingy, move_numpad);
        Some(number_at(*fingy))
    })
}

mod part1 {
    use super::*;

    fn move_numpad(pos: [i32; 2], dir: Direction) -> [i32; 2] {
        let [x, y] = vec2_add(pos, dir.vec());
        [x.clamp(0, 2), y.clamp(0, 2)]
    }

    fn number_at([x, y]: [i32; 2]) -> char {
        ((y * 3 + x + 1) as u8 + b'0') as char
    }

    pub fn get_code<'s>(
        lines: impl IntoIterator<Item = &'s str> + 's,
    ) -> impl Iterator<Item = char> {
        super::get_code(lines, [1, 1], move_numpad, number_at)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_code() {
            let instructions = ["ULL", "RRDDD", "LURDL", "UUUUD"];
            let code: Vec<_> = get_code(instructions).collect();
            assert_eq!(code, ['1', '9', '8', '5']);
            assert_eq!(code.into_iter().collect::<String>(), "1985");
        }
    }
}

mod part2 {
    use super::*;

    fn is_inside([x, y]: [i32; 2]) -> bool {
        match y {
            0 | 4 => x == 2,
            1 | 3 => (1..=3).contains(&x),
            2 => (0..=4).contains(&x),
            _ => false,
        }
    }

    fn move_numpad(pos: [i32; 2], dir: Direction) -> [i32; 2] {
        let new_pos = vec2_add(pos, dir.vec());
        if is_inside(new_pos) { new_pos } else { pos }
    }

    fn number_at([x, y]: [i32; 2]) -> char {
        [
            ['_', '_', '1', '_', '_'],
            ['_', '2', '3', '4', '_'],
            ['5', '6', '7', '8', '9'],
            ['_', 'A', 'B', 'C', '_'],
            ['_', '_', 'D', '_', '_'],
        ][y as usize][x as usize]
    }

    pub fn get_code<'s>(
        lines: impl IntoIterator<Item = &'s str> + 's,
    ) -> impl Iterator<Item = char> {
        super::get_code(lines, [0, 2], move_numpad, number_at)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_code() {
            let instructions = ["ULL", "RRDDD", "LURDL", "UUUUD"];
            let code: Vec<_> = get_code(instructions).collect();
            assert_eq!(code.into_iter().collect::<String>(), "5DB3");
        }
    }
}

fn main() {
    let input = include_str!("d02.txt");
    let code: String = part1::get_code(input.lines()).collect();
    println!("Part1: {}", code);

    let code: String = part2::get_code(input.lines()).collect();
    println!("Part2: {}", code);
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
}
