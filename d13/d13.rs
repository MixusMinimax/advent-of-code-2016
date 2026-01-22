use aoc2016::graph::a_star_rev;
use std::collections::{HashSet, VecDeque};
use vecmath::vec2_sub;

type Pos = [u32; 2];

fn is_wall([x, y]: Pos, favorite_num: u32) -> bool {
    (x * x + 3 * x + 2 * x * y + y + y * y + favorite_num).count_ones() & 1 == 1
}

fn neighbors([x, y]: Pos) -> impl Iterator<Item = Pos> {
    // to not underflow, we just add 1 to everything, do the filtering against 0, then sub 1.
    [[x, y + 1], [x + 2, y + 1], [x + 1, y], [x + 1, y + 2]]
        .into_iter()
        .filter(|&[x, y]| x != 0 && y != 0)
        .map(|[x, y]| [x - 1, y - 1])
}

fn open_neighbors(p: Pos, favorite_num: u32) -> impl Iterator<Item = Pos> {
    neighbors(p).filter(move |&p2| !is_wall(p2, favorite_num))
}

fn path_length(start: Pos, goal: Pos, favorite_num: u32) -> usize {
    a_star_rev(
        &start,
        |&n| n == goal,
        |&n| open_neighbors(n, favorite_num).map(|n| (n, ())),
        |&[x, y]| {
            let vec = vec2_sub([x as i64, y as i64], [goal[0] as i64, goal[1] as i64]);
            vec[0].abs() + vec[1].abs()
        },
        |_, _, _| 1,
    )
    .unwrap()
    .0
    .len()
}

fn possible_locations(start: Pos, max_depth: u32, favorite_num: u32) -> HashSet<Pos> {
    let mut visited = HashSet::new();

    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((current, depth)) = queue.pop_front() {
        visited.insert(current);
        if depth >= max_depth {
            continue;
        }
        queue.extend(
            open_neighbors(current, favorite_num)
                .filter(|p| !visited.contains(p))
                .map(|p| (p, depth + 1)),
        );
    }

    visited
}

fn main() {
    let start = [1, 1];
    let goal = [31, 39];
    let favorite_num = 1362;
    let len = path_length(start, goal, favorite_num);
    println!("Part1: {}", len);

    // aw shucks, part 2 requires dfs or bfs, they didn't assume I'd use A* for part 1
    // So I'm gonna basically have to redo the entire algorithm which could also have been used for
    // part 1...
    // well, it was simple enough

    let locs = possible_locations(start, 50, favorite_num);
    println!("Part2: {}", locs.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_neighbors() {
        let sut = |x: u32, y: u32| neighbors([x, y]).collect::<HashSet<_>>();
        assert_eq!(sut(0, 0), HashSet::from([[0, 1], [1, 0]]));
        assert_eq!(sut(1, 0), HashSet::from([[1, 1], [0, 0], [2, 0]]));
        assert_eq!(sut(5, 0), HashSet::from([[5, 1], [4, 0], [6, 0]]));
        assert_eq!(sut(0, 1), HashSet::from([[1, 1], [0, 0], [0, 2]]));
        assert_eq!(sut(0, 5), HashSet::from([[1, 5], [0, 4], [0, 6]]));
        assert_eq!(sut(1, 1), HashSet::from([[0, 1], [1, 0], [1, 2], [2, 1]]));
        assert_eq!(sut(6, 9), HashSet::from([[5, 9], [7, 9], [6, 8], [6, 10]]));
    }

    #[test]
    fn test_path_length() {
        let l = path_length([1, 1], [7, 4], 10);
        assert_eq!(l, 11);
    }
}
