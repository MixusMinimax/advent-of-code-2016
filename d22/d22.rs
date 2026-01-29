use aoc2016::graph::a_star_rev;
use aoc2016::vec2_hamming_dist;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use std::cmp::max;
use std::ops::{Index, IndexMut};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Drive<'name> {
    name: Cow<'name, str>,
    pos: [usize; 2],
    size: u32,
    used: u32,
    avail: u32,
    use_percent: u8,
}

fn parse_drive(s: &str) -> Result<Drive<'_>, ()> {
    lazy_static! {
        // language=regexp
        static ref PAT: Regex = Regex::new(
            r#"(/dev/grid/node-x(\d+)-y(\d+))\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%"#
        ).unwrap();
    }

    let [name, x, y, size, used, avail, use_percent] = PAT.captures(s).ok_or(())?.extract().1;

    Ok(Drive {
        name: Cow::Borrowed(name),
        pos: [x.parse().map_err(|_| ())?, y.parse().map_err(|_| ())?],
        size: size.parse().map_err(|_| ())?,
        used: used.parse().map_err(|_| ())?,
        avail: avail.parse().map_err(|_| ())?,
        use_percent: use_percent.parse().map_err(|_| ())?,
    })
}

impl Drive<'_> {
    fn fits_into(&self, other: &Drive<'_>) -> bool {
        self.used <= other.avail
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Grid<'i> {
    drives: Vec<Option<Drive<'i>>>,
    width: usize,
    height: usize,
    data_pos: [usize; 2],
}

impl<'i> Grid<'i> {
    fn get(&self, [x, y]: [usize; 2]) -> &Option<Drive<'i>> {
        &self.drives[y * self.width + x]
    }
}

impl<'i> Index<[usize; 2]> for Grid<'i> {
    type Output = Drive<'i>;

    fn index(&self, [x, y]: [usize; 2]) -> &Self::Output {
        self.drives[y * self.width + x].as_ref().unwrap()
    }
}

impl<'i> IndexMut<[usize; 2]> for Grid<'i> {
    fn index_mut(&mut self, [x, y]: [usize; 2]) -> &mut Self::Output {
        self.drives[y * self.width + x].as_mut().unwrap()
    }
}

impl<'i> Grid<'i> {
    fn construct(drives: impl IntoIterator<Item = Drive<'i>>) -> Self {
        let mut v = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for drive in drives {
            let [x, y] = drive.pos;
            if v.len() <= y {
                v.resize_with(y + 1, Vec::new);
                height = max(height, y + 1);
            }
            let row = &mut v[y];
            if row.len() <= x {
                row.resize(x + 1, None);
                width = max(width, x + 1);
            }
            row[x] = Some(drive);
        }
        Grid {
            drives: v
                .into_iter()
                .flat_map(|mut row| {
                    if row.len() == width {
                        row
                    } else {
                        row.resize(width, None);
                        row
                    }
                })
                .collect(),
            width,
            height,
            data_pos: [0, 0],
        }
    }
}

impl<'i> Grid<'i> {
    /// returns pairs (a, b) for which a can be moved into b
    fn possible_moves(&self) -> Vec<([usize; 2], [usize; 2])> {
        let width = self.width;
        let height = self.height;
        let mut result = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let pos = [x, y];
                let Some(cur) = self.get(pos) else {
                    continue;
                };
                if cur.used == 0 {
                    continue;
                }
                let x = x as isize;
                let y = y as isize;
                for neighbor in [[x + 1, y], [x, y + 1], [x - 1, y], [x, y - 1]]
                    .into_iter()
                    .filter(|&[x, y]| {
                        (0isize..width as isize).contains(&x)
                            && (0isize..height as isize).contains(&y)
                    })
                    .map(|[x, y]| [x as usize, y as usize])
                {
                    if let Some(other) = self.get(neighbor)
                        && cur.fits_into(other)
                    {
                        result.push((pos, neighbor));
                    }
                }
            }
        }
        result
    }

    fn execute_move(mut self, (from, to): ([usize; 2], [usize; 2])) -> Self {
        self[to].used += self[from].used;
        let d = &mut self[to];
        d.use_percent = (d.used * 100 / d.size) as u8;
        d.avail = d.size - d.used;
        let d = &mut self[from];
        d.used = 0;
        d.use_percent = 0;
        d.avail = d.size;
        if from == self.data_pos {
            self.data_pos = to;
        }
        self
    }
}

fn find_shortest_path(grid: Grid, goal_pos: [usize; 2]) -> usize {
    a_star_rev(
        &grid,
        |g| {
            println!("{:?}, {}", g.data_pos, g.possible_moves().len());
            g.data_pos == goal_pos
        },
        |g| {
            g.possible_moves()
                .into_iter()
                .map(|m| (g.clone().execute_move(m), m))
                .collect::<Vec<_>>()
        },
        |g| {
            let dst = vec2_hamming_dist(g.data_pos, goal_pos);

            let avg_hole_dist = g
                .possible_moves()
                .into_iter()
                .map(|(_, h)| vec2_hamming_dist(h, g.data_pos))
                .fold((0, 0), |i, g| (i.0 + g, i.1 + 1));
            let avg_hole_dist = avg_hole_dist.0 / avg_hole_dist.1;

            dst as i64 + avg_hole_dist as i64
        },
        |_, _, _| 1,
    )
    .unwrap()
    .0
    .len()
}

fn main() {
    // let input = include_str!("d22.sample.txt");
    let input = include_str!("d22.txt");
    let drives: Vec<_> = input
        .lines()
        .skip_while(|l| !l.starts_with('/'))
        .flat_map(parse_drive)
        .collect();
    let viable_count = (0..drives.len() - 1)
        .flat_map(|a| {
            (a + 1..drives.len())
                .filter(move |&b| a != b)
                .flat_map(move |b| [(a, b), (b, a)])
        })
        .filter(|&(a, b)| drives[a].used != 0 && drives[a].fits_into(&drives[b]))
        .count();
    println!("Part1: {}", viable_count);

    let mut grid = Grid::construct(drives);
    grid.data_pos = [grid.width - 1, 0];

    let l = find_shortest_path(grid, [0, 0]);
    println!("Part2: {}", l);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_drive("/dev/grid/node-x2-y4     94T   68T    26T   72%"),
            Ok(Drive {
                name: "/dev/grid/node-x2-y4".into(),
                pos: [2, 4],
                size: 94,
                used: 68,
                avail: 26,
                use_percent: 72,
            })
        );
    }

    fn demo_grid() -> Grid<'static> {
        Grid::construct(
            r#"
                Filesystem            Size  Used  Avail  Use%
                /dev/grid/node-x0-y0   10T    8T     2T   80%
                /dev/grid/node-x0-y1   11T    6T     5T   54%
                /dev/grid/node-x0-y2   32T   28T     4T   87%
                /dev/grid/node-x1-y0    9T    7T     2T   77%
                /dev/grid/node-x1-y1    8T    0T     8T    0%
                /dev/grid/node-x1-y2   11T    7T     4T   63%
                /dev/grid/node-x2-y0   10T    6T     4T   60%
                /dev/grid/node-x2-y1    9T    8T     1T   88%
                /dev/grid/node-x2-y2    9T    6T     3T   66%
            "#
            .lines()
            .map(str::trim)
            .skip_while(|l| !l.starts_with('/'))
            .flat_map(parse_drive),
        )
    }

    #[test]
    fn test_demo_grid() {
        let grid = demo_grid();
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(grid[[1, 2]].use_percent, 63);
    }

    #[test]
    fn test_possible_moves() {
        let grid = demo_grid();
        let moves = grid.possible_moves();
        assert_eq!(
            moves,
            [
                ([1, 0], [1, 1]),
                ([0, 1], [1, 1]),
                ([2, 1], [1, 1]),
                ([1, 2], [1, 1]),
            ]
        );
    }

    #[test]
    fn test_execute() {
        let grid = demo_grid().execute_move(([1, 0], [1, 1]));
        assert_eq!(
            grid[[1, 0]],
            Drive {
                name: Cow::Borrowed("/dev/grid/node-x1-y0"),
                pos: [1, 0],
                size: 9,
                used: 0,
                avail: 9,
                use_percent: 0,
            }
        );
        assert_eq!(
            grid[[1, 1]],
            Drive {
                name: Cow::Borrowed("/dev/grid/node-x1-y1"),
                pos: [1, 1],
                size: 8,
                used: 7,
                avail: 1,
                use_percent: (100 * 7 / 8) as u8,
            }
        );
    }

    #[test]
    fn test_execute_data_pos() {
        let mut grid = demo_grid();
        grid.data_pos = [2, 1];
        let grid = grid.execute_move(([2, 1], [1, 1]));
        assert_eq!(grid.data_pos, [1, 1]);
    }
}
